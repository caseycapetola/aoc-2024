use regex::Regex;
use std::{fs::File, io::Read, path::Path};

fn main() {
    let mut total = 0;
    
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

    let re = Regex::new(r"mul\(\d*,\d*\)").unwrap();

    let m: Vec<&str> = re.captures_iter(&s).map(|cap| cap.get(0).unwrap().as_str()).collect(); // REVIEW THIS LINE
    for i in m {
        let reg = Regex::new(r"\d*").unwrap();
        let n: Vec<&str> = reg.captures_iter(&i).map(|cap| cap.get(0).unwrap().as_str()).collect();
        let mut nums: Vec<i32> = Vec::new();
        for x in n {
            if x != "" {
                nums.push(x.parse::<i32>().unwrap());
            }
        }
        if nums.len() == 2 {
            let value1= nums.pop().unwrap();
            let value2 = nums.pop().unwrap();
            total += value1*value2;
        }
    }
    println!("TOTAL: {}", total);

    // Part 2
    total = 0;
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

    let re = Regex::new(r"mul\(\d*,\d*\)|do\(\)|don't\(\)").unwrap();

    let m: Vec<&str> = re.captures_iter(&s).map(|cap| cap.get(0).unwrap().as_str()).collect(); // REVIEW THIS LINE
    let mut flag = true;
    for i in m {
        if i == "do()" {
            flag = true;
            continue;
        }
        if i == "don't()" {
            flag = false;
            continue;
        }
        if flag {
            let reg = Regex::new(r"\d*").unwrap();
            let n: Vec<&str> = reg.captures_iter(&i).map(|cap| cap.get(0).unwrap().as_str()).collect();
            let mut nums: Vec<i32> = Vec::new();
            for x in n {
                if x != "" {
                    nums.push(x.parse::<i32>().unwrap());
                }
            }
            if nums.len() == 2 {
                let value1= nums.pop().unwrap();
                let value2 = nums.pop().unwrap();
                total += value1*value2;
            }
        }
    }
    println!("TOTAL: {}", total);
}
