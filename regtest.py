#! /usr/bin/env python3

from os import path
from multiprocessing.pool import ThreadPool
import multiprocessing
import os
import logging
import json
import psutil
import argparse
import subprocess
import re
import glob
import time
import sys
import shlex

# OVERRIDE_FIELDS = ['verifiers', 'memory', 'time-limit', 'memory-limit', 'skip']
# APPEND_FIELDS = ['flags', 'checkbpl', 'checkout']

LANGUAGES = {'c': {'*.c'},
             'cargo': {'Cargo.toml'},
             'cplusplus': {'*.cpp'},
             'rust': {'*.rs'},
             'llvm-ir': {"*.ll"}}


def bold(text):
    return '\033[1m' + text + '\033[0m'


def red(text, log_file):
    if log_file:
        return text
    else:
        return '\033[0;31m' + text + '\033[0m'


def green(text, log_file):
    if log_file:
        return text
    else:
        return '\033[0;32m' + text + '\033[0m'


def get_result(output):
    if re.search(r'SMACK timed out', output):
        return 'timeout'
    elif re.search(r'SMACK found no errors', output):
        return 'verified'
    elif re.search(r'SMACK found an error', output):
        return 'error'
    else:
        return 'unknown'


def merge(metadata, yamldata):
    for key in OVERRIDE_FIELDS:
        if key in yamldata:
            metadata[key] = yamldata[key]

    for key in APPEND_FIELDS:
        if key in yamldata:
            if key in metadata:
                metadata[key] += yamldata[key]
            else:
                metadata[key] = yamldata[key]


def metadata(file):
    m = {
         "name": os.path.basename(os.path.dirname(file)),
         "expect": None,
         "time-limit": 1200,
         "bv": False,
         "incremental": False,
         "smteq": False
        }

    with open(file) as f:
         m.update(json.load(f))

    return m


# integer constants
PASSED = 0
TIMEDOUT = 1
UNKNOWN = 2
FAILED = -1


def process_test(
        cmd,
        test,
        verifier,
        solver,
        expect,
        log_file):
    """
    This is the worker function for each process. This function process the
    supplied test and returns a tuple containing  indicating the test results.

    :return: A tuple with the
    """
    # str_result = "{0:>20}\n".format(test)
    # str_result += "{0:>20} {1:>10}    :".format(verifier, solver)
    str_result = "{},{},{},".format(test, verifier, solver)

    t0 = time.time()
    p = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                         universal_newlines=True)
    out, err = p.communicate()
    elapsed = time.time() - t0
    status = 0

    # get the test results
    result = get_result(out + err)
    if result == expect and status == 0:
        str_result += 'PASSED,'
    elif result == 'timeout':
        str_result += red('TIMEOUT', log_file)
    elif result == 'unknown':
        str_result += red('UNKNOWN', log_file)
    else:
        str_result += red('FAILED ', log_file)

    str_result += '%.2fs' % round(elapsed, 2)
    return str_result


passed = failed = timeouts = unknowns = 0


def tally_result(result):
    """
    Tallies the result of each worker. This will only be called by the main
    thread.
    """
    # log the info
    logging.info(result)

    global passed, failed, timeouts, unknowns
    if "PASSED" in result:
        passed += 1
    elif "FAILED" in result:
        failed += 1
    elif "TIMEOUT" in result:
        timeouts += 1
    elif "UNKNOWN" in result:
        unknowns += 1


def get_extensions(languages):
    languages = list(languages.split(','))
    extensions = set()
    for language in languages:
        extensions |= LANGUAGES[language]
    return extensions


def get_tests(folder):
    pat = path.join(folder, '*/*', "test.json")
    tests = list(glob.glob(pat))
    tests.sort()
    return tests

def get_sources(test_cfg):
    folder = os.path.dirname(test_cfg)
    tests = []
    for ext in ('*.rs', '*.c'):
        pat = path.join(folder, ext)
        tests = tests + list(glob.glob(pat))
    return tests

def get_config(test):
    args = []
    with open(test, 'r') as f:
        conf = json.load(f)
    args.extend(conf['files'])
    args.extend(['--unroll', conf['unroll']])
    if conf['bit-vector']:
        args.extend(['--integer-encoding', 'bit-vector'])
    return args

def main():
    """
    Main entry point for the test suite.
    """
    t0 = time.time()
    num_cpus = multiprocessing.cpu_count()
    mem_total = psutil.virtual_memory().total / (1024 * 1024)

    parser = argparse.ArgumentParser()

    parser.add_argument(
        "--threads",
        action="store",
        dest="n_threads",
        default=num_cpus,
        type=int,
        help='''execute regressions using the selected number of threads in
                parallel''')
    parser.add_argument(
        "--output-log",
        action="store",
        dest="log_path",
        type=str,
        help="sets the output log path. (std out by default)")

    args = parser.parse_args()

    script_directory = os.path.dirname(os.path.abspath(sys.argv[0]))
    tests = get_tests(script_directory)

    # configure the logging
    log_format = ''
    log_level = logging.DEBUG

    logging.basicConfig(format=log_format, level=log_level)

    logging.debug("Creating Pool with '%d' Workers" % args.n_threads)
    p = ThreadPool(processes=args.n_threads)

    try:
        # start the tests
        logging.info("Running regression tests...")

        # start processing the tests.

        results = []
        for test in tests:
            # get the meta data for this test
            meta = metadata(test)

            test_files = get_sources(test)
            # for (verifier, solver) in ...

            # build up the subprocess command
            cmd = ['smack'] + test_files
            cmd += ['--time-limit', str(meta['time-limit'])]
            cmd += ['--float']
            if meta['bv']:
                cmd += ['--integer-encoding', 'bit-vector']
            if meta['smteq']:
                cmd += ['--transform-bpl', './smteq']

            verifier = 'boogie'
            solver = 'z3'
            cmd += ['--verifier', verifier, '--solver', solver]
            # print(" ".join(cmd))
            r = p.apply_async(
                    process_test,
                    args=(
                        cmd[:],
                        meta['name'],
                        verifier,
                        solver,
                        meta['expect'],
                        args.log_path
                    ),
                    callback=tally_result)
            results.append(r)

        # keep the main thread active while there are active workers
        for r in results:
            r.wait()

    except KeyboardInterrupt:
        logging.debug("Caught KeyboardInterrupt, terminating workers")
        p.terminate()  # terminate any remaining workers
        p.join()
    else:
        logging.debug("Quitting normally")
        # close the pool. this prevents any more tasks from being submitted.
        p.close()
        p.join()  # wait for all workers to finish their tasks

    # log the elapsed time
    elapsed_time = time.time() - t0
    logging.info(' ELAPSED TIME [%.2fs]' % round(elapsed_time, 2))

    # log the test results
    logging.info(' PASSED count: %d' % passed)
    logging.info(' FAILED count: %d' % failed)
    logging.info(' TIMEOUT count: %d' % timeouts)
    logging.info(' UNKNOWN count: %d' % unknowns)

    # if there are any failed tests or tests that timed out, set the system
    # exit code to a failure status
    if timeouts > 0 or failed > 0 or unknowns > 0:
        sys.exit(1)


if __name__ == "__main__":
    main()
