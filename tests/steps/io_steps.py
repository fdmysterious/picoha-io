import os
import time
from behave import *
from hamcrest import assert_that, equal_to
from xdocz_helpers import AttachTextLog, PathToRsc
from panduza import Core, Client

###############################################################################
###############################################################################

# Required to parse arguments in steps, for example "{thing}"
use_step_matcher("parse")

###############################################################################
###############################################################################

@Given('io interface "{io_name}" initialized with alias "{io_alias}"')
def step(context, io_name, io_alias):
    context.interfaces["io"][io_name].init(alias=io_alias)

###############################################################################
###############################################################################

@When('io interface "{io_name}" direction is set to "{direction}"')
def step(context, io_name, direction):
    context.interfaces["io"][io_name].direction.set(direction, ensure=True)

###############################################################################
###############################################################################

@Then('io interface "{io_name}" direction is "{direction}"')
def step(context, io_name, direction):
    assert_that(context.interfaces["io"][io_name].direction.get(), equal_to(direction))

###############################################################################
###############################################################################

@When('io interface "{io_name}" value is set to "{value}"')
def step(context, io_name, value):
    context.interfaces["io"][io_name].value.set(int(value), ensure=True)

###############################################################################
###############################################################################

@Then('io interface "{io_name}" value is "{value}"')
def step(context, io_name, value):
    assert_that(context.interfaces["io"][io_name].value.get(), equal_to(int(value)))

