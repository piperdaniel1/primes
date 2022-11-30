// Idea: Multithread the process of generating the primes
// I guess this could work by each thread generating every nth prime.

// So for four threads:
// 3 5 7 9 11 13 15 17 19 21 23 25 
// Thread one: check 3 11 19 (+8 each time)
// Thread two: check 5 13 21 (+8 each time)
// Thread three: check 7 15 23 (+8 each time)
// Thread four: check 9 17 25 (+8 each time)

// So for five threads:
// 3 5 7 9 11 13 15 17 19 21 23 25 27 29 31
// Thread one: check 3 13 23 (+10 each time)
// Thread two: check 5 15 25 (+10 each time)
// Thread three: check 7 17 27 (+10 each time)
// Thread four: check 9 19 29 (+10 each time)
// Thread five: check 11 21 31 (+10 each time)

// We need some way to broadcast the primes to the other threads so that they can use them
// to check the next number. It seems like we should channel all the primes back to the main
// thread, and then have the main thread broadcast them to the other threads.

// If one of the threads is lagging behind a bit the other threads will wait until it catches up 
// Lagging behind is defined as one thread trying to check a number that is more than the square
// of the largest prime that another thread has found. In practice it is possible the threads
// won't lag behind much at all due to the square.