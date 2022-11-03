import math
import sys
import subprocess

def gen_nth_prime(n):
    n -= 1
    primes = []
    curr_num = 2

    while len(primes) < n:
        for j in range(2, curr_num):
            if curr_num % j == 0:
                break

            if j >= math.sqrt(curr_num):
                primes.append(curr_num)
                break

        curr_num += 1

    return primes[-1]

# lmao such python much speed
def fast_gen_nth_prime(n):
    args = ("./rprimes", str(n))
    popen = subprocess.Popen(args, stdout=subprocess.PIPE)
    popen.wait()

    output = popen.stdout
    assert output is not None
    return int(output.read())

def main():
    n = int(sys.argv[1])
    if len(sys.argv) > 2:
        if sys.argv[2] == "slow":
            print(gen_nth_prime(n))
        else:
            print(fast_gen_nth_prime(n))
    else:
        print(fast_gen_nth_prime(n))


if __name__ == '__main__':
    main()
