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


@time.timing
def e45():
    t=lambda n: n*(n+1)//2
    p=lambda n: n*(3*n-1)//2
    h=lambda n: n*(2*n-1)
    tk,pk,hk=(1,1),(1,1),(1,1)
    for i in range(1,1_000_000):
        if tk[1]==pk[1] and pk[1]==hk[1]:
            print(i,tk,pk,hk)
        m=min(min(tk[1],pk[1]),hk[1])
        if tk[1] == m:
            tk = (tk[0]+1,t(tk[0]+1))
        elif pk[1] ==m:
            pk = (pk[0]+1,p(pk[0]+1))
        else:
            hk = (hk[0]+1,h(hk[0]+1))


@time.timing
def e46():
    from itertools import takewhile
    candidates = [True]*1_000_000
    candidates[1]=False
    for i in range(len(candidates)//2):
        candidates[i*2]=False

    primes=list(takewhile(lambda n: n < 10_000,eratosthenes()))
    for p in primes:
        candidates[p]=False

    c=0
    for n in range(50):
        c+=1
        n=c*c*2
        for p in primes:
            candidates[n+p]=False
    print(candidates.index(True))

def prime_factors(x):
    factors = set()
    while x % 2 == 0:
        factors.add(2)
        x/=2

    breakpt = math.sqrt(x)
    i = 3
    while x > 1:
        if x == breakpt+1:
            factors.add(x)
            break
        while x % i == 0:
            factors.add(i)
            x /= i
            breakpt = math.sqrt(x)
        i += 2

    return(factors)

@time.timing
def e47():
    running=[]
    from itertools import count
    for i in range(100000,1000000):
        #print(i)
        if len(running)==4:
            print(min(running))
            break

        pf_i=prime_factors(i)
        #debugging:
        if len(running)==3:
            print("almost:", i, pf_i, running)
        if len(pf_i)==4:
            #print("cand", i, pf_i, running)
            running.append(i)
        else:
            running.clear()


@time.timing
def e48():
    tot=0
    for i in range(1,1001):
        tot+= i**i % 10**10
    print(tot % 10**10)

def is_perm_cheat(a,b):
    a=str(a)
    b=str(b)
    for c in a:
        if c not in b:
            return False
    for c in b:
        if c not in a:
            return False
    return True

@time.timing
def e49():
    from itertools import takewhile;
    primes = set(filter(lambda p: p > 1000, takewhile(lambda n: n < 10_000,eratosthenes())))
    for p in primes:
        qset = [q for q in primes if q < p and is_perm_cheat(p,q)]
        for q in qset:
            r = p + (p-q)
            if r < 10_000 and r in primes and is_perm_cheat(p,r):
                print(q,p,r,q*10**8+p*10**4+r)

# Which prime, below one-million, can be written as the sum of the most consecutive primes?
@time.timing
def e50():
    from collections import defaultdict
    # 32 minutes so far
    from itertools import takewhile;
    primes = set(takewhile(lambda n: n < 1_000_000,eratosthenes()))
    plist = list(takewhile(lambda n: n < 500_000,eratosthenes()))

    d = defaultdict(lambda : 0)
    greatest_seen=300
    for (i,p) in enumerate(plist):
        ct=p
        for (seq_len,q) in enumerate(plist[i+1:]):
            ct+=q
            if ct > 1_000_000:
                break
            if ct in primes and seq_len+1 > d[ct]:
                d[ct] = seq_len+1
                #logging
                if seq_len+1 > greatest_seen:
                    greatest_seen=seq_len+1
                    print("p:", p,"ct:",ct, seq_len+1)
    val = max(d,key=d.get)
    print(val,d[val])
