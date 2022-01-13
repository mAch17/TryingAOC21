use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
enum BinStringDataHolder {
	Length(usize),
	Unknown
}

#[derive(Debug)]
struct Charcounts {
	zero_counts: isize,
	one_counts: isize
}


fn prob1() {
	let mut binnums = vec![];
	let fp = File::open("input.txt").expect("Cannot Find file!! ");
	let mut reader = BufReader::new(fp);
    for line in reader.lines() {
            if let Ok(ip) = line {
                binnums.push(ip)
            }
        }
    let mut per_line_length = BinStringDataHolder::Unknown;
    let mut loccounts:HashMap<usize,Charcounts> = HashMap::new();
    for (linnum,binnumstr) in binnums.iter().enumerate(){
    	if linnum==0{
    		per_line_length = BinStringDataHolder::Length(binnumstr.len());
    		for i in 0..binnumstr.len(){
    			let mut counts = Charcounts{zero_counts:0,one_counts:0};
    			loccounts.insert(i,counts);
    		}
    	} else {
    	if let BinStringDataHolder::Length(firstlinelength) = per_line_length {
    			assert!(firstlinelength==binnumstr.len());
    			} else {
    		panic!("Length known beyond first line at line number {:?}", linnum  );
    		}
    	}
    	// Inside For loop, do things generic to all lines here
    	//println!("{:?}{:?}", linnum, binnumstr );
    	for (charnum,c) in binnumstr.chars().enumerate(){
    		if c=='0'{
    			if let Some(cc) = loccounts.get_mut(&charnum){
    				let prev_zeros = cc.zero_counts;
    				let new_zeros = prev_zeros + 1;
    				cc.zero_counts = new_zeros;
    			} } else if c=='1' {
    			if let Some(cc) = loccounts.get_mut(&charnum){
    				let prev_ones = cc.one_counts;
    				let new_ones = prev_ones + 1;
    				cc.one_counts = new_ones;
    			}	
    			} else {
    				println!("{:?} seen, not counting", c );
    			}
    	}
    }
    let mut gammastr = String::from("");
    let mut epsilonstr = String::from("");
    let mut loccounts_vec:Vec<_> = loccounts.iter().collect();
    &loccounts_vec.sort_by_key(|x| x.0);
    for (posnum,postruct) in &loccounts_vec{
    	let num_zeros = postruct.zero_counts;
    	let num_ones = postruct.one_counts;
    	let mut common = '1';
    	let mut uncommon='0';
    	if num_ones>=num_zeros{
    		common = '0';
    		uncommon = '1';
    	}
    	gammastr.push(common);
    	epsilonstr.push(uncommon);
    }
    println!("{:?} {:?}",gammastr,epsilonstr );
    let gammaval = isize::from_str_radix(&gammastr, 2).unwrap();
    let epsilonval = isize::from_str_radix(&epsilonstr, 2).unwrap();
    println!("{:?} X {:?} = {:?}", gammaval, epsilonval, gammaval * epsilonval );
}



fn main() {
	prob1();
}