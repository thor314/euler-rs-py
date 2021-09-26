import math
import itertools
from importlib.machinery import SourceFileLoader
time = SourceFileLoader("main","../../e1-10/src/main.py").load_module()

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
@time.timing
def e41():
    from itertools import takewhile
    LIMIT=10_000_000
    primes=list(takewhile(lambda n: n < LIMIT,eratosthenes()))
    pandigital_primes=filter(lambda n: is_pandigital(n),primes)
    max_=max(pandigital_primes)
    print(max_)

def is_pandigital(n):
    s=str(n)
    size=len(s)
    for i in range(1,size+1):
        if not str(i) in s:
            return False
    return True

@time.timing
def e42():
    from pathlib import Path
    words = Path('/home/thor/euler/e41-50/src/p042_words.txt').read_text().replace("\"", "").split(',');
    tris = set(i*(i+1)/2 for i in range(30))
    #print(tris)
    twords = [word for word in words if score(word) in tris]
    print(len(twords))
def score(s):
    tot=0
    for c in s:
        tot+=ord(c)-64
    return tot

#Find the sum of all 0 to 9 pandigital numbers with this property:
# d2d3d4 is div by 2
# d3d4d5 is div by 3
# d4d5d6 is div by 5
# ...
# d8d9d10 is div by 17
@time.timing
def e43():
    from itertools import permutations
    ps = [2,3,5,7,11,13,17]
    pans=(i for i in map(lambda v: int(''.join(str(i) for i in v)), permutations(range(10))) if (i%1000)%17==0)
    my_pans=filter(lambda x: is_special(x,ps), pans)
    #print(sum(pans),pans)
    print(sum(my_pans))
def is_special(n,arr):
    for (i,v) in enumerate(arr):
        m=int(str(n)[i+1:i+3+1])
        if not m % v ==0:
            return False
    return True

@time.timing
def e43_now_with_strings():
    from itertools import permutations
    ps = [2,3,5,7,11,13,17]
    pans= map(lambda v: ''.join(str(i) for i in v), permutations(range(10)))
    my_pans=map(lambda s: int(s), filter(lambda x: is_special_str(x,ps), pans))
    #print(sum(pans),pans)
    print(sum(my_pans))
def is_special_str(s,arr):
    for (i,v) in enumerate(arr):
        m=int(s[i+1:i+3+1])
        if not m % arr[i] ==0:
            return False
    return True

# Find the pair of pentagonal numbers, Pj and Pk, for which their sum and difference are pentagonal and D = |Pk − Pj| is minimised; what is the value of D?

# tricky!
@time.timing
def e44():
    pent_set = set()
    candidates=set()
    #for i in itertools.count():
    for i in range(5_000):
        p_i = i*(3*i-1)//2
        for p in pent_set:
            if p_i-p in pent_set:
                #print("i:",i, "p_i:", p_i, "p_i-p:", p_i-p, "check p_i+p:",p_i+p)
                candidates.add((p_i+p,p_i-p))

        for (c,d) in candidates:
            if p_i == c:
                print("index", i, "found!",c, "diff",d)
                break

        candidates = set(filter(lambda e: e[0] > p_i, candidates))
        #print(i,"candidates remaining:", candidates)
        pent_set.add(p_i)
