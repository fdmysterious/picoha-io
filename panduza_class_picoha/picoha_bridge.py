import time
import json
import base64
import serial
import threading
from loguru import logger
from .udev_tty import TTYPortfromUsbInfo
from statemachine import StateMachine, State

# Number of seconds before the state error try to re-init
ERROR_TIME_BEFROE_RETRY_S=3

# -----------------------------------------------------------------------------

class PicohaBridgeMachine(StateMachine):
    """State machine representing the internal state of the bridge
    """
    # States
    initialize = State('Initialize', initial=True)
    running = State('Running')
    error = State('Error')

    # Events
    init_fail = initialize.to(error)
    init_success = initialize.to(running)
    runtine_error = running.to(error)
    restart = error.to(initialize)

# -----------------------------------------------------------------------------

class PicohaBridge:
    """The bridge manage the threadsafe communication between multiple interfaces and one device
    
    Multiple driver_picoha_io will used onez instance of the bridge. One bridge for one device. 
    """

    ###########################################################################
    ###########################################################################
    
    def __init__(self, usb_uuid, vendor_id, product_id, serial=None):
        """Constructor
        """
        self.usbid_vendor = vendor_id
        self.usbid_product = product_id
        self.usbid_serial = serial
        self.start_time = time.time()
        self.fsm = PicohaBridgeMachine()
        self.mutex = threading.Lock()
        self.log = logger.bind(driver_name=f"bridge{usb_uuid}")
        
        # Just the be able to detect transition
        self.previous_state = None
        

    ###########################################################################
    ###########################################################################
    
    def state_initialize(self):
        """Initial state, configure the serial port
        """
        # Get serial port
        self.serial_port = TTYPortfromUsbInfo(self.usbid_vendor, self.usbid_product, self.usbid_serial, base_dev_tty="/dev/ttyACM")
        if self.serial_port is None:
            self.log.error(f"adapter not connected !")
            self.fsm.init_fail()
            self.start_time = time.time()
            return

        # Try to initialize the serial object
        try:
            self.serial_obj = serial.Serial(self.serial_port, timeout=2)
            
            req = { "cod": 10, "pin": 0, "arg": 0 }
            self.serial_obj.write( (json.dumps(req) + "\n") .encode() )
            ret = json.loads(self.serial_obj.readline())
            if "sts" in ret and ret["sts"] == 0:
                self.log.debug(f"Connected to pico probe SUCCESS !")
            else:
                raise Exception("ERROR on the probe connection !")

        except serial.serialutil.SerialException:
            self.log.error(f"serial cannot be initialized !")
            self.fsm.init_fail()
            self.start_time = time.time()
            return

        # Initialization ok !
        self.fsm.init_success()

    ###########################################################################
    ###########################################################################
    
    def state_running(self):
        """
        """
        pass
        # Communication test sequence
        # try:
            
        #     if self.parent_driver.gpio_dir == "in":
        #         req = { "cod": 2, "pin": self.parent_driver.gpio_id, "arg": 0 }
        #         self.serial_obj.write( (json.dumps(req) + "\n") .encode() )
        #         ret = self.serial_obj.readline()
                
        #         new_val = json.loads(ret)["arg"]
        #         if self.parent_driver.gpio_val != new_val:
        #             self.parent_driver.gpio_val = new_val
        #             self.parent_driver.push_io_value(new_val)
        #             self.log.debug(f"RX : {ret}")

        # except serial.serialutil.SerialException as e:
        #     self.log.error(f"adapter unreachable !")
        #     self.fsm.runtine_error()

    ###########################################################################
    ###########################################################################
    
    def state_error(self):
        """
        """
        global ERROR_TIME_BEFROE_RETRY_S
        if time.time() - self.start_time > ERROR_TIME_BEFROE_RETRY_S:
            self.fsm.restart()

    ###########################################################################
    ###########################################################################
    
    def loop(self):
        """Main loop of the process
        """
        # Lock
        self.mutex.acquire()
        cs = self.fsm.current_state

        # Log state transition
        if cs != self.previous_state:
            if self.previous_state is not None:
                self.log.debug(f"{self.previous_state.name} => {cs.name}")
            self.previous_state = cs

        # Execute the correct callback
        if   cs.name == 'Initialize':
            self.state_initialize()
        elif cs.name == 'Running':
            self.state_running()
        elif cs.name == 'Error':
            self.state_error()

        # Release
        self.mutex.release()
        return False

    ###########################################################################
    ###########################################################################

    def set_io_direction(self, gpio_id, direction):
        """Set the io direction

        Args:
            gpio_id (int): id of the targetted gpio
            direction (str): requested direction
        """
        self.log.debug(f"[I] set_io_direction")

        # Lock the mutex
        self.mutex.acquire()
        
        # The bridge must be in running state
        if self.fsm.current_state.name != 'Running':
            return False

        # Initialize variables
        success = None

        # Prepare message for the probe 
        dval = 0
        if direction == "in":
            dval = 1
        if direction == "out":
            dval = 2
        # COD:0 => set pin mode
        req = { "cod": 0, "pin": gpio_id, "arg": dval }
        self.log.debug(f"set_io_direction probe request => {req}")

        # Perform the communication
        try:
            self.serial_obj.write( (json.dumps(req) + "\n") .encode() )
            ret = self.serial_obj.readline()
            self.log.debug(f"ret => {ret}")
            success = True
        except serial.serialutil.SerialException as e:
            self.log.error(f"adapter unreachable !")
            self.fsm.runtine_error()
            success=False

        # Release
        self.mutex.release()
        self.log.debug(f"[O] set_io_direction")
        return success

    ###########################################################################
    ###########################################################################
    
    def set_io_value(self, gpio_id, value):
        """Set the io value threadsafe with the bridge

        Args:
            gpio_id (int): id of the targetted gpio
            value (int): requested value
        """
        self.log.debug(f"[I] set_io_value")

        # Lock the mutex
        self.mutex.acquire()
        
        # The bridge must be in running state
        if self.fsm.current_state.name != 'Running':
            return False

        success = True

        try:
            # COD:1 => write pin value
            req = { "cod": 1, "pin": gpio_id, "arg": value }
            self.log.debug(f"req => {req}")
            self.serial_obj.write( (json.dumps(req) + "\n") .encode() )
            ret = self.serial_obj.readline()
            self.log.debug(f"ret => {ret}")
        except serial.serialutil.SerialException as e:
            self.log.error(f"adapter unreachable !")
            self.fsm.runtine_error()
            success=False

        # Release
        self.mutex.release()
        self.log.debug(f"[O] set_io_value")
        return success

    ###########################################################################
    ###########################################################################
    
    def get_io_value(self, gpio_id):
        """
        """
        # self.log.debug(f"[I] get_io_value")

        # Lock the mutex
        self.mutex.acquire()
        
        # The bridge must be in running state
        if self.fsm.current_state.name != 'Running':
            return -1

        ret_val = -1


        try:
            # COD:2 => read pin value
            req = { "cod": 2, "pin": gpio_id, "arg": 0 }
            # self.log.debug(f"req => {req}")
            self.serial_obj.write( (json.dumps(req) + "\n") .encode() )
            ret = self.serial_obj.readline()
            # self.log.debug(f"ret => {ret}")
            new_val = json.loads(ret)["arg"]
            ret_val = new_val
        except serial.serialutil.SerialException as e:
            self.log.error(f"adapter unreachable !")
            self.fsm.runtine_error()

        self.mutex.release()
        # self.log.debug(f"[O] get_io_value")
        return ret_val

    
