/// Find the largest prime strictly less than n.
/// Returns None if there is no such prime.
pub fn largest_prime_below(n: u64) -> Option<u64> {
    if n <= 2 {
        return None;
    }

    let mut candidate = n - 1;

    // Ensure we start on an odd number, unless candidate is exactly 2.
    if candidate > 2 && candidate.is_multiple_of(2) {
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
        if n.is_multiple_of(p) {
            return n == p;
        }
    }

    // Write n − 1 as d * 2^s with d odd.
    let mut d = n - 1;
    let mut s = 0_u32;
    while d.is_multiple_of(2) {
        d >>= 1;
        s += 1;
    }

    // Deterministic bases for testing all 64 bit integers.
    // Source: research on minimal base sets for 2^64
    const BASES: [u64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_small_primes() {
        let primes = [2_u64, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for &p in &primes {
            assert!(is_prime(p), "expected {p} to be prime");
        }
    }

    #[test]
    // technically 1 isn't composite or prime, but hey ho
    fn is_prime_small_composites() {
        let composites = [0_u64, 1, 4, 6, 8, 9, 10, 12, 15, 21, 25, 100];
        for &c in &composites {
            assert!(!is_prime(c), "expected {c} to be composite");
        }
    }

    #[test]
    fn is_prime_large_known_values() {
        // Some larger values to exercise Miller–Rabin paths.
        assert!(is_prime(1_000_000_007));
        assert!(!is_prime(1_000_000_008));
        assert!(is_prime(4_294_967_291)); // near 2^32
    }

    #[test]
    fn largest_prime_below_basic_cases() {
        assert_eq!(largest_prime_below(3), Some(2));
        assert_eq!(largest_prime_below(4), Some(3));
        assert_eq!(largest_prime_below(10), Some(7));
        assert_eq!(largest_prime_below(11), Some(7));
        assert_eq!(largest_prime_below(12), Some(11));
    }

    #[test]
    fn largest_prime_below_no_prime() {
        assert_eq!(largest_prime_below(0), None);
        assert_eq!(largest_prime_below(1), None);
        assert_eq!(largest_prime_below(2), None);
    }

    #[test]
    fn largest_prime_below_even_and_odd_inputs() {
        // Even input
        assert_eq!(largest_prime_below(100), Some(97));
        // Odd input that is prime itself
        assert_eq!(largest_prime_below(101), Some(97));
        // Odd input that is composite
        assert_eq!(largest_prime_below(1001), Some(997));
    }

    #[test]
    fn largest_prime_below_large() {
        // Not astronomic, but large enough to stress test a bit.
        // 1_000_000_009 is prime, so the largest prime below 1_000_000_010 is 1_000_000_009.
        assert_eq!(largest_prime_below(1_000_000_010), Some(1_000_000_009));
    }
}
