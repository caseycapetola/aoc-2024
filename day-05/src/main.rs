use std::{fs::File, io::Read, path::Path};

fn main() {
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

    let mut lines = s.split("\n\n");
    let ordering_rules = lines.next().unwrap();
    let updates = lines.next().unwrap().split("\n");

    let rule_lines = ordering_rules.split("\n");
    
    let mut rules: Vec<(i32, i32)> = Vec::new();
    for i in rule_lines {
        let mut vals = i.split("|");
        let l = vals.next().unwrap().parse::<i32>().unwrap();
        let r = vals.next().unwrap().parse::<i32>().unwrap();
        rules.push((l, r));
    }

    let mut solution = 0;
    let mut sol_2 = 0;
    for i in updates {
        let mut flag = true;
        let pages = i.split(",");
        let mut prev_pages: Vec<i32> = Vec::new();
        for j in pages {
            let val = j.parse::<i32>().unwrap();
            for k in prev_pages.iter() {
                for l in rules.iter() {
                    if l.0==val && l.1==*k {
                        flag = false;
                    }
                }
            }
            // if flag == false { // Can be used with part 1 to improve speed
            //     break;
            // }
            prev_pages.push(val);
        }
        if flag==true {
            solution += extract_middle(&mut prev_pages);
        }
        else {
            sol_2 += fix_ordering(&rules, &mut prev_pages);
        }
    }

    println!("SOLUTION: {}", solution);
    println!("SOLUTION (part 2): {}", sol_2);

}


fn fix_ordering(rules: &Vec<(i32, i32)>, update: &mut Vec<i32>) -> i32 {
    let mut ordered = false;
    while !ordered {
        ordered = true;
        for i in 0..update.len()-1 {
            for j in i..update.len() {
                for k in rules {
                    if k.0==update[j] && k.1==update[i] {
                        ordered = false;
                        update.swap(i, j);
                    }
                }
            }
        }
    }
    
    extract_middle(update)
}

fn extract_middle(vals: &mut Vec<i32>) -> i32 {
    let len = vals.len();
    vals.swap_remove(len/2)
    
}
