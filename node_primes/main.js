function gen_nth_prime(n) {
  let primes = [2];
  let i = 3;

  while (primes.length < n) {
    for (let j = 0; j < primes.length; j++) {
      if (i % primes[j] == 0) {
        break;
      }

      if (primes[j] * primes[j] > i) {
        primes.push(i);
        break;
      }
    }

    i += 1;
  }

  return primes[primes.length - 1];
}

console.log(gen_nth_prime(1000000)); // 7
