"""
For each name in file_a find the closest string from file_b.
Usage:
    $ python3 find_closest_names.py file_a file_b
"""
import sys
import Levenshtein

file_a = sys.argv[1]
file_b = sys.argv[2]

with open(file_a) as f:
    names_a = [l.rstrip("\n") for l in f.readlines()]

with open(file_b) as f:
    names_b = [l.rstrip("\n") for l in f.readlines()]

def find_max(a, names_b):
    sim = [Levenshtein.jaro_winkler(a, b) for b in names_b]
    ix = sim.index(max(sim))
    return (a, names_b[ix], sim[ix])

result = [find_max(a, names_b) for a in names_a]

output = ["{},{},{:0.10}\n".format(x[0], x[1], x[2]) for x in result]

with open("data/output.txt", "w") as f:
    f.writelines(output)
