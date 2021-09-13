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
    summ = 1
    while True:
        summ+=i
        i+=1
        divisors = multiplicities(summ)
        n_divisors = 1
        f = lambda acc, x: acc * (x+1)
        [n_divisors := f(n_divisors, x) for x in divisors.values()]

        if n_divisors > 500:
            print("value: %s, the %sth triangle with %s divisors" %(summ, i, n_divisors))
            break

@time.timing
def e13():
    from pathlib import Path
    import os
    # FileNotFoundError: [Errno 2] No such file or directory: 'e8.txt'
    if os.path.isfile('/home/thor/euler/e11-20/src/e13.txt'):
        s = Path('/home/thor/euler/e11-20/src/e13.txt').read_text()
    rows = s.strip().split('\n')

    rows13 = list(map(lambda s: int(s[:13]), rows))
    print("e13:", str(sum(rows13))[:10])

def collatz(n):
    if n % 2 == 0:
        return n/2
    else:
        return n*3+1

@time.timing
def e14():
    h = {1:0}

    it_counter = 0
    biggest = (0,0)
    for it in range(2,1_000_000):
        #print(it, h)
        if it in h:
            continue

        nextt = collatz(it)
        it_counter+=1
        cache = [(it, it_counter)]
        while nextt not in h:
            it_counter +=1
            cache.append((nextt, it_counter))
            nextt = collatz(nextt)

        count_last = h[nextt]
        count_for_it = count_last + it_counter

        for n,c in cache:
            #print(n,c)
            count = count_for_it + 1 - c
            h[n] = count
        it_counter = 0

        if count_for_it > biggest[0]:
            biggest = (count_for_it, it)

    print("biggest seq len: %s, for n=%s" % (biggest[0], biggest[1]))

@time.timing
def e15():
    from functools import reduce
    a = reduce(lambda a, b: a*b, range(21,41))
    b = reduce(lambda a, b: a*b, range(1,21))
    print(a/b)

# What is the sum of the digits of the number 2**1000?
@time.timing
def e16():
    from functools import reduce
    digits = str(2**1000)
    a = reduce(lambda a, b: int(a)+int(b), digits)
    print(a)


@time.timing
def e17():
    d = {
        0: 0,
        1: 3,
        2: 3,
        3: 5,
        4: 4,
        5: 4,
        6: 3,
        7: 5,
        8: 5,
        9: 4,
        10: 3,
        11: 6,
        12: 6,
        13: 8,
        14: 8,
        15: 7,
        16: 7,
        17: 9,
        18: 8,
        19: 8,
        20: 6,
        30: 6,
        40: 5,
        50: 5,
        60: 5,
        70: 7,
        80: 6,
        90: 6,
    }

    ans = sum(map(lambda x: count_letters(x,d), range(1001)))
    print(ans)

def count_letters(x,d):
    a,b,c,e = x % 10, x // 10 % 10, x // 100 % 10, x // 1000 % 10
    #print(e,c,b,a)
    if b == 1:
        aa=0
    else:
        aa=d[a]
    if b == 1:
        bb=d[b*10 + a]
    else:
        bb=d[b*10]
    if c > 0:
        if aa ==0 and bb==0:
            cc = 7+d[c]
        else:
            cc = 3+7+d[c]
    else:
        cc = 0
    if e > 0:
        ee=8 + d[e]
    else:
        ee=0

    #print("%s: %s,%s,%s,%s" % (x,ee,cc,bb,aa))
    return aa+bb+cc+ee
