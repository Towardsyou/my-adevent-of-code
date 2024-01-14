use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub fn main() -> Result<(), Error> {
    let path = "input2.txt";
    // let path = "test2.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut ans = 0;
    let mut ans2 = 0;
    for (idx, line) in buffered.lines().enumerate() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let line = line.unwrap();
        let content = line.split(": ").nth(1).expect("No '\"' in the row");
        let combinations = content.split("; ");
        for c in combinations {
            for e in c.split(", ") {
                if e.ends_with("green") {
                    let t = e
                        .trim()
                        .split_ascii_whitespace()
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    green = max(green, t);
                } else if e.ends_with("red") {
                    let t = e
                        .trim()
                        .split_ascii_whitespace()
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    red = max(red, t);
                } else if e.ends_with("blue") {
                    let t = e
                        .trim()
                        .split_ascii_whitespace()
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    blue = max(blue, t);
                }
            }
        }
        println!("{line}\n  -> green: {green}, red: {red}, blue: {blue}");
        if red <= 12 && green <= 13 && blue <= 14 {
            ans += idx as i32 + 1
        }
        ans2 += green * red * blue;
    }

    println!("{ans}, {ans2}");

    Ok(())
}
