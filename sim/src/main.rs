#![allow(unused)]


use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


pub fn main() {
    
let args: Vec<String> = env::args().skip(1).collect(); //The skip here stops the executable binary being placed in the output

println!("{:?}", args);

//This just ensures the correct number of command line arguments are provided and provides the user with a usage guide if incorrect
//The max amount of arguments (if using the verbose and or helper flag) is 9 whereas the minimum is 8
if args.len() < 8 || args.len() > 9 { 
    println!("Usage: [-hv] -s <s> -E <E> -b <b> -t <tracefile>");
    return;
}

let index_of_s = args.iter().position(|find_s: &String| find_s == "-s").unwrap();

#[allow(non_snake_case)] //Although I know this goes against variable naming conventions as we are dealing with a capital E I think it is justified to defy these conventions for clarity
let index_of_E = args.iter().position(|find_E: &String| find_E == "-E").unwrap();

let index_of_b = args.iter().position(|find_b: &String| find_b == "-b").unwrap();
let index_of_t = args.iter().position(|find_t: &String| find_t == "-t").unwrap();

let value_of_s: &String = &args[index_of_s + 1];
#[allow(non_snake_case)]
let value_of_E:&String = &args[index_of_E + 1];
let value_of_b:&String = &args[index_of_b + 1];
let path_to_trace:&String = &args[index_of_t + 1];



println!("Index of -s string: {}, Number value of s: {}",index_of_s, value_of_s);

println!("Index of -E string: {}, Number value of E: {}",index_of_E, value_of_E);

println!("Index of -b string: {}, Number value of b: {}",index_of_b, value_of_b);

println!("Index of -t string: {}, Path to trace file: {}",index_of_b, path_to_trace);

#[allow(non_snake_case)]
//This dynamically calculates how many BYTES the combined cache is
fn calculate_cache_size(s: &String, E:&String, b:&String) -> u32{
    let int_of_s: u32= s.parse().unwrap(); //This is the amount of Sets 
    let int_of_E: u32= E.parse().unwrap(); //This is the amount of Cache Lines per set 
    let int_of_b: u32= b.parse().unwrap(); //This is the block offset (the amount of bits that determine which byte to take from the requested block)

    let cache_bytes_size: u32 = (2_u32.pow(int_of_s)) * (2_u32.pow(int_of_b)) * int_of_E;

    cache_bytes_size

}

fn make_file_line_seperated_vector(filepath: &str) -> Vec<String>{
    let file = File::open(filepath).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut vec_of_lines: Vec<String> = vec![];
    for line in reader.lines(){
        let line_to_add = line.unwrap();
        vec_of_lines.push(line_to_add);
    }
    return vec_of_lines;
}

let vector_of_lines = make_file_line_seperated_vector(&path_to_trace);
let mut count_of_lines = 0;
let mut vector_of_line_index:Vec<usize> = vec![];
let mut index_counter = 0;

for line in &vector_of_lines{
    println!("{}", line);
}

if value_of_E == "1"{
    println!("This is a direct mapped cache")
}


let cache_bytes_size = calculate_cache_size(value_of_s, value_of_E, value_of_b);

println!("The size of the cache is {} bytes", cache_bytes_size)

}