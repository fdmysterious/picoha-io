from behave import fixture

from panduza import Io

@fixture
def interface_io(context, name):
    # -- SETUP-FIXTURE PART:
    if "interfaces" not in context:
        context.interfaces = dict()
    if "io" not in context.interfaces:
        context.interfaces["io"] = dict()
    context.interfaces["io"][name] = Io()

    # -- READY FOR THE STEP --
    yield context.interfaces["io"][name]

    # -- CLEANUP-FIXTURE PART:

