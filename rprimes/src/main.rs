use std::env;

fn gen_nth_prime(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut i = 3;

    while primes.len() < n as usize {
        for p in &primes {
            if i % p == 0 {
                break;
            }
            if p * p > i {
                primes.push(i);
                break;
            }
        }
        i += 1;
    }
    primes[n as usize - 1]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u32>().unwrap();

    println!("{}", gen_nth_prime(n));
}
