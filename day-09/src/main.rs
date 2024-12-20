use std::{env, fs::File, io::Read, path::Path};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
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
    let mut disk = Vec::<String>::new();
    let mut is_file = true;
    let mut index= 0;
    for i in s.chars() {
        if is_file {
            for _ in 0..i.to_digit(10).unwrap() as i32 {
                // println!("INDEX: {}", index);
                // println!("x = {}", x);
                let value = index.to_string();
                disk.push(value.clone());
            }
            index += 1;
            is_file = false;
        }
        else {
            for _ in 0..i.to_digit(10).unwrap() as i32 {
                disk.push(".".to_string());
            }
            is_file = true;
        }
    }
    // println!("DISK (PRE): {}", disk.iter().collect::<String>());
    for x in (0..disk.len()).rev() {
        if disk[x] != "." {
            for y in 0..disk.len() {
                if x==y {
                    break;
                }
                if disk[y] == "." {
                    disk.swap(x, y);
                }
            }
        }
    }

    let mut checksum = 0;
    for i in 0..disk.len() {
        if disk[i] != "." {
            checksum += i as i32 * disk[i].chars().next().unwrap().to_digit(10).unwrap() as i32
        }
    }
    // println!("DISK: {:?}", disk);
    // println!("DISK joined: {}", disk.iter().collect::<String>());
    println!("CHECKSUM: {}", checksum);

}
