import os
import sys
 
# debug
print("REQUEST TO START PLATFORM IN TEST MODE WITH TREE (", sys.argv[1], ")")


# Covergae
os.environ["COVERAGE_PROCESS_START"] = os.getcwd() + '/.coveragerc'
print("COVERAGE_PROCESS_START > ", os.environ["COVERAGE_PROCESS_START"])
import coverage
cov = coverage.process_startup()
if not cov:
    print("COVERAGE FAIL !")
else:
    print("COVERAGE READY !")


# Start the platform
import sys
from loguru import logger
from panduza_platform import MetaPlatform

logger.remove()
logger.add(sys.stdout, format="[{time}] {level: <10}> {message}", level="DEBUG")

srv = MetaPlatform()
srv.force_log = True
srv.register_driver_plugin_discovery()
srv.load_tree_overide(sys.argv[1])
srv.run()
logger.warning("Platform stopped !")

