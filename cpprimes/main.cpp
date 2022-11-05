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

int main() {
    std::cout << generate_nth_prime(1000000) << std::endl;
    return 0;
}
