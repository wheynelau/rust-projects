from random import random
import time
import os

inside = 0
iters = int(os.environ["ITER"])

start = time.time()
for i in range(iters):
    x, y = random(), random()
    if x**2 + y**2 <= 1:
        inside += 1

print(f"Time taken for {iters}: ", time.time() - start)

print(4 * inside / iters)
