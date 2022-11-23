use regex::Regex;

fn primes(maximum: u64) -> Vec<u64> {
    let mut primes = vec![2];

    for candidate in 3..maximum {
        let square_root = (candidate as f64).sqrt() as u64 + 1;
        let is_prime = primes
            .iter()
            .take_while(|p| p <= &&square_root)
            .all(|p| candidate % p != 0);
        if is_prime {
            primes.push(candidate);
        }
    }

    primes
}

pub fn get_primos_adri(digits: u8) -> Vec<u64> {
    let maximum: u64 = 10_u64.pow(digits.into());
    let is_primo_adri = Regex::new(format!(r"^[2,3,5,7]{{{}}}7$", digits - 1).as_str()).unwrap();

    let primos_adri: Vec<_> = primes(maximum)
        .iter()
        .filter(|p| is_primo_adri.is_match(&p.to_string()))
        .copied()
        .collect::<Vec<u64>>();

    primos_adri
}
