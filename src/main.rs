use regex::Regex;
use clap::{Parser, command};

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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t=100000)]
    maximum: u64,
}

fn main() {

    let args = Args::parse();
    
    let maximum: u64 = args.maximum;
    let is_primo_adri = Regex::new(r"[2,3,5,7]{4}7").unwrap();

    let primes = primes(maximum);

    let primos_adri: Vec<_> = primes
        .iter()
        .filter(|p| is_primo_adri.is_match(&p.to_string()))
        .collect();

    println!(
        "{} Primos de Adri hasta {}:\n{:?}",
        primos_adri.len(),
        maximum,
        primos_adri
    );
}
