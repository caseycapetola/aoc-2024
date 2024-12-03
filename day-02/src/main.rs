use std::{fs::File, io::Read, path::Path};

fn main() {
    let mut count_unsafe = 0;
    let mut count_safe = 0;
    
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

    for i in lines {
        let str_nums: Vec<&str> = i.split_whitespace().collect();
        let mut nums: Vec<i32> = Vec::new();
        for j in str_nums {
            nums.push(j.parse::<i32>().unwrap());
        }
        let mut prev = nums[0];
        let mut curr = nums[1];

        
        let incr = prev < curr;

        let mut safe_flag = true;
        let mut subroutine_flag: bool;
        let mut temp: Vec<i32> = Vec::new();
        for y in 1..nums.len() {
            temp.push(nums[y]);
        }
        subroutine_flag = subroutine(&temp);
        temp.clear();
        if subroutine_flag {
            count_safe += 1;
            continue;
        }
        for x in 1..nums.len() {
            for z in 0..nums.len() {
                if z != x {
                    temp.push(nums[z]);
                }
            }
            subroutine_flag = subroutine(&temp);
            temp.clear();
            if subroutine_flag {
                break;
            }
            
            prev = nums[x-1];
            curr = nums[x];

            // Cases where conflicts with incr/decr
            if (prev > curr && incr) || (prev < curr && !incr) {
                count_unsafe += 1;
                safe_flag = false;
                break;
            }

            // Cases where diff is not within range
            if (curr-prev).abs() > 3 || (curr-prev).abs() < 1 {
                count_unsafe += 1;
                safe_flag = false;
                break; 
            }
        }
        if safe_flag || subroutine_flag {
            count_safe += 1;
        }
    }
    println!("Unsafe Reports: {}\nSafe Reports: {}", count_unsafe, count_safe);
}

fn subroutine(nums: &Vec<i32>) -> bool {
    let mut prev = nums[0];
    let mut curr = nums[1];

    
    let incr = prev < curr;

    for x in 1..nums.len() {
        prev = nums[x-1];
        curr = nums[x];

        // Cases where conflicts with incr/decr
        if (prev > curr && incr) || (prev < curr && !incr) {
            return false;
        }

        // Cases where diff is not within range
        if (curr-prev).abs() > 3 || (curr-prev).abs() < 1 {
            return false; 
        }
    }
    
    true
}
