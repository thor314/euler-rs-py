import math
from importlib.machinery import SourceFileLoader
time = SourceFileLoader("main","../../e1-10/src/main.py").load_module()

def is_amicable(a):
    if a % 2 == 0: # 50% optimization for n odd
        get_divisors = lambda n: [i for i in filter(lambda x: n%x==0, range(1,int(n/2+1)))]
    else:
        get_divisors = lambda n: [i for i in filter(lambda x: n%x==0, range(1,int(n/2+1),2))]
    div_a = get_divisors(a)
    divisors_sum = sum(div_a)
    other_divisors = get_divisors(divisors_sum)
    other_sum = sum(other_divisors)
    #print(a,div_a, divisors_sum,other_divisors, other_sum)
    if a == other_sum and a != divisors_sum:
        print(a, divisors_sum)
        return True
    else:
        return False
@time.timing
def e21():
    f = lambda x: (x, is_amicable(x))
    second = lambda tup: tup[1]
    first = lambda tup: tup[0]
    arr = [i for i in map(first,filter(second, map(f,range(1,10000))))]
    #print(arr)
    print(sum(arr))

# For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
# What is the total of all the name scores in the file?
@time.timing
def e22():
    from pathlib import Path
    names = Path('/home/thor/euler/e21-30/src/p022_names.txt').read_text().replace("\"", "").split(',');
    #print(names[:10], names.index("COLIN"))
    names.sort()
    #print(names[:10], names.index("COLIN"))
    s="0ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    score_c = lambda c: s.index(c)
    score_name = lambda pos,name: (1+pos)*sum([score_c(c) for c in name])
    name_scores = [score_name(i,name) for (i,name) in enumerate(names)]
    print(sum(name_scores))

@time.timing
def e23():
    #calculate the divisor_sum of each index
    STOP = 28124
    arr = [0] * STOP
    for i in range(1,STOP):
        for j in range(i*2, STOP,i):
            arr[j] += i
    abundants = list(map(lambda tup: tup[0], filter(lambda tup: tup[0] < tup[1], enumerate(arr))))

    # construct array of whether index i is a sum of 2 abundants
    is_ab_sum = [False] * STOP
    for i in abundants:
        for j in abundants:
            if i + j < STOP:
                is_ab_sum[i+j]=True
            else: # how would I do this `break` in a non-iterative way? I think I'd have to use recursion.
                break # inner j loop
    #print(list(filter(lambda tup: not tup[1], enumerate(is_ab_sum)))[:40])
    print(sum(map(lambda tup: tup[0], filter(lambda tup: not tup[1], enumerate(is_ab_sum)))))

@time.timing
def e24():
    from itertools import permutations;
    s = [i for i in permutations(range(10))]
    print("".join(map(str,s[1_000_000-1])))

#What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
@time.timing
def e25():
    counter_a = 2
    a,b=1,1
    while len(str(a)) < 1000:
        counter_a+=1
        a,b = a+b,a
    print(counter_a)

@time.timing
def e26():
    arr = sorted([get_cycle_lens(i) for i in range(2,1000)], key=lambda x:x[1], reverse=True)
    print(arr[0][0])

def get_cycle_lens(divisor): # -> (n, cycle_len)
    repeat_b = False
    remainder = 10**len(str(divisor-1))
    s = ""
    repeat_b, repeat_ind = False, 0
    while not repeat_b:
        quotient,remainder = remainder // divisor, remainder % divisor
        if remainder==0:
            break
        remainder*=10
        s += str(quotient)
        repeat_b,repeat_ind = check_repeats(s)
    return (divisor,repeat_ind)

def check_repeats(s):
    M_SLICE_LEN = 3
    if len(s) < M_SLICE_LEN +1:
        return False,0
    n_chunks = min(10,len(s) - M_SLICE_LEN)
    for i in range(n_chunks):
        chunk = s[i:i+M_SLICE_LEN]
        last_idx = len(s) - M_SLICE_LEN
        last_chunk = s[last_idx:last_idx+M_SLICE_LEN]
        if chunk == last_chunk:
            return True, last_idx
    return False,0
# This is one I should spend a bit of time on the review. It took me a long time to come up with a not crap algorithm.
# Wow. Dark magic. https://github.com/nayuki/Project-Euler-solutions/blob/e4ea051b715390924114aa35936f21a2ac585144/python/p026.py
def e26_nayuki():
    # usage of "key" here isn't something I knew I could do!
    print(max(range(1,1000), key=reciprocal_cycle_len))
def reciprocal_cycle_len(n):
    import itertools
    seen={}
    x=1
    for i in itertools.count():
        if x in seen:
            print("seen:",x, seen[x])
            return i - seen[x]
        else:
            print("unseen:",x,i)
            seen[x]=i
            x = x*10%n



@time.timing
def e27():
    arr = [((a,b),e27_prime_seq_len(2,a,b,2)) for a in range(-1000,1000) for b in filter(lambda b: is_prime(b) and is_prime(1+a+b), range(max(a,2),1000))]
    arr=sorted(arr, key=lambda x: x[1],reverse=True)
    print(arr[0], arr[0][0][0]*arr[0][0][1])

def e27_prime_seq_len(n,a,b,counter):
    next = n**2 + a*n + b
    if is_prime(next):
        return e27_prime_seq_len(n+1,a,b,counter+1)
    else:
        return counter

def is_prime(n):
    if n < 2:
        return False
    elif n == 2:
        return True
    else:
        if n%2==0:
            return False
        lim=math.ceil(math.sqrt(n)+1)
        for i in range(3,lim,2):
            if n%i==0:
                return False
        return True

@time.timing
def e28():
    s=1001
    v=e28_gen_arr(s)
    row_sums = sum([v[i][i] for i in range(s)])+sum([v[s-1-i][i] for i in range(s)])-v[s//2][s//2]
    print(row_sums)

def e28_gen_arr(size):
    #arr=[[0]*size]*size # Ugh, I can't do this? How to fix?
    arr=[]
    for i in range(size):
        col=[]
        for j in range(size):
            col.append(0)
        arr.append(col)
    i,j=size//2,size//2
    count=1

    while True:
        if arr[i][j]!=0:
            raise ValueError('Already has value at index i:%s j:%s; %s' % (i,j, arr[i][j]))
        arr[i][j]=count
        count+=1

        if (i,j) == (0,size-1):
            return arr
        else:
            if i == j:
                if i <= size//2:
                    dirr="right"
                else:
                    dirr="left"
            elif i<j:
                if i+j >= size:
                    dirr="down"
                else:
                    dirr="right"
            else:
                if i+j>size-1:
                    dirr="left"
                else:
                    dirr="up"
        if dirr=="right":
            j+=1
        elif dirr=="left":
            j-=1
        elif dirr=="up":
            i-=1
        elif dirr=="down":
            i+=1
# Others made a clever sequence observation to avoid constructing the array: https://github.com/nayuki/Project-Euler-solutions/blob/e4ea051b715390924114aa35936f21a2ac585144/python/p028.py
@time.timing
def e29():
    r = set([a**b for a in range(2,101) for b in range(2,101)])
    print(len(r))

@time.timing
def e30():
    # Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
    # note 9**5=59049; therefore n is at most 590490, and probably much less
    results=[i for i in range(2,590_490) if is_fifth_pow_sum(i)]
    print(sum(results),results)

def is_fifth_pow_sum(n):
    digit_v = digits(n)
    summ=sum([i**5 for i in digit_v])
    return summ==n

def digits(n):
    arr=[]
    for d in str(n):
        arr.append(int(d))
    return arr
