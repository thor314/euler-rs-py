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

@timing
def e2():
    init = (1,1)
    sum = 0
    while init[0] < 4000000:
        if init[0] % 2 == 0:
            sum += init[0]
        init = (init[0] + init[1], init[0])
    print(sum)



def main():
    e1()
    e2()

if __name__ == "__main__":
    main()
