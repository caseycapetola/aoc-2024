use std::{fs::File, io::Read, path::Path};

#[derive(Debug)]
enum ArrowKind {
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
}
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

    let mut num_cols = 0;
    let lines = s.split("\n");
    let mut grid: Vec<&str> = Vec::new();
    for i in lines {
        if num_cols == 0 {
            num_cols = i.len();
        }
        for j in 0..i.len() {
            grid.push(&i[j..j+1]);
        }
    }
    // grid[0 as usize] = "?";
    // println!("GRID: {:?}", grid);
    
    let mut index = 0;
    let mut dir: ArrowKind = ArrowKind::DownArrow;
    for i in 0..grid.len() {
        if grid[i]=="^" {
            index = i;
            dir = ArrowKind::UpArrow;
            break;
        }
        else if grid[i]=="v" {
            index = i;
            dir = ArrowKind::DownArrow;
            break;
        }
        else if grid[i]==">" {
            index = i;
            dir = ArrowKind::RightArrow;
            break;
        }
        else if grid[i]=="<" {
            index = i;
            dir = ArrowKind::LeftArrow;
            break;
        }
    }
    println!("ARROW KIND: {:?}", dir);
    let mut done = false;
    while !done {
        match movement(&mut grid, &mut dir, num_cols, index) {
            Some(pos) => index = pos,
            None => done = true,
        }
    }
    let mut count = 0;
    for i in grid {
        if i=="X" {
            count += 1;
        }
    }
    println!("DISTINCT POSITIONS: {}", count);


}

// Return None when done with movement opportunities
fn movement(grid: &mut Vec<&str>, dir: &mut ArrowKind, num_cols: usize, mut pos: usize) -> Option<usize> {
    match dir {
        ArrowKind::UpArrow => {
            if pos<num_cols {
                grid[pos] = "X";
                return None;
            }
            if grid[pos-num_cols] == "#" {
                *dir = ArrowKind::RightArrow;
                return Some(pos);
            }
            let prev_pos = pos;
            pos -= num_cols;
            grid[prev_pos] = "X";
            return Some(pos);

        },
        ArrowKind::DownArrow => {
            if pos+num_cols>=grid.len() {
                grid[pos] = "X";
                return None;
            }
            if grid[pos+num_cols] == "#" {
                *dir = ArrowKind::LeftArrow;
                return Some(pos);
            }
            let prev_pos = pos;
            pos += num_cols;
            grid[prev_pos] = "X";
            return Some(pos);
        },
        ArrowKind::LeftArrow => {
            if pos == 0 || (pos-1)%num_cols == (num_cols-1) {
                grid[pos] = "X";
                return None;
            }
            if grid[pos-1] == "#" {
                *dir = ArrowKind::UpArrow;
                return Some(pos);
            }
            let prev_pos = pos;
            pos -= 1;
            grid[prev_pos] = "X";
            return Some(pos);
        },
        ArrowKind::RightArrow => {
            if pos == grid.len()-1 || (pos+1)%num_cols == 0 {
                grid[pos] = "X";
                return None;
            }
            if grid[pos+1] == "#" {
                *dir = ArrowKind::DownArrow;
                return Some(pos);
            }
            let prev_pos = pos;
            pos += 1;
            grid[prev_pos] = "X";
            return Some(pos);
        },
    }
}

fn _get_obstacles(grid: &mut Vec<&str>, locations: &mut Vec<usize>) {
    for i in 0..grid.len() {
        if grid[i] == "#" {
            locations.push(i);
        }
    }
}

fn _loop_check(grid: &mut Vec<&str>, dir: &mut ArrowKind, num_cols: usize, mut pos: usize) -> Option<usize> {
    match dir {
        ArrowKind::UpArrow => {
            if pos<num_cols {
                grid[pos] = "X";
                return None;
            }
            if grid[pos-num_cols] == "#" {
                *dir = ArrowKind::RightArrow;
                return Some(pos);
            }
            let prev_pos = pos;
            pos -= num_cols;
            grid[prev_pos] = "X";
            return Some(pos);

        },
        ArrowKind::DownArrow => {
            if pos+num_cols>=grid.len() {
                grid[pos] = "X";
                return None;
            }
            if grid[pos+num_cols] == "#" {
                *dir = ArrowKind::LeftArrow;
                return Some(pos);
            }
            let prev_pos = pos;
            pos += num_cols;
            grid[prev_pos] = "X";
            return Some(pos);
        },
        ArrowKind::LeftArrow => {
            if pos == 0 || (pos-1)%num_cols == (num_cols-1) {
                grid[pos] = "X";
                return None;
            }
            if grid[pos-1] == "#" {
                *dir = ArrowKind::UpArrow;
                return Some(pos);
            }
            let prev_pos = pos;
            pos -= 1;
            grid[prev_pos] = "X";
            return Some(pos);
        },
        ArrowKind::RightArrow => {
            if pos == grid.len()-1 || (pos+1)%num_cols == 0 {
                grid[pos] = "X";
                return None;
            }
            if grid[pos+1] == "#" {
                *dir = ArrowKind::DownArrow;
                return Some(pos);
            }
            let prev_pos = pos;
            pos += 1;
            grid[prev_pos] = "X";
            return Some(pos);
        },
    }
}
