import math
from importlib.machinery import SourceFileLoader
time = SourceFileLoader("main","../../e1-10/src/main.py").load_module()

# How many different ways can £2 be made using any number of coins?
@time.timing
def e31():
    coins = list(reversed([1,2,5,10,20,50,100,200])) # len 8
    amt = 200;
    print(num_ways_sum(coins,amt,[]))

def num_ways_sum(arr,total,debug_arr):
    if total == 0:
        #print(debug_arr, sum(debug_arr))
        return 1
    else:
        final = 0
        arr = list(filter(lambda x: x <= total,arr))
        for (i, coin) in enumerate(arr):
            new_debug_arr = [k for k in debug_arr]
            new_debug_arr.append(coin)
            #print("arr: %s; total: %s, debug_arr:%s; arr[%s:]: %s"%(arr,total,debug_arr,i,arr[i]))
            final += num_ways_sum(arr[i:], total-coin, new_debug_arr)
        return final

@time.timing
def e32():
    set_ = set()
    for a in filter(lambda a: unique_digits_not_zero(a),range(102,9877)):
        digits_a=set(str(a))
        for b in filter(lambda b: b % 11 != 0 and
                        a*b<10_000 and
                        len(digits_a.intersection(set(str(b)))) == 0, range(2,99)):
            c=a*b
            digits_b=set(str(b))
            digits_c=set(str(c))
            if unique_digits_not_zero(c) and len(digits_c)+len(digits_b)+len(digits_a)==9 and len(digits_c.intersection(digits_a)) == 0 and len(digits_c.intersection(digits_b)) == 0:
                print(a,b,a*b)
                set_.add(a*b)
    print((sum(set_),set_))

def unique_digits_not_zero(n):
    digits_n=set(str(n))
    return len(str(n))==len(digits_n) and '0' not in digits_n

@time.timing
def e33():
    from functools import reduce;
    v = [(i,j) for i in range(1,100) for j in range(i,100) if is_curious(i,j)]
    print(v)
    product=reduce(lambda a,b: (a[0]*b[0],a[1]*b[1]),v)
    print(reduced(product[0],product[1])[1])

def reduced(a,b):
    g = math.gcd(a,b)
    return (a/g,b/g)

def is_curious(a,b):
    dsa = set(str(a))
    dsb = set(str(b))
    intersect = set(dsa.intersection(dsb))
    if len(intersect)==0:
        return False
    else:
        if '0' in intersect:
            return False
        else:
            for c in intersect:
                dsa.remove(c)
                dsb.remove(c)
            if len(dsa)>0 and len(dsb)>0:
                b_=int("".join(dsb))
                a_=int("".join(dsa))
                curious=reduced(a_,b_)
                reduced_=reduced(a,b)
                return curious==reduced_
            else:
                return False

@time.timing
def e34():
    facs = list(map(lambda x: fac(x),range(10)))
    lim=2_200_000
    v= [n for n in range(3,lim) if n==sum(map(lambda k: facs[int(k)],list(str(n))))]
    print(sum(v),v)

def fac(n):
    if n==0 or n==1:
        return 1
    else:
        return n*fac(n-1)

#How many circular primes are there below one million?
@time.timing
def e35():
    primes=primes_lt(1_000_000)
    primes_lt_1m=set(filter(lambda x: circular(x,primes),primes))
    print(len(primes_lt_1m),primes_lt_1m)

def primes_lt(n):
    from itertools import takewhile
    s = set(takewhile(lambda x: x < n, eratosthenes()))
    return s

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
def circular(n,primes):
    if n not in primes:
        return False
    s=str(n)
    # eg, s=197:
    for _i in range(len(s)):
        s=s[1:]+s[0]
        if int(s) not in primes:
            #print(s,"not in primes")
            return False
    return True

@time.timing
def e36():
    v = [i for i in range(1,1_000_000) if is_pal_d(i) if is_pal_b(i)]
    print(sum(v),v)

def is_pal_d(n):
    s=str(n)
    return s==s[::-1]
    True
def is_pal_b(n):
    s=str(to_bin(n))
    return s==s[::-1]
def to_bin(n):
    print(bin(n)[2:])
    return bin(n)[2:]


@time.timing
def e37():
    #from itertools import next;
    primes = set()
    v=[]
    e=eratosthenes()
    while len(v) < 11:
        p=e.__next__()
        primes.add(p)
        if is_prime_trunc(p, primes):
            v.append(p)
    print(sum(v),v)

def is_prime_trunc(p,primes):
    if p in [2,3,5,7]:
        return False
    nn,nm=p,p
    while nn > 0:
        #print(p,nn,nm)
        if not nn in primes or not nm in primes:
            return False
        nn //= 10
        nm %= 10**(len(str(nm))-1)
    return True

#What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?
@time.timing
def e38():
    v = [n for n in map(lambda x: e38_expand(x),range(1,10_000)) if is_pandigital(n)]
    print(max(v),v)

def e38_expand(n):
    m=0;
    counter=1
    while len(str(m)) < 9:
        temp=n*counter
        m *= 10**len(str(temp))
        m += temp
        counter+=1
    return m

def is_pandigital(n):
    s=set("123456789")
    return len(set(str(n)).intersection(s))==9 and len(str(n))==9

import unittest
class MyTest(unittest.TestCase):
    def test_num_ways_sum(self):
        coins = [10,5,2,1]
        self.assertEqual(num_ways_sum(coins,2,[]),2)
        self.assertEqual(num_ways_sum(coins,3,[]),2)
        self.assertEqual(num_ways_sum(coins,4,[]),3)
        self.assertEqual(num_ways_sum(coins,5,[]),4)

if __name__ == '__main__':
    unittest.main()
