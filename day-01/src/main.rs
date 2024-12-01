use std::{fs::File, io::Read, path::Path};
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let path = Path::new("sample.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    let lines = s.split("\n");
    // let mut left = vec!(0; lines.clone().count());
    let mut left = vec!(0; 1);
    let mut right = vec!(0; 1);
    for i in lines {
        let collect = i.split_whitespace().collect();
        let content: Vec<&str> = collect;
        left.push(content[0].parse::<i32>().unwrap());
        right.push(content[1].parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();

    let mut diff = 0;

    for x in 0..left.len() {
        println!("{}   {}", left[x], right[x]);
        diff += (left[x] - right[x]).abs();
    }

    println!("TOTAL DIFFERENCE: {}", diff);

    let mut frequency: HashMap<i32, i32> = HashMap::new();
    for i in left {
        let count = frequency.entry(i).or_insert(0);
        *count += 1;
    }
    let mut similarity_score = 0;
    for j in right {
        match frequency.contains_key(&j) {
            true => similarity_score += j*frequency.get(&j).unwrap(),
            false => (),
        }
    }
    println!("SIMILARITY SCORE: {}", similarity_score);


}