from behave import *
from xdocz_helpers import AttachTextLog, PathToRsc
from panduza import Core

###############################################################################
###############################################################################

# Required to parse arguments in steps, for example "{thing}"
use_step_matcher("parse")

###############################################################################
###############################################################################

@Given('core aliases loaded with file "{filepath}"')
def step(context, filepath):
    Core.LoadAliases(json_filepath=PathToRsc(filepath))

