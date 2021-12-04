use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct Position {
	x: isize,
	y: isize
}

#[derive(Debug)]
struct Position2 {
    x: isize,
    y: isize,
    aim: isize
}

fn move_sub(point: &mut Position, instruction:&String) {
    let string_split = instruction.split(" ").collect::<Vec<&str>>();
    assert!(string_split.len()==2);
    let instruction = string_split[0];
    if let Ok(magnitude) = string_split[1].parse::<isize>(){
            if instruction=="up"{
        point.y -= magnitude
    } else if instruction == "down"{
        point.y += magnitude
    } else if instruction == "forward"{
        point.x += magnitude
    } else {
        println!("No idea about instruction {:?}", instruction );
        assert!(false);
    }
    }
    else {println!("Cannot Convert {:?} into number", string_split[1] ); assert!(false); }

}

fn move_sub2(point: &mut Position2, instruction:&String) {
    let string_split = instruction.split(" ").collect::<Vec<&str>>();
    assert!(string_split.len()==2);
    let instruction = string_split[0];
    if let Ok(magnitude) = string_split[1].parse::<isize>(){
            if instruction=="up"{
        point.aim -= magnitude
    } else if instruction == "down"{
        point.aim += magnitude
    } else if instruction == "forward"{
        point.x += magnitude;
        point.y += (point.aim * magnitude )
    } else {
        println!("No idea about instruction {:?}", instruction );
        assert!(false);
    }
    }
    else {println!("Cannot Convert {:?} into number", string_split[1] ); assert!(false); }

}

fn main() {
	let mut instructions = vec![];
	let fp = File::open("input.txt").expect("Cannot Find file!! ");
	let mut reader = BufReader::new(fp);
    for line in reader.lines() {
            if let Ok(ip) = line {
                instructions.push(ip)
            }
        }
    let mut point = Position{x:0,y:0};
    for instruction in &instructions{
        move_sub(&mut point,&instruction);
    }
    println!("Final Position {:?}", point );
    println!("Answer 1 {:?}", point.x * point.y );

    let mut point2 = Position2{x:0,y:0,aim:0};
    for instruction in &instructions{
        move_sub2(&mut point2,&instruction);
    }
    println!("Final Position {:?}", point2 );
    println!("Answer 2 {:?}", point2.x * point2.y );

    }