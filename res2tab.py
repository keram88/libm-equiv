#! /usr/bin/env python3

import sys
import csv
from pprint import pprint

def verif_name(verifier: str, solver: str):
    return verifier.capitalize() + "/" + solver.upper()

def get_status(res):
    if any(map(lambda x: x[0] == 'VERIFIED', res.values())):
        return 'Equivalent'
    if any(map(lambda x: x[0] == 'FAILED', res.values())):
        return 'Error'
    return 'Unknown'

def get_time(res):
    if res[0] == 'TIMEOUT':
        return 'TO'
    time = float(res[1])
    stddev = float(res[2])

    # return "{:.1f}$\pm${:.2f}s".format(time, stddev)
    return "{:.1f}±{:.2f}s".format(time, stddev)

rawres = []
with open(sys.argv[1], 'r') as f:
    reader = csv.reader(f, delimiter=',')
    for row in reader:
        rawres.append(row)

# discard header
rawres = rawres[1:]

res = {bench[0]:dict() for bench in rawres}

solvers = set()
for x in rawres:
    solver = verif_name(x[1], x[2])
    solvers.add(solver)
    res[x[0]][verif_name(x[1], x[2])] = x[3:]

solvers = sorted(solvers)

# Header
print(" & ".join(['Function Name', 'Status'] + solvers))

for k in sorted(res.keys()):
    print(" & ".join([k, get_status(res[k])] + [get_time(res[k][s]) for s in solvers]))
