package main

import (
	"fmt"
	"math"
)

func get_nth_prime(n int) int {
    n += 1
    // Create primes array with first two primes
    primes := []int{2, 3}

    var i int

    for i = 2; len(primes) < n; i++ {
        var j int

        is_prime := true
        for j = 0; j < len(primes); j++ {
            if i % primes[j] == 0 {
                is_prime = false
                break
            }

            if primes[j] >= int(math.Ceil(math.Sqrt(float64(i))))  {
                break
            }
        }

        if is_prime {
            primes = append(primes, i)
        }
    }
    return primes[n-1]
}

func main() {
    fmt.Println(get_nth_prime(1000000))
}
