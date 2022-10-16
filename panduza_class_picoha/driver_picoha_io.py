import time
import json
import base64
import serial
import threading
from loguru import logger
from panduza_platform import MetaDriverIo


from .picoha_bridge import PicohaBridge

class DriverPicohaIO(MetaDriverIo):
    """Driver IO for the Picoha
    """
    
    # Managed bridges
    # { portname => PicohaBridge }
    Bridges = dict()

    ###########################################################################
    ###########################################################################
    
    def config(self):
        """FROM MetaDriver
        """
        return {
            "compatible": "picoha_io",
            "info": { "type": "io", "version": "1.0" },
            "settings": {
                "usbid_vendor":  "[optional] Usb vendor ID in the following format (\"16c0\" : default)",
                "usbid_product": "[optional] Usb product ID in the following format (\"05e1\" : default)",
                "usbid_serial":  "[optional] Usb serial ID",
            }
        }

    ###########################################################################
    ###########################################################################

    def setup(self, tree):
        """FROM MetaDriver
        """
        # Initialize properties
        self.gpio_id = -1
        self.gpio_dir = 'out'
        self.gpio_val = 0
        self.polling_time_ms = 1000
        self.usbid_vendor = "16c0"
        self.usbid_product = "05e1"
        self.usbid_serial = None

        # Import settings
        if "settings" in tree:
            settings = tree["settings"]
            if "gpio_id" in settings:
                self.gpio_id = settings["gpio_id"]
            if "polling_time_ms" in settings:
                self.polling_time_ms = settings["polling_time_ms"]
            if "usbid_vendor" in settings:
                self.usbid_vendor = settings["usbid_vendor"]
            if "usbid_product" in settings:
                self.usbid_product = settings["usbid_product"]
            if "usbid_serial" in settings:
                self.usbid_serial = settings["usbid_serial"]

        # 
        usb_uuid = self.usbid_vendor + self.usbid_product
        self.log.debug(f"usb_uuid = {usb_uuid}")

        # Register commands
        self.register_command("value/set", self.__value_set)
        self.register_command("direction/set", self.__direction_set)

        # Init the bridge
        if usb_uuid not in DriverPicohaIO.Bridges:
            DriverPicohaIO.Bridges[usb_uuid] = PicohaBridge(usb_uuid, self.usbid_vendor, self.usbid_product, self.usbid_serial)
        self.bridge = DriverPicohaIO.Bridges[usb_uuid]

    ###########################################################################
    ###########################################################################

    def on_start(self):
        """From MetaDriver
        """
        pass
        
    ###########################################################################
    ###########################################################################

    def loop(self):
        """FROM MetaDriver
        """
        if self.gpio_dir == "in":
            new_val = self.bridge.get_io_value(self.gpio_id)
            if self.gpio_val != new_val:
                self.gpio_val = new_val
                self.push_io_value(new_val)
                self.log.debug(f"new val read : {self.gpio_val}")

        return self.bridge.loop()

    ###########################################################################
    ###########################################################################

    def __value_set(self, payload):
        """Apply set value request
        """
        # Parse request
        req = self.payload_to_dict(payload)
        req_value = req["value"]
        # Update value
        if self.bridge.set_io_value(self.gpio_id, req_value):
            self.gpio_val = req_value
            self.push_io_value(req_value)
            self.log.info(f"new value : {req_value}")
        else:
            self.log.error(f"fail setting new value : {req_value}")

    ###########################################################################
    ###########################################################################

    def __direction_set(self, payload):
        """Apply set direction request
        """
        # Parse request
        req = self.payload_to_dict(payload)
        req_direction = req["direction"]

        # Update direction
        ret = self.bridge.set_io_direction(self.gpio_id, req_direction)
        if   ret == None:
            self.log.error("Wierd behaviour during set_io_direction")
        elif ret == True:
            self.gpio_dir = req_direction
            self.push_io_direction(req_direction)
            self.log.info(f"new direction : {req_direction}")
        else:
            self.log.error(f"fail setting new direction : {req_direction}")

