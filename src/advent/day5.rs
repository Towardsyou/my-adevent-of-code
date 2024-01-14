use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader, Error, Read},
};

// pub fn main() -> Result<(), Error> {
//     let path = "input5.txt";
//     // let path = "test5.txt";

//     let mut input = File::open(path)?;
//     let mut buf = String::new();
//     input.read_to_string(&mut buf).unwrap();

//     let content = buf.split("\n\n").collect::<Vec<&str>>();
//     assert!(content.len() == 8, "not enough lines");

//     let seeds = content[0]
//         .split(" ")
//         .into_iter()
//         .skip(1)
//         .map(|s| s.parse::<u32>().unwrap())
//         .collect::<Vec<u32>>();
//     let seed2soil = part2map(content[1]);
//     let soil2fertilizer = part2map(content[2]);
//     let fertilizer2water = part2map(content[3]);
//     let water2light = part2map(content[4]);
//     let light2temperature = part2map(content[5]);
//     let temperature2humidity = part2map(content[6]);
//     let humidity2location = part2map(content[7]);

//     let mut ans = 0x3f3f3f3f;
//     for seed in &seeds {
//         let soil = seed2soil.get(seed);
//         let fertilizer = soil2fertilizer.get(&soil);
//         let water = fertilizer2water.get(&fertilizer);
//         let light = water2light.get(&water);
//         let temperature = light2temperature.get(&light);
//         let humidity = temperature2humidity.get(&temperature);
//         let location = humidity2location.get(&humidity);
//         println!(
//             "Ans for {seed} is {location}",
//             seed = seed,
//             location = location
//         );
//         ans = min(ans, location);
//     }

//     // println!("seeds: {seeds:?}");
//     // println!("seed2soil: {seed2soil:?}");
//     println!("ans: {}", ans);

//     Ok(())
// }

// fn part2map(rows: &str) -> Map<u32, u64> {
//     let mut map = Map::new();
//     for row in rows.split("\n").into_iter().skip(1) {
//         let parts = row.split(" ").collect::<Vec<&str>>();
//         let soil = parts[0].parse::<u32>().unwrap();
//         let seed = parts[1].parse::<u32>().unwrap();
//         let cnt = parts[2].parse::<u32>().unwrap();
//         map.insert(soil, seed, cnt);
//     }
//     map
// }

// struct Map<T, K> {
//     values: Vec<(T, T, K)>,
// }

// impl Map<u32, u64> {
//     fn new() -> Self {
//         Self { values: Vec::new() }
//     }

//     fn insert(&mut self, dst: u32, src: u32, length: u32) {
//         self.values.push((dst, src, src as u64 + length as u64));
//     }

//     fn get(&self, v: &u32) -> u32 {
//         for value in &self.values {
//             if *v >= value.1 && u64::from(*v) < value.2 {
//                 return v - value.1 + value.0;
//             }
//         }
//         v.clone()
//     }
// }

pub fn main() -> Result<(), Error> {
    let path = "input5.txt";
    // let path = "test5.txt";

    let mut input = File::open(path)?;
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    let content = buf.split("\n\n").collect::<Vec<&str>>();
    assert!(content.len() == 8, "not enough lines");

    let seeds: Vec<i64> = content[0]
        .split(" ")
        .into_iter()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut seeds = seeds
        .iter()
        .step_by(2)
        .cloned()
        .zip(seeds[1..].iter().step_by(2).cloned())
        .collect::<Vec<(i64, i64)>>();
    for s in &mut seeds {
        s.1 += s.0;
    }

    let queue = [
        part2map(content[1]),
        part2map(content[2]),
        part2map(content[3]),
        part2map(content[4]),
        part2map(content[5]),
        part2map(content[6]),
        part2map(content[7]),
    ];

    let mut cur = seeds;
    for map in queue {
        let mut tmp: Vec<(i64, i64)> = vec![];
        for (l, r) in cur {
            tmp.append(&mut map.get(l, r));
        }
        cur = tmp;
        // println!("{:?}", &cur);
    }
    let ans = cur.iter().min().unwrap().0;

    println!("ans: {:?}", ans);

    Ok(())
}

fn part2map(rows: &str) -> Map<i64> {
    let mut map = Map::new();
    for row in rows.split("\n").into_iter().skip(1) {
        let parts = row.split(" ").collect::<Vec<&str>>();
        let soil = parts[0].parse::<i64>().unwrap();
        let seed = parts[1].parse::<i64>().unwrap();
        let cnt = parts[2].parse::<i64>().unwrap();
        map.insert(soil, seed, cnt);
    }
    map
}

struct Map<T> {
    values: Vec<(T, T, T)>,
}

impl Map<i64> {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn insert(&mut self, dst: i64, src: i64, length: i64) {
        self.values.push((src, length, dst - src));
    }

    fn get(&self, l: i64, r: i64) -> Vec<(i64, i64)> {
        let mut ans = vec![];
        println!("get: {} {} {:?}", l, r, self.values);
        for (left, length, diff) in &self.values {
            let left = left.clone();
            let length = length.clone();
            let diff = diff.clone();
            if (l > left && l - left >= length) || r <= left {
                continue;
            }
            if l >= left && r - left <= length {
                ans.push((l + diff, r + diff));
                return ans;
            }
            if l < left && r - left > length {
                ans.push((left + diff, left + length + diff));
                ans.append(&mut self.get(l, left));
                ans.append(&mut self.get(left + length, r));
                return ans;
            }
            if l < left && r - left <= length {
                ans.push((left + diff, r + diff));
                ans.append(&mut self.get(l, left));
                return ans;
            }
            if l >= left && r - left > length {
                ans.push((l + diff, left + length + diff));
                ans.append(&mut self.get(left + length, r));
                return ans;
            }
        }
        if ans.len() == 0 {
            ans.push((l, r));
        }
        ans
    }
}

// TODO use sort&binary search instead of linear iteration when searching for overlapping intervals
