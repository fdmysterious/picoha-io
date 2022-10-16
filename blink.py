import time
from panduza import Core, Client, Io

"""
{
    "name": "led",
    "driver": "picoha_io",
    "settings": {
        "gpio_id": 25,
        "polling_time_ms": 10000,
        "usbid_vendor": "16c0",
        "usbid_product": "05e1"
    }
},
{
    "name": "test",
    "driver": "picoha_io",
    "settings": {
        "gpio_id": 0,
        "polling_time_ms": 10000,
        "usbid_vendor": "16c0",
        "usbid_product": "05e1"
    }
}
"""

Core.LoadAliases(
    {
        "local_test": {
            "url": "localhost",
            "port": 1883,
            "interfaces": {
                "pico_led": "pza/test/picoha_io/led",
                "test": "pza/test/picoha_io/test",
                "test_out": "pza/test/picoha_io/test_out"
            }
        }
    }
)


# io_led = Io(alias="pico_led")
# io_led.direction.set("out", ensure=True)

# io_led.value.set(0, ensure=True)
# time.sleep(1)
# io_led.value.set(1, ensure=True)
# time.sleep(1)
# io_led.value.set(0, ensure=True)
# time.sleep(1)
# io_led.value.set(1, ensure=True)


io_in = Io(alias="test")
io_in.direction.set("in", ensure=True)

io_out = Io(alias="test_out")
io_out.direction.set("out", ensure=True)

io_out.value.set(1, ensure=True)
time.sleep(2)
print( io_in.value.get() ) # need wait for value operation

io_out.value.set(0, ensure=True)
time.sleep(2)
print( io_in.value.get() )

