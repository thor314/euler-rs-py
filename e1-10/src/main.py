# reviewing algorithms with assistence of:
#
# http://www.s-anand.net/euler.html
# https://github.com/nayuki/Project-Euler-solutions/tree/master/python

import math
import time

def timing(f):
    """Convenient decorator to time a function. Borrowed from:
    https://stackoverflow.com/questions/5478351/python-time-measure-function#5478448"""
    def wrap(*args, **kwargs):
        time1 = time.time()
        ret = f(*args, **kwargs)
        time2 = time.time()
        print('{:s} function took {:.3f} ms'.format(f.__name__, (time2-time1)*1000.0))

        return ret
    return wrap

@timing
def e1():
    threes = 3 * 333 * 334 / 2
    fives = 5 * 199 * 200 / 2
    fifteens = 15 * 66 * 67 / 2
    print(threes + fives - fifteens)
    # review note, an alternate solution:
    # sum(x for x in range(1000) if x % 3 == 0 or x % 5 == 0)

@timing
def e2():
    init = (1,1)
    sum = 0 # review note, apparently I don't need to initialize variables in python, and could have disincluded this line, which seems gross, but is more concise
    while init[0] < 4000000:
        if init[0] % 2 == 0:
            sum += init[0]
        init = (init[0] + init[1], init[0])
    print(sum)

@timing
def e2_alt():
    """More precisely what I did in the rust solution, used a closure (lambda), which I wanted to make sure I still knew how to do in python."""
    init = (1,1)
    sum = 0
    f = lambda a,b: (a+b, a)
    while init[0] < 4000000:
        if init[0] % 2 == 0:
            sum += init[0]
        init = f(init[0],init[1])
    print(sum)

# in review: a functional solution that I find elegant:
"""
def e2_alt2():
    import itertools
    def fib():
        x,y = 0,1
        while True: # exploited laziness lol
            yield x
            x,y = y, x+y
    print(sum(x for x in itertools.takewhile(lambda x: x <= 4_000_000, fib()) if x % 2 == 0))
"""

@timing
def e3():
    """largest prime factor of 600851475143"""
    x = 600851475143
    root_x = math.sqrt(x)
    i = 3
    factors = []
    while i < root_x and x > 1:
        if x % i == 0:
            factors.append(i)
            x /= i
            i -= 2 # catch repeats
        i += 2
    print(factors)

def is_pal(i):
    s = str(i)
    if s == s[::-1]:
        return 1
    else:
        return 0

@timing
def e4():
    """Find the largest palindrome made from the product of two 3-digit numbers."""
    max = 0
    for i in range(100,999): # review note: could have used range(100,999,-1) to speed up
        for j in range(i,999):
            prod = i*j
            # review note: more is_pal is pretty short, could have one-lined it:
            # if str(i * j) == str(i * j)[::-1])
            if is_pal(prod) and prod > max:
                max = prod
    print(max)
# one liner: https://github.com/zacharydenton/euler/blob/master/004/palindrome.py
# print(max(a * b for a in range(100, 1000) for b in range(a, 1000) if str(a * b) == str(a * b)[::-1]))

@timing
def e5():
    print(19*17*16*13*11*9*7*5)

@timing
def e6():
    """Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum."""
    sum_of_squares = sum(map(lambda x: x*x, range(101)))
    summ = sum(range(101))
    square_of_sums = summ*summ

    print("diff of %s and %s is %s" % (sum_of_squares, square_of_sums, square_of_sums - sum_of_squares))
# one liner:
# print(sum(range(101))**2 - sum(x**2 for x in range(1,101)))

@timing
def e7():
    """get the 10,0001st prime"""
    # this is the first problem where the answer is not immediately obvious, if we accept that we should generate our own primes
    # naive algorithm: n=3, increment by 2, check if any element in the list primes divides n, stop when the 10,001st prime is collected
    count = 1
    primes = [2]
    n=3

    while count < 10001:
        b = True
        for p in primes:
            if n % p == 0:
                #print("%s divides %s" % (p, n))
                b = False
                # n is not prime
                break
        if b == True:
            #print("appending %s" % n)
            primes.append(n)
            #print("primes: ", primes)
            count+=1
        n+=2

    print(primes[-1])
# review: This solution was also fine, if verbose: https://github.com/saturnisbig/euler-python/blob/master/pro007.py
# But even more concisely, I could take a note from here, which implements a lazy Eratosthenes sieve:
# https://github.com/zacharydenton/euler/blob/master/007/find-primes.py

@timing
def e8():
    """get the 13 adjacent digits of e8.txt that produce the largest product.
    Struggled with opening a file for 20 minutes. Solved with an absolute path. Wtf, why not relative path in defaults?
    """
    from pathlib import Path
    import os
    import numpy
    # FileNotFoundError: [Errno 2] No such file or directory: 'e8.txt'
    if os.path.isfile('/home/thor/euler/e1-10/src/e8.txt'):
        print("file found") #this arm triggers
        s = Path('/home/thor/euler/e1-10/src/e8.txt').read_text().replace('\n','')
    else:
        print("wtf")

    max_prod = 0
    # cycle through s, taking each digit, mapping to int, multiplying, saving product if larger than max.
    for i in range(len(s) - 13):
        # TypeError: string indices must be integers, whoops, colon not comma
        v = s[i:i+13]
        #print("try:", v)
        # took awhile to find this:
        int_v = [int(i) for i in v]

        #AttributeError: 'map' object has no attribute 'astype'
        #v.astype(np.int)
        #TypeError: cannot perform reduce with flexible type
        prod = numpy.prod(int_v)
        if prod > max_prod:
            max_prod = prod
    print(max_prod)
# review: solution was fine, took me awhile to figure out how to python.
# I could have made use of the reduce method, https://github.com/saturnisbig/euler-python/blob/master/pro008.py#L38
# this was my favorite solution: https://github.com/nayuki/Project-Euler-solutions/blob/master/python/p008.py

@timing
def e9():
    for a in range(1,1001):
        for b in range(a,1001):
            for c in range(b,1001):
                if c*c == b*b + a*a:
                    #print("candidate: %s, %s, %s" % (a, b, c))
                    if a + b + c == 1000:
                        print("success! Pythagorean candidate found: %s, %s, %s, with product: %s" % (a, b,c, a*b*c))
                        return
# well I missed a pretty obvious optimization. In review, I could have written:
@timing
def e9_alt(): # lol, this took 17ms
    for a in range(1,1001):
        for b in range(a,1001):
            c = 1000-a-b
            if b*b == a*a+c*c:
                print(a,b,c)
                return

# alternatively, with a list comprehension (3x slower than last solution):
@timing
def e9_alt2(): #53ms
    print([(a,b,1000-a-b) for a in range(1,1001) for b in range(a,1000-a) if b*b==a*a+(1000-a-b)**2])

@timing
def e10_slow():
    """Find the sum of all the primes below two million."""
    k = 2_000_000
    r = [n for n in range(3,k,2)]
    i = 0
    while i < len(r) and i < math.sqrt(k):
        p = r[i]
        i +=1
        j = i
        while j < len(r):
            if p == 0:
                j+=1
                continue
            if r[j] % p == 0:
                # remove r[j] from r
                r[j] = 0
                #del r[j]
            j+=1

    print("first 100 primes?: %s" % r[:100])
    print("sum of primes: %s" % (2+sum(r)))
    # this problem ran in 80 seconds.
    # what could I have done more efficiently, presupposing I disallow myself to use a pre-generated list of primes?
    # No silly. I shouldn't be removing elements from an array with 2 million elements. Use a different data structure. Or just mark elements in a list as true or false.

@timing
def e10_a_little_faster_but_not_much(): # 20seconds
    k = 200_000
    r = [2,3,5,7]
    i = 9
    while i < k:
        for j in r:
            # the following check would have sped up my algorithm significantly
            if math.sqrt(j) > i:
                break
            if i % j == 0:
                #print("nope: %s" % i)
                break
        #print("yup: %s" % i)
        if i % j != 0:
            r.append(i)
        i+=2
    print("first 100 primes?: %s" % r[:100])
    print("sum of primes: %s" % (sum(r)))

# In review, I need a faster primes algorithm or to stop trying to remove elems from an array.

# from https://github.com/saturnisbig/euler-python/blob/master/pro010.py
def is_prime(num):
    if num < 2:
        return False
    if num == 2 or num == 3:
        return True
    # ignore even numbers
    for i in range(3, int(num**0.5+1), 2):
        if num % i == 0:
            return False
    return True

@timing
def e10_not_awful(n): # 4 seconds for 2million
    result = 2
    start = 3
    while True:
        if start >= n:
            break
        if is_prime(start):
            result += start
        start += 2
    return result

#https://github.com/zacharydenton/euler/blob/master/010/sum-primes.py
# using a clever algorithm that I certainly would not have naturally produced on my own, (but might have been able to find with a duckduck).
def eratosthenes():
    '''Yields, one by one, the sequence of prime numbers via a (really cool) modification of the Sieve of Eratosthenes.'''
    D = {}  # map composite integers to primes witnessing their compositeness
    q = 2   # first integer to test for primality
    while 1:
        if q not in D:
            yield q        # not marked composite, must be prime
            D[q*q] = [q]   # first multiple of q not already marked
        else:
            for p in D[q]: # move each witness to its next multiple
                D.setdefault(p+q,[]).append(p)
            del D[q]       # no longer need D[q], free memory
        q += 1
@timing
def e10_fast(): # 840ms
    from itertools import takewhile
    print(sum(takewhile(lambda x: x < 2000000, eratosthenes())))
