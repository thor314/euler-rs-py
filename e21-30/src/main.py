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
        #print("AHTNEHOUEHUT")
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
   r eturn (divisor,repeat_ind)

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
