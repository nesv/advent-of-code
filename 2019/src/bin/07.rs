use aoc::{
    Input,
    intcode::Program,
};
use rand::{
    seq::SliceRandom,
    thread_rng,
};
use std::collections::HashMap;
use std::io::Result;
use std::iter::FromIterator;

fn main() -> Result<()> {
    let inpath: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => { panic!("no input file provided"); },
    };
    let input = Input::from_file(inpath).expect("failed to load input file");
    let code = input.to_string().expect("failed to load intcode");

    let (sequence, signal) = part1(&code)?;
    println!("{} => {}", sequence, signal);

    let (sequence, signal) = part2(&code)?;
    println!("{} => {}", sequence, signal);

    Ok(())
}

fn part1(code: &str) -> Result<(i32, i64)> {
    let sequence: Vec<i64> = vec![0, 1, 2, 3, 4];

    // Store the attempted sequences, and their results.
    let mut m: HashMap<i32, i64> = HashMap::new();
    loop {
        // There are 5! => 120 possible combinations for the input.
        // Leave once we have found all of them.
        if m.len() == 120 {
            break;
        }

        // Generate a random sequence.
        let seq: Vec<i64> = sequence
            .choose_multiple(&mut thread_rng(), 5)
            .map(|&n| n)
            .collect();

        // If we have already seen this sequence, skip it.
        let kvs: Vec<String> = seq.iter().map(|&n| n.to_string()).collect();
        let key: i32 = String::from_iter(kvs).as_str().parse().expect("failed to parse sequence as i32");
        if m.contains_key(&key) {
            continue;
        }

        // Amplifiers.
        let mut a = Program::from(code);
        a.input(seq[0]);
        a.input(0);
        let out = a.execute()?;
        let signal: i64 = out[0];

        let mut b = Program::from(code);
        b.input(seq[1]);
        b.input(signal);
        let out = b.execute()?;
        let signal: i64 = out[0];

        let mut c = Program::from(code);
        c.input(seq[2]);
        c.input(signal);
        let out = c.execute()?;
        let signal: i64 = out[0];

        let mut d = Program::from(code);
        d.input(seq[3]);
        d.input(signal);
        let out = d.execute()?;
        let signal: i64 = out[0];

        let mut e = Program::from(code);
        e.input(seq[4]);
        e.input(signal);
        let out = e.execute()?;
        let signal: i64 = out[0];

        m.insert(key, signal);
    }

    // Find the largest output.
    let mut sequence = 0;
    let mut max = 0;
    for (&seq, &n) in &m {
        if n > max {
            max = n;
            sequence = seq;
        }
    }

    Ok((sequence, max))
}

fn part2(code: &str) -> Result<(i32, i64)> {
    let sequence: Vec<i64> = vec![5, 6, 7, 8, 9];

    // Store the attempted sequences, and their results.
    let mut m: HashMap<i32, i64> = HashMap::new();
    loop {
        // There are 5! => 120 possible combinations for the input.
        // Leave once we have found all of them.
        if m.len() == 120 {
            break;
        }

        // Generate a random sequence.
        let seq: Vec<i64> = sequence
            .choose_multiple(&mut thread_rng(), 5)
            .map(|&n| n)
            .collect();

        // If we have already seen this sequence, skip it.
        let kvs: Vec<String> = seq.iter().map(|&n| n.to_string()).collect();
        let key: i32 = String::from_iter(kvs).as_str().parse().expect("failed to parse sequence as i32");
        if m.contains_key(&key) {
            continue;
        }

        eprintln!("{:?}", &seq);

        // Amplifiers.
        let mut a = Program::from(code);
        let mut b = Program::from(code);
        let mut c = Program::from(code);
        let mut d = Program::from(code);
        let mut e = Program::from(code);

        let mut max_signal: i64 = 0;
        let mut inputs: Vec<i64> = vec![0];
        loop {
            a.input(seq[0]);
            for i in inputs {
                a.input(i);
            }
            let out = a.execute()?;
            dbg!(&out);

            b.input(seq[1]);
            for i in out {
                b.input(i);
            }
            let out = b.execute()?;
            dbg!(&out);

            c.input(seq[2]);
            for i in out {
                c.input(i);
            }
            let out = c.execute()?;
            dbg!(&out);

            d.input(seq[3]);
            for i in out {
                d.input(i);
            }
            let out = d.execute()?;
            dbg!(&out);

            e.input(seq[4]);
            for i in out {
                e.input(i);
            }
            let out = e.execute()?;
            dbg!(&out);

            let signal: i64 = out[out.len()-1];
            if signal > max_signal {
                max_signal = signal;
            } else if signal == max_signal {
                break;
            }

            inputs = out;
        }

        m.insert(key, max_signal);
    }

    // Find the largest output.
    let mut sequence = 0;
    let mut max = 0;
    for (&seq, &n) in &m {
        if n > max {
            max = n;
            sequence = seq;
        }
    }

    Ok((sequence, max))
}

#[test]
fn test_part2_18216() {
    let code = String::from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    let (_, signal) = part2(&code).unwrap();
    assert_eq!(signal, 18216);
}


#[test]
fn test_part2_139629729() {
    let code = String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    let (_, signal) = part2(&code).unwrap();
    assert_eq!(signal, 139629729);
}
