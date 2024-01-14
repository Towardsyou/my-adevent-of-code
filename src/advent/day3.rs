use std::{
    fs::File,
    io::{BufRead, BufReader, Error}, collections::HashMap,
};

// pub fn main() -> Result<(), Error> {
//     let path = "input3.txt";
//     // let path = "test3.txt";

//     let input = File::open(path)?;
//     let buffered = BufReader::new(input);

//     let mut g: Vec<Vec<char>> = vec![];

//     for line in buffered.lines() {
//         let line = line.unwrap();
//         let mut cur: Vec<char> = vec![];
//         for c in line.chars() {
//             cur.push(c);
//         }
//         g.push(cur);
//     }

//     let m = g.len();
//     let mut left: usize = 0x3f3f3f3f;
//     let mut right: usize = 0x3f3f3f3f;
//     let mut ans = 0;
//     for i in 0..m {
//         let n = g[i].len();
//         let mut j = 0;
//         let mut num = 0;
//         let mut found = false;
//         while j < n {
//             while j < n && g[i][j].is_digit(10) {
//                 if left == 0x3f3f3f3f {
//                     left = j;
//                 }
//                 num = num * 10 + g[i][j].to_digit(10).unwrap();
//                 right = j;
//                 j += 1;
//             }
//             if left == 0x3f3f3f3f {
//                 j += 1;
//                 continue;
//             }
//             if i >= 1 {
//                 let tmp_left = if left > 0 { left - 1 } else { 0 };
//                 let tmp_right = if right < g[i - 1].len() - 1 { right + 1 } else { right };
//                 for k in tmp_left..=tmp_right {
//                     if !g[i - 1][k].is_digit(10) && g[i - 1][k] != '.' {
//                         found = true;
//                         break;
//                     }
//                 }
//             }
//             if left >= 1 && !g[i][left - 1].is_digit(10) && g[i][left - 1] != '.' {
//                 found = true;
//             }
//             if right < m - 1 && !g[i][right + 1].is_digit(10) && g[i][right + 1] != '.' {
//                 found = true;
//             }
//             if i < m - 1 {
//                 let tmp_left = if left > 0 { left - 1 } else { 0 };
//                 let tmp_right = if right < g[i + 1].len() - 1 { right + 1 } else { right };
//                 for k in tmp_left..=tmp_right {
//                     if !g[i + 1][k].is_digit(10) && g[i + 1][k] != '.' {
//                         found = true;
//                         break;
//                     }
//                 }
//             }
//             println!("Number {found}: {num}");
//             if found {
//                 ans += num;
//             }
//             left = 0x3f3f3f3f;
//             right = 0x3f3f3f3f;
//             num = 0;
//             found = false;
//         }
//     }

//     println!("Answer is: {ans}");
//     Ok(())
// }

pub fn main() -> Result<(), Error> {
    let path = "input3.txt";
    // let path = "test3.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut g: Vec<Vec<char>> = vec![];

    for line in buffered.lines() {
        let line = line.unwrap();
        let mut cur: Vec<char> = vec![];
        for c in line.chars() {
            cur.push(c);
        }
        g.push(cur);
    }
    
    let mut map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let m = g.len();
    let mut left: usize = 0x3f3f3f3f;
    let mut right: usize = 0x3f3f3f3f;
    let mut ans = 0;
    for i in 0..m {
        let n = g[i].len();
        let mut j = 0;
        let mut num = 0;
        while j < n {
            while j < n && g[i][j].is_digit(10) {
                if left == 0x3f3f3f3f {
                    left = j;
                }
                num = num * 10 + g[i][j].to_digit(10).unwrap();
                right = j;
                j += 1;
            }
            if left == 0x3f3f3f3f {
                j += 1;
                continue;
            }
            if i >= 1 {
                let tmp_left = if left > 0 { left - 1 } else { 0 };
                let tmp_right = if right < g[i - 1].len() - 1 { right + 1 } else { right };
                for k in tmp_left..=tmp_right {
                    if g[i - 1][k] == '*' {
                        map.entry((i - 1, k)).or_insert(Vec::new()).push(num);
                    }
                }
            }
            if left >= 1 && g[i][left - 1] == '*' {
                map.entry((i, left - 1)).or_insert(Vec::new()).push(num);
            }
            if right < m - 1 && g[i][right + 1] == '*' {
                map.entry((i, right + 1)).or_insert(Vec::new()).push(num);
            }
            if i < m - 1 {
                let tmp_left = if left > 0 { left - 1 } else { 0 };
                let tmp_right = if right < g[i + 1].len() - 1 { right + 1 } else { right };
                for k in tmp_left..=tmp_right {
                    if g[i + 1][k] == '*' {
                        map.entry((i + 1, k)).or_insert(Vec::new()).push(num);
                    }
                }
            }
            left = 0x3f3f3f3f;
            right = 0x3f3f3f3f;
            num = 0;
        }
    }
    for (pos, v) in map {
        if v.len() == 2 {
            println!("Found: star at {pos:?} has: {v:?} -> {}", v[0] * v[1]);
            ans += v[0] * v[1];
        }
    }

    println!("Answer is: {ans}");
    Ok(())
}
