import math

# imports the module from the given path
# ref https://www.geeksforgeeks.org/how-to-import-a-python-module-given-the-full-path/
from importlib.machinery import SourceFileLoader
time = SourceFileLoader("main","../../e1-10/src/main.py").load_module()

# read the 20x20 grid from a file, parse it into a 2d array, search in the 4 directions for the max product
@time.timing
def e11():
    from pathlib import Path
    import os
    # FileNotFoundError: [Errno 2] No such file or directory: 'e8.txt'
    if os.path.isfile('/home/thor/euler/e11-20/src/e11.txt'):
        s = Path('/home/thor/euler/e11-20/src/e11.txt').read_text()
        #print(s)
    # parse it (how?)
    # this took about 10 mins to figure out
    rows = s.split('\n')
    v=[]
    i=0
    for row in rows:
        r = [int(k) for k in row.split()]
        v.insert(i,r)
        i+=1
    # now search:
    # h: v[i,j..j+3], v: v[i..i+3,j], d1: v[i..i+3,j..j+3], d2: v[i..i+3,j..j-3]
    big=0
    for i in range(7,20):
        for j in range(20):
            if j < 17: # horizontal
                temp = v[i][j] * v[i][j+1] * v[i][j+2] * v[i][j+3]
                if temp > big:
                    big = temp
                    print("h_ new biggest: %s starting at: (%s,%s), with init value %s:" % (big,i,j, v[i][j]))
                if i < 17: # d1
                    temp = v[i][j] * v[i+1][j+1] * v[i+2][j+2] * v[i+3][j+3]
                    if temp > big:
                        big = temp
                        print("d\ new biggest: %s starting at: (%s,%s), with init value %s:" % (big,i,j, v[i][j]))

            if i < 17: # vertical
                temp = v[i][j] * v[i+1][j] * v[i+2][j] * v[i+3][j]
                if temp > big:
                    big = temp
                    print("v| new biggest: %s starting at: (%s,%s), with init value %s:" % (big,i,j, v[i][j]))
                if j > 2: # d2
                    temp = v[i][j] * v[i+1][j-1] * v[i+2][j-2] * v[i+3][j-3]
                    if temp > big:
                        big = temp
                        print("d/ new biggest: %s starting at: (%s,%s), with init value %s:" % (big,i,j, v[i][j]))

    print(big)
    # 10m to write this part... but oof, wrong answer. debugggggg.
    # the numbers 51267216 = product 66 91 88 97, do exist vertically in the grid.
    # edge conditions? My edges seem fine.
    # not counting up to biggest? Seems not.
    # oh. I transposed a line in d/. Lame. That took 30m to find.

def multiplicities(inp):
    d = {}
    n = inp
    if n < 2:
        return d
    while n%2==0:
        if not d.get(2):
            d[2] = 1
        else:
            d[2] = d[2]+1

        n /= 2
        #print("2: %s" % d.get(2))
    i = 3;
    while n > 1:
        while n % i == 0:
            if not d.get(i):
                d[i] = 1
            else:
                d[i] = d[i]+1

            #print("%s: %s" % (i,d.get(i)))
            n /= i
        i+=2
    #print("%s has multiplicities: %s" %(inp, d))
    return d

@time.timing
def e12():
    import functools
    i = 2
    sum = 1
    while True:
        sum+=i
        i+=1
        divisors = multiplicities(sum)
        n_divisors = 1
        f = lambda acc, x: acc * (x+1)
        [n_divisors := f(n_divisors, x) for x in divisors.values()]

        if n_divisors > 500:
            print("value: %s, the %sth triangle with %s divisors" %(sum, i, n_divisors))
            break
