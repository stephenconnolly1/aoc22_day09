use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;

enum Instruction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Default)]
struct Pos{
    pub x: i32,
    pub y: i32,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len()==2, "You must provide a file argument");
    let file_path = &args[1];
    let history: HashMap<Pos, i32> = process_instructions(file_path);   
    println!("{}", history.len());
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Instruction {
    let mut toks = line.split(' ').fuse();
    let first = toks.next();
    let second = toks.next();
    let mut distance: u32 = 0;
    match second {
        Some(num) => {
            println!("number of steps: {}", num);
            distance = num.parse::<u32>().unwrap();
        }
        _ => println! ("Invalid parameter") 
    };
    
    match first {
        Some("U") => Instruction::Up(distance),
        Some("D") => Instruction::Down(distance),
        Some("L") => Instruction::Left(distance),
        Some("R") => Instruction::Right(distance),
        _ => todo!(),
    }
}
fn process_instructions(file_path: &String) -> HashMap<Pos, i32>{
    let mut tail_history: HashMap<Pos, i32> = HashMap::new();
    
    let mut rope = [Pos::default(); 10];
    tail_history.insert(rope[9].clone(), 0);
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {
                println! ("{:?}", l);
                let i: Instruction = parse_line(&l);
                follow_heads(i, &mut rope, &mut tail_history) ;
            }
        }
    }
    return tail_history;
}

fn follow_heads(i: Instruction, rope: &mut [Pos], tail_history: &mut HashMap<Pos, i32> )  {
    let distance = match i {
        Instruction::Up (dist)=> dist,
        Instruction::Down (dist)=> dist,
        Instruction::Left (dist)=> dist,
        Instruction::Right (dist)=> dist,
    }; 
    for _ in 0..distance {
        match i {
            Instruction::Up     (_) => rope[0].y = rope[0].y+1,
            Instruction::Down   (_) => rope[0].y = rope[0].y-1,
            Instruction::Left   (_) => rope[0].x = rope[0].x-1,
            Instruction::Right  (_) => rope[0].x = rope[0].x+1, 
        };
        for i in 0..rope.len()-1 {
            follow_head(rope[i].clone(), &mut rope[i+1]);
        }
        tail_history.insert(rope[9].clone(), 0);
    }
}
fn follow_head(leader: Pos,   follower: &mut Pos) {

    // If head is now 1 or zero distance from tail, do nothing
    // Else tail needs to move closer to head. 
    // If head and tail in same row or column then tail moves in that row/col
    // otherwise tail moves diagonally to be in same row/col as head
    if (leader.x - follower.x).abs() <= 1 && (leader.y - follower.y).abs() <= 1 {return};

    // leader and follower same column
    if leader.x==follower.x {
        if leader.y > follower.y { 
            follower.y = follower.y+1;
        }
        else {
            follower.y = follower.y-1;
        }
        return;
    }

    // leader and follower same row
    if leader.y==follower.y {
        if leader.x > follower.x { 
            follower.x = follower.x+1;
        }
        else {
            follower.x = follower.x-1;
        }
        return;
    }

    // leader and follower in diff row and column, move diagnoally
    if follower.x > leader.x {
        follower.x = follower.x - 1;
    } else {
        follower.x = follower.x + 1;
    }    
    if follower.y > leader.y {
        follower.y = follower.y - 1;
    } else{
        follower.y = follower.y + 1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let file_path = "./src/test2.txt".to_string();
        let history: HashMap<Pos, i32> = process_instructions(&file_path);
        assert_eq!(history.len(), 36);

    }
}