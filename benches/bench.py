import os
import timeit
try:
    from ukkonen import distance
except ImportError:
    distance = None

from ukkonen_rs import ukkonen

DIR = os.path.dirname(__file__)

with open(os.path.join(DIR, 'gpl.txt')) as f:
    gpl = f.read()

with open(os.path.join(DIR, 'apache.txt')) as f:
    apache2 = f.read()

NUM_ITER = 10**6

if distance is not None:
    cpp_time = timeit.timeit(lambda: distance(gpl, apache2, 1757), number=NUM_ITER)
    print(f'C++ took {cpp_time}us')

rs_time = timeit.timeit(lambda: ukkonen(gpl, apache2, 1757), number=NUM_ITER)

print(f'Rust took {rs_time}us')
