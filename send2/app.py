import time
import random


overall = 0.0
cycles = 0
while True:
    starttime = time.time()
    i = 0
    count = 0
    while i < 1000000:
        i = i+1

        rnd = random.randint(0,10)
        count = count + rnd
    end = time.time()
    diff = end-starttime
    print("Reached: "+str(count)+" in "+str(diff)+" seconds")
    overall=overall+diff
    cycles = cycles+1
    print("avg: "+str(overall/cycles))
    
