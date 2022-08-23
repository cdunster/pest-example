use pest_example::sum_csv_values;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let mut file = File::open(opt.path).expect("Cannot open file");
    let sum = sum_csv_values(&mut file).expect("Failed to parse file");
    println!("Sum: {}", sum);
}
