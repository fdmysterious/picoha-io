import pyudev


def TTYPortfromUsbInfo(vendor_id, product_id, serial=None, base_dev_tty="/dev/ttyACM"):
    """Find tty port from usb information
    """
    # Explore usb device with tty subsystem
    udev_context = pyudev.Context()
    # for device in udev_context.list_devices(ID_BUS='usb', SUBSYSTEM='tty'):
    for device in udev_context.list_devices(SUBSYSTEM='tty'):
        properties = dict(device.properties)
        
        #
        # logger.debug(f"{properties}")

        # Need to find the one with the DEVNAME corresponding to the /dev serial port
        if 'DEVNAME' not in properties or not properties['DEVNAME'].startswith(base_dev_tty):
            continue

        # Check vendor/product/serial
        if vendor_id == properties["ID_VENDOR_ID"] and product_id == properties["ID_MODEL_ID"]:
            if serial:
                if serial == properties["ID_SERIAL_SHORT"]:
                    return properties["DEVNAME"]
            else:
                return properties["DEVNAME"]

    return None

