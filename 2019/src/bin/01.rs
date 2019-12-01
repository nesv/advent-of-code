use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let in_file: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("Specify a path to the input file");
            std::process::exit(1);
        }
    };
    let file = File::open(in_file)?;
    let mut fuel_per_component: i32 = 0;
    let mut total_fuel_required: i32 = 0;
    for line in BufReader::new(file).lines() {
        let mass: i32 = line.unwrap().parse().unwrap();
        fuel_per_component += calc_fuel(mass);
        total_fuel_required += calc_fuel_weight(mass);
    }
    println!("Fuel for components: {}", fuel_per_component);
    println!("Total fuel required: {}", total_fuel_required);
    Ok(())
}

fn calc_fuel(n: i32) -> i32 {
    match n {
        0 => 0,
        _ => (n / 3) - 2,
    }
}

#[test]
fn test_calc_fuel() {
    assert_eq!(calc_fuel(12), 2);
    assert_eq!(calc_fuel(14), 2);
    assert_eq!(calc_fuel(1969), 654);
    assert_eq!(calc_fuel(100756), 33583);
}

fn calc_fuel_weight(n: i32) -> i32 {
    let w = calc_fuel(n);
    if w <= 0 {
        return 0;
    }
    return w + calc_fuel_weight(w);
}

#[test]
fn test_calc_fuel_weight() {
    assert_eq!(calc_fuel_weight(14), 2);
    assert_eq!(calc_fuel_weight(1969), 966);
    assert_eq!(calc_fuel_weight(100756), 50346);
}
