use std::{env, fs::File, io::{self, BufRead, Read}, path::Path};

fn main() {
    // EXAMPLE FILE PARSING FOR DAY 6
    // let grid: Vec<Vec<char>> = include_str!("../input.txt")
    //     .lines()
    //     .map(|line| line.chars().collect())
    //     .collect();
    
    env::set_var("RUST_BACKTRACE", "1");

    let path = Path::new("sample_small.txt");
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
    let mut count = 0;
    for i in lines {
        let mut eqn = i.split(":");
        let soln = eqn.next()
            .unwrap()
            .parse::<i128>()
            .unwrap();
        let vals_str: Vec<&str> = eqn.next()
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();
        let mut vals: Vec<i128> = Vec::new();
        for i in vals_str {
            vals.push(i.parse::<i128>().unwrap());
        }
        // println!("soln: {}\nvals: {:?}", soln, vals);
        // println!("{}\n\n", subroutine(soln, &vals));
        match subroutine(soln, &vals) {
            true => count += soln,
            false => (),
        }
    }
    println!("SOLUTION: {}", count);

    let file_path = "sample_small.txt";

    // Read equations from the file
    let equations = read_equations(file_path).expect("Failed to read equations from file");

    let mut total_calibration_result = 0;

    for (target, numbers) in equations {
        if can_produce_target(&numbers, target) {
            total_calibration_result += target;
            // println!("Valid equation for target {}: {:?}", target, numbers);
        } else {
            // println!("No valid equation for target {}: {:?}", target, numbers);
        }
    }

    println!("Total calibration result: {}", total_calibration_result);

}

fn subroutine(soln: i128, vals: &Vec<i128>) -> bool {
    let exp = (vals.len()-1) as u32;
    let mut num_ways = 2_i128.pow(exp);
    let mask = 0b1;
    let mut count;
    if num_ways == 1 {
        if soln == vals[0] {
            return true;
        }
        return false;
    }
    // Part 1 Logic
    while num_ways>0 {
        num_ways -= 1;
        let mut temp = num_ways;
        count = vals[0];
        for i in 1..vals.len() {
            // println!("STATE\n\ni: {}, soln: {}, vals: {:?}", i, soln, vals);
            if temp & mask == 0b1 {
                match count.checked_add(vals[i]) {
                    Some(x) => count = x,
                    None => break,
                }
            }
            else {
                match count.checked_mul(vals[i]) {
                    Some(x) => count = x,
                    None => break,
                }
            }
            temp = temp>>1;
        }
        if count == soln {
            return true;
        }
    }

    false
}

// Part 2 logic -> call subroutine on subset of concatenated vector
// Use similar bit logic: 1 -> concatenate, 0 -> don't concatenate
fn _part_2(soln: i128, vals: &Vec<i128>) -> bool {
    // let sol_str = soln.to_string();
    // println!("PART 2: soln: {} --- vals: {:?}", soln, vals);
    let exp = (vals.len()-1) as u32;
    let mut num_ways = 2_i128.pow(exp);
    // println!("NUM WAYS: {}", num_ways);
    let mask = 0b1;
    let mut workstring ;
    let mut new_vals: Vec<i128> = Vec::new();

    // if subroutine(soln, vals) {
    //     return true;
    // }

    while num_ways>0 {
        new_vals.clear();
        workstring = vals[0].to_string();
        num_ways -= 1;
        let mut temp = num_ways;
        let mut vals_copy = vals.clone();
        vals_copy.reverse();
        vals_copy.pop();
        for _ in 1..vals.len() {
            if temp & mask == 0b1 {
                let val_str = vals_copy.pop()
                    .unwrap_or_default()
                    .to_string();
                // println!("VAL_STR: {}", val_str);
                workstring += &val_str;
                // println!("soln: {}\nworkstring: {}\n", soln, workstring);
                // println!("WORKSTRING CONCAT: {}", workstring);
            }
            else {
                match workstring {
                    val if val == "DONE".to_string() => (),
                    _ => new_vals.push(workstring.parse::<i128>().unwrap()), 
                }
                match vals_copy.pop() {
                    Some(x) => workstring = x.to_string(),
                    None => workstring = "DONE".to_string(),
                }
            }
            temp = temp>>1;
        }
        if soln == 7290 {
            print!("b4: {:?} --- ", new_vals);
        }
        match workstring {
            ref val if *val == "DONE".to_string() => (),
            _ => new_vals.push(workstring.parse::<i128>().unwrap()),
        }
        if soln == 7290 {
            println!("vec: {:?}", new_vals);
        }
        if subroutine(soln, &new_vals) {
            if soln == 7290 {
                println!("SOLUTION FOUND");
            }
            return true;
        }
    }
    false

}

// CHAT GPT WORK :(
// Function to read equations from a file
fn read_equations(file_path: &str) -> io::Result<Vec<(i64, Vec<i64>)>> {
    let mut equations = Vec::new();
    let file = File::open(file_path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue; // Skip invalid lines
        }

        let target: i64 = parts[0].trim().parse().expect("Invalid target value");
        let numbers: Vec<i64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().expect("Invalid number"))
            .collect();

        equations.push((target, numbers));
    }
    Ok(equations)
}

// Function to check if a target value can be produced
fn can_produce_target(numbers: &[i64], target: i64) -> bool {
    dfs(numbers, target, 0, numbers[0]).unwrap_or(false)
}

// Depth-first search to explore all possible operator combinations with overflow checks
fn dfs(numbers: &[i64], target: i64, index: usize, current_value: i64) -> Option<bool> {
    if index == numbers.len() - 1 {
        return Some(current_value == target);
    }

    let next_index = index + 1;

    // Try addition
    if let Some(next_value) = current_value.checked_add(numbers[next_index]) {
        if dfs(numbers, target, next_index, next_value).unwrap_or(false) {
            return Some(true);
        }
    }

    // Try multiplication
    if let Some(next_value) = current_value.checked_mul(numbers[next_index]) {
        if dfs(numbers, target, next_index, next_value).unwrap_or(false) {
            return Some(true);
        }
    }

    // Try concatenation
    let concatenated_value = concatenate(current_value, numbers[next_index]);
    if dfs(numbers, target, next_index, concatenated_value).unwrap_or(false) {
        return Some(true);
    }

    Some(false)
}

// Function to concatenate two numbers
fn concatenate(left: i64, right: i64) -> i64 {
    let right_str = right.to_string();
    let concatenated = format!("{}{}", left, right_str);
    concatenated.parse::<i64>().unwrap()
}