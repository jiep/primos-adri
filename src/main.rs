use clap::{command, Parser};
use primos_adri::get_primos_adri;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 5)]
    #[arg(value_parser = clap::value_parser!(u8).range(2..))]
    digits: u8,
}

fn main() {
    let args = Args::parse();

    let maximum = 10_u64.pow(args.digits as u32);

    let primos_adri = get_primos_adri(args.digits);

    let number_of_primos_adri = primos_adri.len();
    let primos_adri = primos_adri
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    println!(
        "{} Primos de Adri up to {}:\n{}",
        number_of_primos_adri, maximum, primos_adri
    );
}
