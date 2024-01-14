use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Error}, cmp::min,
};

const NUMBER_OF_ROWS: usize = 192;

pub fn main() -> Result<(), Error> {
    let path = "input4.txt";
    // let path = "test4.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut points = [0; NUMBER_OF_ROWS];
    for (idx, line) in buffered.lines().enumerate() {
        let line = line.unwrap();
        let content = line.split(": ").nth(1).unwrap();
        let parts: Vec<&str> = content.split(" | ").collect();
        let winning = parts[0]
            .split_ascii_whitespace()
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();
        let hands = parts[1]
            .split_ascii_whitespace()
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut t = 0;
        for e in &hands {
            if winning.contains(e) {
                // t <<= 1;
                t += 1
            }
        }
        // t >>= 1;
        points[idx as usize] = t;
        // ans += t;
        // println!("{line}: {winning:?} {hands:?} {t}");
    }
    println!("{:?}", points);
    let mut t = [0; NUMBER_OF_ROWS + 1];
    for i in 0..NUMBER_OF_ROWS {
        if i > 1 {
            t[i] += t[i - 1];
        }
        if points[i] == 0 {
            continue;
        }
        if i != NUMBER_OF_ROWS - 1 {
            t[i + 1] += 1 + t[i];
        }
        if i + points[i] + 1 <= NUMBER_OF_ROWS - 1 {
            t[i + points[i] + 1] -= 1 + t[i];
        }
    }
    println!("{:?}", &t);
    // for i in 1..NUMBER_OF_ROWS {
    //     t[i] += t[i - 1]
    // }
    // println!("{t:?}");

    println!("Answer is: {}", t.iter().sum::<i32>() + NUMBER_OF_ROWS as i32);
    Ok(())
}

// fn f(points: &[i32; NUMBER_OF_ROWS], i: usize) -> i32 {
//     if i >= NUMBER_OF_ROWS {
//         return 0;
//     }
//     println!("{}", i);
//     let mut ans: i32 = points[i];
//     for j in 0..points[i] as usize {
//         ans += f(points, i + j + 1);
//     }
//     ans
// }