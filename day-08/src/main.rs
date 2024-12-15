use std::{fs::File, io::Read, path::Path};
use std::collections::{HashMap, HashSet};

fn calc_distance(p1: (i32, i32), p2: (i32, i32)) -> (i32, i32) {
    (p2.0 - p1.0, p2.1 - p1.1)
}

fn main() {
    let part_flag = 2;
    let path = Path::new("sample.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    let lines = s.split("\n");
    let mut char_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut row = 0;
    let mut num_cols: i32 = 0;
    let mut num_rows = 0;
    for line in lines {
        num_rows += 1;
        num_cols = line.len() as i32;
        let mut col = 0;
        for ch in line.chars() {
            if ch == '.' {
                col += 1;
                continue;
            }
            // println!("LINE: {}\nCHAR: {}\n({},{})\n", line, ch, row, col);
            char_map.entry(ch).or_insert_with(Vec::new).push((row, col));
            col += 1;
        }
        row += 1;
    }
    // println!("ROWS: {}\nCOLS: {}", num_rows, num_cols);
    let mut count = 0;
    let mut seen_coords = HashSet::new();
    for (_ch, coords) in &char_map {
        // println!("Character: {}, Coordinates: {:?}", ch, coords);
        if part_flag==1 {
            for i in 0..coords.len() {
                for j in 0..coords.len() {
                    if i == j {
                        continue;
                    }
                    let dir = calc_distance(coords[i], coords[j]);
                    // println!("Distance between {:?} and {:?} is {:?}", coords[i], coords[j], dir);
                    if coords[i].0 + dir.0 * 2 < 0 || coords[i].1 + dir.1 * 2 < 0 {
                        continue;
                    }
                    if coords[i].0 + dir.0 * 2 >= num_rows || coords[i].1 + dir.1 * 2 >= num_cols {
                        continue;
                    }
                    let new_coord = (coords[i].0 + dir.0 * 2, coords[i].1 + dir.1 * 2);
                    // println!("New coordinate: {:?}", new_coord);
                    if !seen_coords.contains(&new_coord) {
                        count += 1;
                        seen_coords.insert(new_coord);
                    }
                }
            }
        }
        // println!("COORDS: {:?}", coords);
        if part_flag == 2 {
            for i in 0..coords.len() {
                for j in 0..coords.len() {
                    if i == j {
                        continue;
                    }
                    let dir = calc_distance(coords[i], coords[j]);
                    let mut modifier = 1;
                    let mut flag = true;
                    while flag {
                        // println!("{:?}, {:?} --> {}", coords[i], coords[j], modifier);
                        // println!("Distance between {:?} and {:?} is {:?}", coords[i], coords[j], dir);
                        if coords[i].0 + dir.0 * modifier < 0 || coords[i].1 + dir.1 * modifier < 0 {
                            flag=false;
                            continue;
                        }
                        if coords[i].0 + dir.0 * modifier >= num_rows || coords[i].1 + dir.1 * modifier >= num_cols {
                            flag=false;
                            continue;
                        }
                        let new_coord = (coords[i].0 + dir.0 * modifier, coords[i].1 + dir.1 * modifier);
                        // println!("New coordinate: {:?}", new_coord);
                        if !seen_coords.contains(&new_coord) {
                            count += 1;
                            seen_coords.insert(new_coord);
                        }
                        modifier += 1;
                    }
                    // println!("DONE");
                }
            }
        }
    }
    println!("Count: {}", count);
}
