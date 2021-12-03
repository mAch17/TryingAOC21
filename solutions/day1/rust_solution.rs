use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn main() {
	let mut numbers = vec![];
	let fp = File::open("input.txt").expect("Cannot Find file!! ");
	let mut reader = BufReader::new(fp);
    for line in reader.lines() {
            if let Ok(ip) = line {
                if let Ok(num) = ip.parse::<i32>() {
                	numbers.push(num);
                }
            }
        }
let mut counter = 0;
for i in 1..numbers.len(){
	if numbers[i]>numbers[i-1]{
		counter += 1;
	}
}
println!("Answer Part 1 :: {:?}", counter );
let mut counter2 = 0;
for i in 3..numbers.len(){
	let first_window = &numbers[i-3..i];
	let first_window_sum = &first_window.iter().sum::<i32>();
	let second_window = &numbers[i-2..i+1];
	let second_window_sum = &second_window.iter().sum::<i32>();
	if second_window_sum > first_window_sum{
		counter2 += 1;
	}
}
println!("Answer Part 2 :: {:?}", counter2 );
}
