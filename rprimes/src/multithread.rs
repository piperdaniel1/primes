use std::thread;
use std::sync::mpsc;

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

fn generator_thread(starting_num: u64, add_amount: u64, tx: mpsc::Sender<u64>, rx: mpsc::Receiver<u64>, rx_max_num: mpsc::Receiver<u64>) {
    let mut primes = vec![2];
    let mut i = starting_num;
    let mut max_num: u64 = 4;
    let thread_num = (starting_num - 3) / 2;

    println!("[gen {}] starting at {}", thread_num, starting_num);

    loop {
        // Check a prime
        for p in &primes {
            if i % p == 0 {
                break;
            }
            if p * p > i {
                primes.push(i);

                // Send the new prime to the main thread
                tx.send(i).unwrap();

                println!("[gen {}] found and sent prime {}", thread_num, i);
                break;
            }
        }

        i += add_amount;
        println!("[gen {}] updated i to {}", thread_num, i);

        // Receive all of the max numbers from the main thread
        loop {
            let recv = match rx_max_num.try_recv() {
                Ok(p) => p,
                Err(_) => {
                    if i <= max_num {
                        break;
                    } else {
                        // Hopefully the main thread will send a max number soon
                        thread::sleep(std::time::Duration::from_millis(100));
                        continue;
                    }
                },
            };

            // Update the max num
            if recv > max_num {
                println!("[gen {}] updated max num to {}", thread_num, i);
                max_num = recv;
            }
        }

        // Receive all of the new primes from the main thread
        loop {
            let recv = match rx.try_recv() {
                Ok(p) => p,
                Err(_) => break,
            };

            // Main thread wants us to stop
            if recv == 0 {
                println!("[gen {}] received exit code...", thread_num);
                return;
            }

            // We continue by adding the new prime to the number we are checking
            let ind = primes.binary_search(&recv);
            primes.insert(ind.unwrap_or_else(|x| x), recv);
            println!("[gen {}] added remotely found prime {} to list", thread_num, recv);
        }
    }
}

pub fn gen_nth_prime(n: u64, num_threads: u64) -> u32 {
    // Create channels for each thread to send the primes they find to the main thread
    // We keep the rx_prime_channels in this thread and give the tx_prime_channels to the
    // generator threads so they can send the primes they find back to us
    let mut rx_prime_channels: Vec<mpsc::Receiver<u64>> = Vec::new();

    // Create channels for each thread to receive the primes the other threads find.
    // We keep the tx_prime_channels in this thread and give the rx_prime_channels to the
    // generator threads so they can receive the primes the other threads find
    let mut tx_sync_channels: Vec<mpsc::Sender<u64>> = Vec::new();

    // Create channels for each thread to receive the maximum number they are allowed to check
    // to at this time. We keep the tx_max_num_channels in this thread and give the rx_max_num_channels
    // to the generator threads so they can receive the maximum number they are allowed to check.
    // This allows us to keep the threads in sync so that they don't lag behind each other.
    let mut tx_max_num_channels: Vec<mpsc::Sender<u64>> = Vec::new();

    // Calculate starting nums and add amounts
    let mut starting_nums: Vec<u64> = Vec::new();
    let mut add_amounts: Vec<u64> = Vec::new();

    for i in 0..num_threads {
        starting_nums.push(3 + i * 2);
        add_amounts.push(num_threads * 2);
    }

    // Keep track of the primes we have found
    let mut primes = vec![2];
    let mut max_nums_recieved: Vec<u64> = vec![0; num_threads as usize];
    let mut max_num = 4;

    // Spawn all of the threads and give them the channels they need
    // Keep the channels we need to communicate with the threads
    for i in 0..num_threads {
        let (tx_prime, rx_prime) = mpsc::channel::<u64>();
        let (tx_sync, rx_sync) = mpsc::channel::<u64>();
        let (tx_max_num, rx_max_num) = mpsc::channel::<u64>();

        rx_prime_channels.push(rx_prime);
        tx_sync_channels.push(tx_sync);
        tx_max_num_channels.push(tx_max_num);

        println!("[main] Starting thread {}", i);

        thread::Builder::new().name(i.to_string()).spawn(move || {
            generator_thread(2*i+3, num_threads*2, tx_prime, rx_sync, rx_max_num);
        }).unwrap();
    }

    loop {
        // Receive all of the primes from the threads and add them to our list of primes
        // Also send the new primes to the other threads
        for i in 0..num_threads {
            loop {
                let recv = match rx_prime_channels[i as usize].try_recv() {
                    Ok(p) => p,
                    Err(_) => break,
                };

                // We continue by adding the new prime to the number we are checking
                let ind = primes.binary_search(&recv);
                primes.insert(ind.unwrap_or_else(|x| x), recv);

                println!("[main] Received prime {} from thread {}", recv, i);

                // Update the max num for that thread
                if recv > max_nums_recieved[i as usize] {
                    max_nums_recieved[i as usize] = recv;
                }

                // Send the new prime to the other threads (besides thread i)
                for j in 0..num_threads {
                    if i != j {
                        println!("[main] Sending prime {} to thread {}", recv, j);
                        tx_sync_channels[j as usize].send(recv).unwrap();
                    }
                }
            }
        }

        // Get the min max num recieved
        let mut max_num_recieved = 0;

        for i in 0..num_threads {
            if max_nums_recieved[i as usize] > max_num_recieved || max_num_recieved == 0 {
                max_num_recieved = max_nums_recieved[i as usize]
            }
        }

        // Update the threads
        if max_num_recieved*max_num_recieved > max_num {
            max_num = max_num_recieved*max_num_recieved;
            println!("[main] Sending new max num {} to all threads", max_num);
            // Send the max number to the threads
            for i in 0..num_threads {
                tx_max_num_channels[i as usize].send(max_num).unwrap();
            }
        }

        // Check if we have found enough primes
        if primes.len() as u64 >= n {
            println!("[main] We found enough primes, exiting...");
            break;
        }
    }

    return primes[n as usize - 1] as u32;
}
