from behave import fixture


@fixture
def client(context, name):
    # -- SETUP-FIXTURE PART:
    if "clients" not in context:
        context.clients = dict()

    context.clients[name] = None

    # -- READY FOR THE STEP --
    yield context.clients[name]

    # -- CLEANUP-FIXTURE PART:

