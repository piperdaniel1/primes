#include <stdlib.h>
#include <stdio.h>
#include "da_void.h"

int main(int argc, char **argv) {
    if (argc != 2) {
        printf("Usage: %s <nth prime to find>\n", argv[0]);
        return 1;
    }

    int num_primes = atoi(argv[1]);

    if (num_primes < 1) {
        printf("<nth prime> must be greater than 0\n");
        return 1;
    }

    struct Void_Ary * primes = ary_void_create();
    int* prime = malloc(sizeof(int));
    *prime = 2;
    ary_void_push_back(primes, prime);

    int i = 3;
    while (ary_void_get_length(primes) < num_primes) {
        for (int j=0; j<ary_void_get_length(primes); j++) {
            int prime = *(int *)ary_void_get_index(primes, j);

            if (i % prime == 0) {
                break;
            }

            if (prime*prime > i) {
                int* new_prime = malloc(sizeof(int));
                *new_prime = i;

                ary_void_push_back(primes, (void*) new_prime);

                break;
            }
        }

        i++;
    }

    int last_prime = *(int *)ary_void_get_index(primes, ary_void_get_length(primes) - 1);
    printf("The %dth prime is %d\n", num_primes, last_prime);

    for (int i=0; i<ary_void_get_length(primes); i++) {
        free(ary_void_get_index(primes, i));
    }

    ary_void_free(primes);

    return 0;
}
