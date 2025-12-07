use std::env;
use std::process;

/// Modular multiplication (a * b) % m using u128 to avoid overflow.
fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// Modular exponentiation base^exp % modu.
fn mod_pow(mut base: u64, mut exp: u64, modu: u64) -> u64 {
    let mut result = 1_u64 % modu;
    base %= modu;

    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modu);
        }
        base = mod_mul(base, base, modu);
        exp >>= 1;
    }

    result
}

/// Deterministic Miller–Rabin primality test for 64 bit integers.
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    // Small primes first, also filters obvious composites.
    const SMALL_PRIMES: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];
    for &p in &SMALL_PRIMES {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return n == p;
        }
    }

    // Write n − 1 as d * 2^s with d odd.
    let mut d = n - 1;
    let mut s = 0_u32;
    while d % 2 == 0 {
        d >>= 1;
        s += 1;
    }

    // Deterministic bases for testing all 64 bit integers.
    // Source: research on minimal base sets for 2^64
    const BASES: [u64; 7] = [
        2,
        325,
        9375,
        28178,
        450775,
        9780504,
        1795265022,
    ];

    'outer: for &a in &BASES {
        if a % n == 0 {
            continue; // Skip if a is a multiple of n
        }
        let mut x = mod_pow(a % n, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }

    true
}

/// Find the largest prime strictly less than n.
/// Returns None if there is no such prime.
fn largest_prime_below(n: u64) -> Option<u64> {
    if n <= 2 {
        return None;
    }

    let mut candidate = n - 1;

    // Ensure we start on an odd number, unless candidate is exactly 2.
    if candidate > 2 && candidate % 2 == 0 {
        candidate -= 1;
    }

    loop {
        if is_prime(candidate) {
            return Some(candidate);
        }

        if candidate <= 3 {
            break;
        }

        // Skip even numbers.
        candidate = candidate.saturating_sub(2);
    }

    if candidate >= 2 && is_prime(candidate) {
        Some(candidate)
    } else {
        None
    }
}

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: largest_prime_below <number>");
        process::exit(1);
    });

    let n: u64 = arg.parse().unwrap_or_else(|_| {
        eprintln!("Error: '{}' is not a valid unsigned integer", arg);
        process::exit(1);
    });

    match largest_prime_below(n) {
        Some(p) => println!("Largest prime less than {} is {}", n, p),
        None => println!("There is no prime less than {}", n),
    }
}
