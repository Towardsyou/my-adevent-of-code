use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn main() -> Result<(), Error> {
    let path = "input1.txt";
    let mut res = 0;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map.insert("0", 0);
    map.insert("1", 1);
    map.insert("2", 2);
    map.insert("3", 3);
    map.insert("4", 4);
    map.insert("5", 5);
    map.insert("6", 6);
    map.insert("7", 7);
    map.insert("8", 8);
    map.insert("9", 9);

    for line in buffered.lines() {
        let line = line.unwrap();

        let mut l = 0x3f3f3f3f;
        let mut lv = 0;
        let mut r = 0;
        let mut rv = 0;

        for (key, value) in &map {
            if let Some(idx) = line.find(key) {
                if idx < l {
                    l = idx;
                    lv = *value;
                }
            }
            if let Some(idx) = line.rfind(key) {
                if idx >= r {
                    r = idx;
                    rv = *value;
                }
            }
        }
        println!("{line}: {}", lv * 10 + rv);
        res += lv * 10 + rv;
    }

    println!("{}", res);

    Ok(())
}