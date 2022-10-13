import time
from behave import *
from hamcrest import assert_that, equal_to
from xdocz_helpers import AttachTextLog, PathToRsc

###############################################################################
###############################################################################

# Required to parse arguments in steps, for example "{thing}"
use_step_matcher("parse")

###############################################################################
###############################################################################

@Step('wait for "{sleep_time}" seconds')
def step(context, sleep_time):
    time.sleep(int(sleep_time))
