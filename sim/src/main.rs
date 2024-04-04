#[allow(unused)]
use core::num;
use std::vec;
#[allow(unused)]
use std::process::id;
#[allow(unused)]
use std::{alloc::System, env, fs::File, io::{self, BufRead, BufReader}, collections::HashMap};
#[macro_use]
extern crate maplit;
#[allow(unused)]
pub mod cache;


fn make_file_line_separated_vector(filepath: &str) -> Vec<String>{
    let file = File::open(filepath).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut vec_of_lines: Vec<String> = vec![];
    for line in reader.lines(){
        
        let line_to_add = line.unwrap();
        if !line_to_add.contains('I'){
            vec_of_lines.push(line_to_add);
        }
    }
    return vec_of_lines;
}

fn convert_from_hex_to_binary(hex_address: &str) -> Result<String, &'static str>{ //The function either returns a String or an error (as handled by &'static str) Just for future reference, the reason you chose a static lifetime is so the error message/string persists for the whole time the program executes as we might need to see at the last second that one of the strings failed to convert to binary
    let my_map_of_hex_to_binary: HashMap<char, String> = hashmap! {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'a' => "1010".to_string(),
        'b' => "1011".to_string(),
        'c' => "1100".to_string(),
        'd' => "1101".to_string(),
        'e' => "1110".to_string(),
        'f' => "1111".to_string()
    };
    let mut binary = String::new();
    for number in hex_address.chars(){
        if let Some(converted_value_of_single_character) =  my_map_of_hex_to_binary.get(&number){
        binary.push_str(converted_value_of_single_character);
    }
    else{
        return Err("Hex character not valid")
    }
}
    Ok(binary)
    
}

struct TupleOfTagAndAddress<'a>{
    tag: &'a str,
    binary: String
}

fn turn_line_sep_vector_into_tuple(line_of_vec: &str)-> Result<TupleOfTagAndAddress, &'static str>{ //if first_byte != 32{ remember the below function should only run if this if statement is satisfied as 32 is equivalent to a space 
    let first_byte:i32 = line_of_vec.chars().next().unwrap() as i32;
    if first_byte == 32{
        let index_of_comma = line_of_vec.find(",").unwrap_or(line_of_vec.len()); //This tries to find the comma and if it fails it instead returns the length of the string 
        let line_of_vec_with_size_removed: &str = &line_of_vec[0..index_of_comma];
        let split_instruction: Vec<&str> = line_of_vec_with_size_removed.split_whitespace().collect();
        let tag_address = TupleOfTagAndAddress{
            tag: split_instruction[0],
            binary: convert_from_hex_to_binary(split_instruction[1]).unwrap() 
        };
        Ok(tag_address)
    }
        else{
            return Err("First character is not a space and therefore it is ignored")
        }
    } 

    struct BinaryInTagSetBlockParts<>{//Okay so we need to split the binary address into the tag bits, set bits and block bits and store the type of address it is alongside it. 
        type_of_mem_access: String,
        tag_bits: String,
        set_bits: String,
    }

fn split_binary_address_into_type_t_s_and_b(vec_of_trace_file_input: Vec<String>, value_of_s: &String, value_of_b: &String)-> Result<Vec<BinaryInTagSetBlockParts>, &'static str>{
    if !vec_of_trace_file_input.is_empty(){
        let mut vec_of_binary_split_memory_addresses:Vec<BinaryInTagSetBlockParts>  = Vec::new();
        for line in vec_of_trace_file_input{
            let cloned_line = line.to_string();
            let input_tuple = turn_line_sep_vector_into_tuple(&cloned_line).unwrap();
            let cloned_binary = input_tuple.binary.to_string();
            let length_of_binary = cloned_binary.len();
            let value_of_s_as_num: usize = value_of_s.parse().unwrap();
            let value_of_b_as_num: usize = value_of_b.parse().unwrap();

            let block_bits_end_index = length_of_binary - value_of_b_as_num; 

            let binary_split_memory_address_to_input = BinaryInTagSetBlockParts{
                type_of_mem_access: input_tuple.tag.to_string(),
                tag_bits: cloned_binary[0..block_bits_end_index-value_of_s_as_num].to_string(), 
                set_bits: cloned_binary[block_bits_end_index-value_of_s_as_num..block_bits_end_index].to_string(),
            };


            // println!("Original full binary: {}, Tag bits: {}, Set bits: {}",&input_tuple.binary, &binary_split_memory_address_to_input.tag_bits, binary_split_memory_address_to_input.set_bits);

            vec_of_binary_split_memory_addresses.push(binary_split_memory_address_to_input);         
        }
        Ok(vec_of_binary_split_memory_addresses)
    }
    
    else{
        return Err("Vector is empty and therefore the for loop did not run")
    }

}

#[allow(unused)]
fn main() {

let args: Vec<String> = env::args().skip(1).collect(); //The skip here stops the executable binary being placed in the output


if args.len() < 8 || args.len() > 9 { //The max amount of arguments (if using the verbose and or helper flag) is 9 whereas the minimum is 8

    println!("Usage: [-hv] -s <s> -E <E> -b <b> -t <tracefile>"); //This just ensures the correct number of command line arguments are provided and provides the user with a usage guide if incorrect

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

let vec_of_trace_file = make_file_line_separated_vector(path_to_trace);

let mut vec_of_binary_split_memory_addresses = split_binary_address_into_type_t_s_and_b(vec_of_trace_file.clone(), value_of_s,value_of_b).unwrap();

let cache_sets: usize = 2_usize.pow(value_of_s.parse().unwrap()); //rows
let cache_lines: usize = value_of_E.to_string().parse().unwrap(); //columns add one as we need a column for the block stored within the cache 


    let mut my_cache = cache::ArrayRepresentationOfCache::new(value_of_s.parse().unwrap(), value_of_E.parse().unwrap(), cache_sets, cache_lines);

    
    if value_of_E == "1" {
        my_cache.create_two_d_array_with_index_if_dmc();
    }
    
    if value_of_s == "1" && value_of_E != "1"{
        my_cache.modify_two_d_array_to_be_correct_rows_and_correct_col_for_fully_associative();
    }



    for binary in vec_of_binary_split_memory_addresses{
        if value_of_E == "1"{ //This means direct mapped cache
            my_cache.dmc_process(binary.set_bits.clone(), binary.tag_bits.clone(), binary.type_of_mem_access.clone());
        }
        if value_of_s == "1" && value_of_E != "1"{//Fully associative as one set
           my_cache.insert_into_cache_if_fully_associative(binary.set_bits, binary.tag_bits, binary.type_of_mem_access);
        }
        if value_of_s != "1" && value_of_E != "1"{//Set associative as more than one set and more than one cache line
            println!("This is a set associative cache");
            //my_cache.modify_cache_structure_for_set_associative(my_cache.value_of_s_as_usize);
        } 
    }

    my_cache.print_array();
    my_cache.print_hits_misses_evictions();

}
    //To follow convention I will test the main.rs file here
    #[cfg(test)] //Basically tells rust to NOT compile the tests at the same time as compiling the rest of the code
    mod tests{
        use cache;
        use super::*;

        #[test]
        fn test_make_file_line_separated_vector_valid_filepath(){
            let line_sep_vec = make_file_line_separated_vector("src/traces/custom.trace");
            let correct_vec = vec![" S 00602264,1", " L 00602260,4", " M 7fefe059c,4", " L 7fefe0594,4", " L 7fefe059c,4", " L 7fefe059c,4"];
            assert_eq!(line_sep_vec, correct_vec);
            let line_sep_vec_tested_on_instructions = make_file_line_separated_vector("src/traces/customWITHI.trace");
            let correct_vec_2 = vec![" S 00602264,1", " L 00602260,4", " M 7fefe059c,4", " L 7fefe0594,4", " L 7fefe059c,4", " L 7fefe059c,4"]; //Should be the same as removed instructions
            assert_eq!(line_sep_vec_tested_on_instructions, correct_vec_2);
        }

        #[test]
        #[should_panic]
        fn test_make_file_line_separated_vector_invalid_filepath(){
            make_file_line_separated_vector("src/traces/DOESNOTEXIST/custom.trace"); //Will pass only if it DOES panic 

        }
    
        #[test]
        fn test_hex_to_binary() {
            let hex = "1a3f";
            let binary = convert_from_hex_to_binary(hex).unwrap();
            assert_eq!(binary, "0001101000111111");
        }

        #[test]
        fn test_hex_to_binary_invalid_input(){
            let invalid_hex = "z";
            match convert_from_hex_to_binary(invalid_hex){
                Ok(_) => panic!("Expected an error for invalid hex input, but got nada."),
                Err(e) => assert_eq!(e, "Hex character not valid", "Unexpected error message for invalid hex input.")
            }
        }

        #[test]
        fn test_turn_line_sep_vector_into_tuple() {
            let line = "  M 20,1";
    
            if let Ok(tuple) = turn_line_sep_vector_into_tuple(line) {
                assert_eq!(tuple.tag, "M");
            } else {
                panic!("Expected that the M would be equal to tuple.tag");
            }
        }

        #[test]
        fn test_turn_line_sep_vector_into_tuple_invalid_input_no_space() {
            let line = "I,20";
            assert!(turn_line_sep_vector_into_tuple(line).is_err(), "Although instructions are already supposedly filtered, if one sneaks through the program should crash here and - as such - this test should fail");
        }

        #[test]
        fn test_split_binary_address_into_type_t_s_and_b(){

            let vec_of_trace_file:Vec<String> = vec![" M 20,1".to_string()];
            let value_of_s:&String = &"2".to_string();
            let value_of_b:&String = &"2".to_string();
            let binary_split = split_binary_address_into_type_t_s_and_b(vec_of_trace_file, value_of_s, value_of_b).unwrap();

            //20 converted to binary from hex should be 0010 0000
            //Set bits should be 00
            //Tag bits should be 0010 (or first 4 bits)
            //Type of mem access should be M

            assert_eq!(binary_split[0].set_bits, "00");
            assert_eq!(binary_split[0].tag_bits, "0010");
            assert_eq!(binary_split[0].type_of_mem_access, "M");
        }

        #[test]
        fn test_split_binary_address_into_type_t_s_and_b_with_empty_vec(){

            let vec_of_trace_file:Vec<String> = vec![];
            let value_of_s:&String = &"2".to_string();
            let value_of_b:&String = &"2".to_string();

            match split_binary_address_into_type_t_s_and_b(vec_of_trace_file, value_of_s, value_of_b){
                Ok(_) => panic!("Expected an error for empty vec file, but got nada."),
                Err(e) => assert_eq!(e, "Vector is empty and therefore the for loop did not run", "Unexpected error message for invalid hex input.")
            }
        }

        //From this point I will try to test cache functionality by creating a cache
        #[test]
        fn test_cache_struct_array_creation_dmc(){

            let value_of_s:&String = &"2".to_string();
            let value_of_e:&String = &"1".to_string(); // A direct mapped cache 

            let my_test_cache = cache::ArrayRepresentationOfCache::new(value_of_s.parse().unwrap(), value_of_e.parse().unwrap(),4, 1);

            for line in my_test_cache.two_d_array{
                assert_eq!(line.len(), 2); //As the value_of_e is one the cache only has one line or row as well as one more to store the set index (as determined by the amount of set bits) 
            }
             
        }

        #[test]
        fn test_cache_struct_array_creation_fac(){

            let value_of_s:&String = &"1".to_string(); // A fully associative cache 
            let value_of_e:&String = &"3".to_string(); 

            let mut my_test_cache = cache::ArrayRepresentationOfCache::new(value_of_s.parse().unwrap(), value_of_e.parse().unwrap(),1,3);
            my_test_cache.modify_two_d_array_to_be_correct_rows_and_correct_col_for_fully_associative(); //Modifies array to be one continuous vector (or one set)

            for line in my_test_cache.two_d_array{
                println!("{:?}", line);
                assert_eq!(line.len(), 3); 
            }
             
        }

        #[test]
        fn test_cache_struct_output_dmc(){
            let value_of_s:&String = &"2".to_string();
            #[allow(non_snake_case)]
            let value_of_E:&String = &"1".to_string(); // A direct mapped cache 

            let value_of_b: &String = &"1".to_string();

            let mut my_test_cache = cache::ArrayRepresentationOfCache::new(value_of_s.parse().unwrap(), value_of_E.parse().unwrap(),4, 1);

            let vec:Vec<String> = vec![" S 00602264,1".to_string(), " L 00602260,4".to_string(), " M 7fefe059c,4".to_string(), " L 7fefe0594,4".to_string(), " L 7fefe059c,4".to_string(), " L 7fefe059c,4".to_string()];

            let vec_of_binary_split_memory_addresses = split_binary_address_into_type_t_s_and_b(vec, value_of_s, value_of_b).unwrap();

            for binary in vec_of_binary_split_memory_addresses{
                if value_of_E == "1"{ //This means direct mapped cache
                    my_test_cache.create_two_d_array_with_index_if_dmc();
                    my_test_cache.dmc_process(binary.set_bits.clone(), binary.tag_bits.clone(), binary.type_of_mem_access.clone());
                }
                if value_of_s == "1" && value_of_E != "1"{
                    my_test_cache.insert_into_cache_if_fully_associative(binary.set_bits, binary.tag_bits, binary.type_of_mem_access);
                }
            }
            assert_eq!(my_test_cache.cache_hits, 2);
            assert_eq!(my_test_cache.cache_misses, 5);
            assert_eq!(my_test_cache.cache_evictions, 3);
    }

    #[test]
    fn test_cache_struct_output_fac(){
        let value_of_s:&String = &"1".to_string();
        #[allow(non_snake_case)]
        let value_of_E:&String = &"5".to_string(); // A direct mapped cache 

        let value_of_b: &String = &"1".to_string();

        let mut my_test_cache = cache::ArrayRepresentationOfCache::new(value_of_s.parse().unwrap(), value_of_E.parse().unwrap(),1, 5);

        let vec:Vec<String> = vec![" S 00602264,1".to_string(), " L 00602260,4".to_string(), " M 7fefe059c,4".to_string(), " L 7fefe0594,4".to_string(), " L 7fefe059c,4".to_string(), " L 7fefe059c,4".to_string()];

        let vec_of_binary_split_memory_addresses = split_binary_address_into_type_t_s_and_b(vec, value_of_s, value_of_b).unwrap();

        for binary in vec_of_binary_split_memory_addresses{
            if value_of_E == "1"{ //This means direct mapped cache
                my_test_cache.create_two_d_array_with_index_if_dmc();
                my_test_cache.dmc_process(binary.set_bits.clone(), binary.tag_bits.clone(), binary.type_of_mem_access.clone());
            }
            if value_of_s == "1" && value_of_E != "1"{
                my_test_cache.insert_into_cache_if_fully_associative(binary.set_bits, binary.tag_bits, binary.type_of_mem_access);
            }
        }
        assert_eq!(my_test_cache.cache_hits, 3);
        assert_eq!(my_test_cache.cache_misses, 4);
        assert_eq!(my_test_cache.cache_evictions, 0);
        }

    }