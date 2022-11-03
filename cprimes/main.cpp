#include <iostream>
#include <vector>
#include <math.h>

int generate_nth_prime(int n) {
    std::vector<int> primes;

    for (int i = 2; primes.size() < n; i++) {
        bool is_prime = true;

        for (int j = 0; j < primes.size(); j++) {
            if (i % primes[j] == 0) {
                is_prime = false;
                break;
            }

            if (primes[j] > sqrt(i)) {
                break;
            }
        }

        if (is_prime) {
            primes.push_back(i);
        }
    }
    
    return primes[n - 1];
}

// Doesn't work, is also slower
int generate_nth_prime2(int n) {
    n++;

    int num_gen;
    int i = 2;
    int last_prime = 0;

    while(num_gen < n) {
        bool is_prime = true;

        for (int j = 2; j <= sqrt(i); j++) {
            if (i % j == 0) {
                is_prime = false;
                break;
            }
        }

        if (is_prime) {
            num_gen++;
            last_prime = i;
        }

        i++;
    }
    
    return last_prime;
}

int main() {
    std::cout << generate_nth_prime(1000000) << std::endl;
    return 0;
}
