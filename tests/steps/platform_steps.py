import os
import time
from behave import *
from xdocz_helpers import AttachTextLog, PathToRsc
# from panduza import Core, Interface

###############################################################################
###############################################################################

# Required to parse arguments in steps, for example "{thing}"
use_step_matcher("parse")


###############################################################################
###############################################################################

@Given('the connected test platform interface')
def step(context):
    # context.current_test_client = Client(url=url, port=int(port))
    # AttachTextLog(context, f"Client created with url:'{url}' and port:'{port}'")
    pass


# Given 
# When the tree is required

