import time
import signal
import logging
import subprocess
from steps.xdocz_helpers import PathToRsc

PLATFORM_PROC=None
PLATFORM_LOGF=None

###############################################################################
###############################################################################

def platform_start(context, treefile):
    """Start the platform
    """
    global PLATFORM_PROC
    global PLATFORM_LOGF

    # Prepare logging file
    PLATFORM_LOGF = open('test_reports/platform_log.txt', 'a')

    # Start the subprocess with the platform
    platform_run_script = PathToRsc('test-py-platform-run.py')
    logging.debug(f"##### RUN PLATFORM WITH: {platform_run_script} #####")
    treefilepath = PathToRsc(treefile)
    PLATFORM_PROC = subprocess.Popen(["python3", platform_run_script, treefilepath], stdout=PLATFORM_LOGF, stderr=PLATFORM_LOGF)

    # Let some time to the platform to boot
    time.sleep(3)

###############################################################################
###############################################################################

def platform_stop(context):
    """Stop the platform
    """
    global PLATFORM_PROC
    global PLATFORM_LOGF
    logging.debug(f"STOPING PLATFORM...")
    if PLATFORM_PROC:
        PLATFORM_PROC.send_signal(signal.SIGINT)
        PLATFORM_PROC=None
    if PLATFORM_LOGF:
        PLATFORM_LOGF.close()
        PLATFORM_LOGF=None
    logging.debug(f"##### PLATFORM STOPPED ! #####")
