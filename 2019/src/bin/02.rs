use std::io::Result;

fn main() -> Result<()> {
    let in_file: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("Specify a path to the input file");
            std::process::exit(1);
        }
    };
    let input: String = std::fs::read_to_string(in_file)?;
    let mem: Vec<usize> = input.trim_end().split(",").map(|i| match i.parse::<usize>() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}: {}", err, i);
            std::process::exit(1);
        },
    }).collect();

    // Part 1.
    let mut p1 = mem.clone();
    p1[1] = 12;
    p1[2] = 2;
    let after = compute(p1);
    println!("{}", after[0]);

    // Part 2.
    let want = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = mem.clone();
            mem[1] = noun;
            mem[2] = verb;
            if compute(mem)[0] == want {
                let result = (100 * noun) + verb;
                println!("{}", result);
                return Ok(());
            }
        }
    }

    Ok(())
}

fn compute(mem: Vec<usize>) -> Vec<usize> {
    let mut mem = mem.clone();
    let mut i: usize = 0;
    loop {
        // mem[i] == instruction pointer
        match mem[i] {
            // Addition.
            1 => {
                let noun = mem[i+1];
                let verb = mem[i+2];
                let output = mem[i+3];
                mem[output] = mem[noun] + mem[verb];
            },

            // Multiplication.
            2 => {
                let noun = mem[i+1];
                let verb = mem[i+2];
                let output = mem[i+3];
                mem[output] = mem[noun] * mem[verb];
            },

            // HCF.
            99 => { break; },

            _ => { eprintln!("Unexpected opcode at ip={}: {}", i, mem[i]); }
        }

        if i + 4 >= mem.len() {
            break;
        }
        i += 4;
    }
    mem
}

#[test]
fn test_compute() {
    let v = compute(vec![1, 0, 0, 0, 99]);
    assert_eq!(v[0], 2);

    let v = compute(vec![2, 3, 0, 3, 99]);
    assert_eq!(v[3], 6);

    let v = compute(vec![2, 4, 4, 5, 99, 0]);
    assert_eq!(v[5], 9801);

    let v = compute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
    assert_eq!(v[0], 30);
    assert_eq!(v[4], 2);
}
