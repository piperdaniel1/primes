use std::env;

mod multithread;

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

    if args.len() == 1 || args.len() > 2 {
        println!("Usage: ./primes <n> [multithread]");
        return;
    }

    let n = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Usage: ./primes <n> [multithread]");
            return;
        }
    };

    if args.len() > 2 && args[2] == "multithread" {
        println!("Sorry, multithreading is not implemented yet.");
    } else {
        println!("{}", gen_nth_prime(n));
    }
}
