import time
import math

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

@timing
def e2():
    init = (1,1)
    sum = 0
    while init[0] < 4000000:
        if init[0] % 2 == 0:
            sum += init[0]
        init = (init[0] + init[1], init[0])
    print(sum)

@timing
def e2k():
    """more precisely what I did in the rust solution, used a closure (lambda)"""
    init = (1,1)
    sum = 0
    f = lambda a,b: (a+b, a)
    while init[0] < 4000000:
        if init[0] % 2 == 0:
            sum += init[0]
        init = f(init[0],init[1])
    print(sum)

@timing
def e3():
    """largest prime factor of 600851475143g"""
    x = 600851475143
    root_x = math.sqrt(x)
    i = 3
    factors = []
    while i < root_x and x > 1:
        if x % i == 0:
            factors.append(i)
            x /= i
            i -= 2
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
    for i in range(100,999):
        for j in range(i,999):
            prod = i*j
            if is_pal(prod) and prod > max:
                max = prod
    print(max)

@timing
def e5():
    print(19*17*16*13*11*9*7*5)
