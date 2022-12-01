use std::env;
use std::thread;
use std::sync::mpsc;

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

fn test_sender_thread(tx: mpsc::Sender<u32>) {
    for i in 0..10 {
        tx.send(i).unwrap();
        thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args.len() > 3 {
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
        let prime = multithread::gen_nth_prime(n as u64, 4);
        println!("{}", prime);
    } else {
        println!("{}", gen_nth_prime(n));
    }
}
