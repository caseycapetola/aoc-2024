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

    let lines = s.split("\n");
    let lines_clone = lines.clone();
    let mut num_cols = 0;
    for i in lines {
        num_cols = i.len();
    }
    let input = lines_clone.collect::<Vec<&str>>().join("");
    // println!("{}", input);
    let mut xmas_count = 0;
    let mut mas_count = 0;
    let mut r = 0;
    let mut d = 0;
    let mut ur = 0;
    let mut dr = 0;
    for i in 0..input.len() {
        // check to the right
        match check_right(&input, i, num_cols) {
            true => {
                xmas_count += 1;
                r += 1;
            },
            false => (),
        }
        match check_down(&input, i, num_cols) {
            true => {
                xmas_count += 1;
                d += 1;
            },
            false => (),
        }
        match check_up_right(&input, i, num_cols) {
            true => {
                xmas_count += 1;
                ur += 1;
            },
            false => (),
        }
        match check_down_right(&input, i, num_cols) {
            true => {
                xmas_count += 1;
                dr += 1;
            },
            false => (),
        }
        match check_cross(&input, i, num_cols) {
            true => mas_count += 1,
            false => (),
        }
    }

    println!("XMAS Count: {}", xmas_count);
    println!("R: {}, D: {}, UR: {}, DR: {}", r, d, ur, dr);
    println!("X-MAS Count: {}", mas_count);
}

fn check_right(chars: &String, index: usize, num_cols: usize) -> bool {
    let last_index = index+4;
    let row = index/num_cols;
    if last_index>chars.len() || (last_index-1)/num_cols != row {
        return false;
    }
    
    let subset = &chars[index..last_index];

    if subset == "SAMX" || subset == "XMAS" {
        return true;
    }
    false
}

fn check_down(chars: &String, index: usize, num_cols: usize) -> bool {
    let last_index = index + (num_cols*3);
    if index+(num_cols+1)>=chars.len() || index+(num_cols*2+1)>=chars.len() || last_index>=chars.len() {
        return false;
    }
    let char1: &str = &chars[index..index+1];
    let char2: &str = &chars[index+(num_cols)..index+(num_cols+1)];
    let char3: &str = &chars[index+(num_cols*2)..index+(num_cols*2+1)];
    let char4: &str = &chars[last_index..last_index+1];

    check_chars(char1, char2, char3, char4)
}

fn check_up_right(chars: &String, index: usize, num_cols: usize) -> bool {
    // println!("UP RIGHT CHECK");
    let cur_row = index/num_cols;
    let temp_cols = i32::try_from(num_cols).ok().unwrap();
    let i_check = i32::try_from(index).ok().unwrap();
    if i_check - temp_cols*3 + 3 <= 0 {
        return false;
    }
    let i1 = (index - num_cols + 1, (index-num_cols+1)/num_cols);
    let i2 = (i1.0 - num_cols + 1, (i1.0 - num_cols + 1)/num_cols);
    let i3 = (i2.0 - num_cols + 1, (i2.0 - num_cols + 1)/num_cols);
    // println!("index: ({},{}), i1: {:?}, i2: {:?}, i3: {:?}", index, cur_row, i1, i2, i3);
    if i3.1==i2.1 || i2.1==i1.1 || i1.1==cur_row {
        return false;
    }

    let c1 = &chars[index..index+1];
    let c2 = &chars[i1.0..i1.0+1];
    let c3 = &chars[i2.0..i2.0+1];
    let c4 = &chars[i3.0..i3.0+1];

    check_chars(c1, c2, c3, c4)
}

fn check_down_right(chars: &String, index: usize, num_cols: usize) -> bool {
    let cur_row = index/num_cols;
    let temp_cols = i32::try_from(num_cols).ok().unwrap();
    let temp_len = i32::try_from(chars.len()).ok().unwrap();
    let i_check = i32::try_from(index).ok().unwrap();
    if i_check + temp_cols*3 + 3 >= temp_len {
        return false;
    }
    let i1 = (index + num_cols + 1, (index+num_cols+1)/num_cols);
    let i2 = (i1.0 + num_cols + 1, (i1.0 + num_cols + 1)/num_cols);
    let i3 = (i2.0 + num_cols + 1, (i2.0 + num_cols + 1)/num_cols);
    if i3.1-i2.1>1 || i2.1-i1.1>1 || i1.1-cur_row>1 {
        // println!("index: ({},{}), i1: {:?}, i2: {:?}, i3: {:?}", index, cur_row, i1, i2, i3);
        return false;
    }

    let c1 = &chars[index..index+1];
    let c2 = &chars[i1.0..i1.0+1];
    let c3 = &chars[i2.0..i2.0+1];
    let c4 = &chars[i3.0..i3.0+1];

    check_chars(c1, c2, c3, c4)
}

fn check_chars(c1: &str, c2: &str, c3: &str, c4: &str) -> bool {
    if c1 == "X" && c2 == "M" && c3 == "A" && c4 == "S" {
        return true;
    }
    if c1 == "S" && c2 == "A" && c3 == "M" && c4 == "X" {
        return true;
    }
    false
}

fn check_cross(chars: &String, center: usize, num_cols: usize) -> bool {
    let cur_row = center/num_cols;
    if &chars[center..center+1] != "A" {
        return false;
    }
    if cur_row == 0 || cur_row == chars.len()/num_cols - 1 || center%num_cols==0 || center%num_cols==num_cols-1 {
        return false;
    }
    let ul = &chars[center-num_cols-1..center-num_cols];
    let ur = &chars[center-num_cols+1..center-num_cols+2];
    let dl = &chars[center+num_cols-1..center+num_cols];
    let dr = &chars[center+num_cols+1..center+num_cols+2];

    if ul=="M" && ur=="M" && dl=="S" && dr=="S" {
        return true;
    }
    if ul=="S" && ur=="S" && dl=="M" && dr=="M" {
        return true;
    }
    if ul=="M" && ur=="S" && dl=="M" && dr=="S" {
        return true;
    }
    if ul=="S" && ur=="M" && dl=="S" && dr=="M" {
        return true;
    }
    false
}
