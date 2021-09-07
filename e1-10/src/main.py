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

@timing
def e6():
    """Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum."""
    sum_of_squares = sum(map(lambda x: x*x, range(101)))
    summ = sum(range(101))
    square_of_sums = summ*summ

    print("diff of %s and %s is %s" % (sum_of_squares, square_of_sums, square_of_sums - sum_of_squares))

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


from pathlib import Path
import os
import numpy
@timing
def e8():
    """get the 13 adjacent digits of e8.txt that produce the largest product.
    Struggled with opening a file for 20 minutes. Solved with an absolute path. Wtf, why not relative path in defaults?
    """
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

@timing
def e9():
    for a in range(1,1001):
        for b in range(a,1001):
            for c in range(b,1001):
                if c*c == b*b + a*a:
                    #print("candidate: %s, %s, %s" % (a, b, c))
                    if a + b + c == 1000:
                        print("success! Pythagorean candidate found: %s, %s, %s, with product: %s" % (a, b,c, a*b*c))


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
            if r[j] % p == 0:
                # remove r[j] from r
                del r[j]
            j+=1

    print("first 100 primes?: %s" % r[:100])
    print("sum of primes: %s" % (2+sum(r)))
    # this problem ran in 80 seconds.
    # what could I have done more efficiently, presupposing I disallow myself to use a pre-generated list of primes?
    # Well, I might have tried instead

@timing
def e10_even_slower():
    """unfortunately, this ain't better. About 7* slower in fact."""
    k = 200_000
    r = [2,3,5,7]
    i = 9
    while i < k:
        for j in r:
            if i % j == 0:
                #print("nope: %s" % i)
                break
        #print("yup: %s" % i)
        if i % j != 0:
            r.append(i)
        i+=2
    print("first 100 primes?: %s" % r[:100])
    print("sum of primes: %s" % (sum(r)))
