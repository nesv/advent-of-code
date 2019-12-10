use aoc::Input;
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let in_file: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "no input file specified",
            ));
        }
    };
    let input = Input::from_file(in_file)?.to_string().expect("no input");
    let input: Vec<u32> = input
        .as_str()
        .chars()
        .map(|c| {
            let n: u32 = c.to_digit(10).unwrap();
            n
        })
        .collect();

    // Size of the image (x * y).
    let img_size: usize = 25 * 6;

    let mut layers: Vec<Vec<u32>> = vec![];
    let num_layers: usize = input.len() / img_size;
    for i in 0..num_layers {
        let mut layer: Vec<u32> = vec![];
        for j in 0..img_size {
            let k = (img_size * i) + j;
            layer.push(input[k]);
        }
        layers.push(layer);
    }

    // Find the layer with the fewest number of 0 digits, and then multiply
    // the number of 1 digits in that layer, with the number of 2 digits
    // in that layer.
    let mut num_zeroes: usize = usize::max_value();
    let mut layer_idx: usize = usize::max_value();
    for (i, layer) in layers.iter().enumerate() {
        let n = layer.iter().filter(|&&d| d == 0).count();
        if n < num_zeroes {
            layer_idx = i;
            num_zeroes = n;
        }
    }

    let ones = layers[layer_idx].iter().filter(|&&d| d == 1).count();
    let twos = layers[layer_idx].iter().filter(|&&d| d == 2).count();

    println!("{}", ones * twos);

    // Decode the image.
    //
    // 0 = black
    // 1 = white
    // 2 = transparent
    //
    // layer[0] is in front, layer[layers.len()-1] is in the back.
    let mut img: Vec<Vec<Vec<u32>>> = vec![];
    for layer in layers {
        let mut rows: Vec<Vec<u32>> = vec![];
        for y in 0..6 {
            let mut row: Vec<u32> = vec![];
            for x in 0..25 {
                let k = (25 * y) + x;
                row.push(layer[k]);
            }
            rows.push(row);
        }
        img.push(rows);
    }

    let mut final_img: Vec<Vec<u32>> = img[0].clone();
    for i in 1..img.len() {
        let layer = &img[i];
        for (y, row) in layer.iter().enumerate() {
            for (x, colour) in row.iter().enumerate() {
                if final_img[y][x] == 2 {
                    final_img[y][x] = *colour;
                }
            }
        }
    }

    for y in final_img {
        for x in y {
            if x == 1 {
                print!("\u{2588}");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    Ok(())
}
