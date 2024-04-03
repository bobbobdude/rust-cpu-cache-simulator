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


#[allow(non_snake_case)]
fn calculate_cache_size(s: &String, E:&String, b:&String) -> u32{ //This dynamically calculates how many BYTES the combined cache is
    let int_of_s: u32= s.parse().unwrap(); //This is the amount of Sets 
    let int_of_E: u32= E.parse().unwrap(); //This is the amount of Cache Lines per set 
    let int_of_b: u32= b.parse().unwrap(); //This is the block offset (the amount of bits that determine which byte to take from the requested block)
    let cache_bytes_size: u32 = (2_u32.pow(int_of_s)) * (2_u32.pow(int_of_b)) * int_of_E;
    cache_bytes_size
}

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
    hex_address: &'a str,
    size: &'a str,
    binary: String
}

fn turn_line_sep_vector_into_tuple(line_of_vec: &str)-> Result<TupleOfTagAndAddress, &'static str>{ //if first_byte != 32{ remember the below function should only run if this if statement is satisfied as 32 is equivalent to a space 
    let first_byte:i32 = line_of_vec.chars().next().unwrap() as i32;
    if first_byte == 32{
        let index_of_comma = line_of_vec.find(",").unwrap_or(line_of_vec.len()); //This tries to find the comma and if it fails it instead returns the length of the string 
        let index_of_size = index_of_comma + 1;
        let line_of_vec_with_size_removed: &str = &line_of_vec[0..index_of_comma];
        let split_instruction: Vec<&str> = line_of_vec_with_size_removed.split_whitespace().collect();
        let tag_address = TupleOfTagAndAddress{
            tag: split_instruction[0],
            hex_address: split_instruction[1],
            size: &line_of_vec[index_of_size..],
            binary: convert_from_hex_to_binary(split_instruction[1]).unwrap() 
        };
        Ok(tag_address)
    }
        else{
            return Err("First character is not a space and therefore it is ignored")
        }
    } 
    




let cache_bytes_size = calculate_cache_size(value_of_s, value_of_E, value_of_b);
let vec_of_trace_file = make_file_line_separated_vector(path_to_trace);


struct BinaryInTagSetBlockParts<>{//Okay so we need to split the binary address into the tag bits, set bits and block bits and store the type of address it is alongside it. 
    type_of_mem_access: String,
    tag_bits: String,
    set_bits: String,
    size: String
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

            let mut binary_split_memory_address_to_input = BinaryInTagSetBlockParts{
                type_of_mem_access: input_tuple.tag.to_string(),
                tag_bits: cloned_binary[0..block_bits_end_index-value_of_s_as_num].to_string(), 
                set_bits: cloned_binary[block_bits_end_index-value_of_s_as_num..block_bits_end_index].to_string(), 
                size: input_tuple.size.to_string()
            };

            // println!("Original full binary: {}, Original hex address: {}, Tag bits: {}, Set bits: {}",&input_tuple.binary, &input_tuple.hex_address, &binary_split_memory_address_to_input.tag_bits, binary_split_memory_address_to_input.set_bits);

            vec_of_binary_split_memory_addresses.push(binary_split_memory_address_to_input);         
        }
        Ok(vec_of_binary_split_memory_addresses)
    }
    
    else{
        return Err("Vector is empty and therefore the for loop did not run")
    }

}

let mut vec_of_binary_split_memory_addresses = split_binary_address_into_type_t_s_and_b(vec_of_trace_file.clone(), value_of_s,value_of_b).unwrap();
    //Okay so to represent the cache sets, and the amount of cache lines within those sets I have decided to create a fixed size 2d array. 


    struct ArrayRepresentationOfCache{
        value_of_s_as_usize: usize,
        value_of_e_as_usize: usize,
        rows_or_cache_sets: usize,
        cols_or_cache_lines: usize,
        two_d_array: Vec<Vec<String>>,
        cache_hits: i32,
        cache_misses: i32, 
        cache_evictions: i32
    }

    //https://rust-unofficial.github.io/patterns/idioms/ctor.html
    
    impl ArrayRepresentationOfCache{ 
        fn new(value_of_s_as_usize: usize, value_of_e_as_usize: usize, rows_or_cache_sets: usize, cols_or_cache_lines: usize)-> Self{//Equivalent of constructor in Java
            let two_d_array = vec![vec!["empty".to_string();cols_or_cache_lines + 1];rows_or_cache_sets]; //Plus 1 as we need to account for the extra initial block and the set bits 
            let initial_value_of_all_counters = 0;
            Self {value_of_s_as_usize, value_of_e_as_usize, rows_or_cache_sets, cols_or_cache_lines, two_d_array, cache_hits:initial_value_of_all_counters, cache_misses:initial_value_of_all_counters, cache_evictions:initial_value_of_all_counters}
        }
    }

    impl ArrayRepresentationOfCache{
        fn print_array(&self){
            for vec in &self.two_d_array{
                println!("{:?}", vec)
            }
            println!()
        }
    }

    impl ArrayRepresentationOfCache{ 
        fn dmc_process(&mut self, set_bits: String, tag_bits: String, type_of_instruction: String){ //returns none if the set is not found, resulting in a certain cache miss. 
            let mut index = usize::from_str_radix(&set_bits, 2).unwrap();
            // println!("{}", index);
            if self.two_d_array[index][1] == "empty"{
                // println!("Gets into first if statement");
                self.cache_misses += 1;
                self.two_d_array[index][1] = tag_bits;
                if type_of_instruction == "M"{
                    self.cache_hits +=1;
                }
            }
            else if self.two_d_array[index][1] != "empty"{
                // println!("Gets into else if statement");
                if tag_bits == self.two_d_array[index][1]{
                    self.cache_hits += 1;
                    if type_of_instruction == "M"{
                        self.cache_hits += 1;
                    }
                }
                else{
                    // println!("Gets into final else statement");
                    self.cache_misses += 1;
                    self.cache_evictions += 1;
                    self.two_d_array[index][1] = tag_bits;
                    if type_of_instruction == "M"{
                        self.cache_hits += 1;
                    }
                }
            }
        } 
    }

    impl ArrayRepresentationOfCache{
        fn create_two_d_array_with_index_if_dmc(&mut self){
            let size_of_set_bits = self.value_of_s_as_usize;
            // println!("{}", num_bits)
            let amount_of_different_indexes = self.rows_or_cache_sets;


            let mut create_vec_of_dec_int_to_add = vec![];
            for i in 0..amount_of_different_indexes.try_into().unwrap(){
                create_vec_of_dec_int_to_add.push(i)
            }
            // println!("{:?}", create_vec_of_dec_int_to_add);

            let mut vec_of_final_bits = vec![];
            let bits_needed = size_of_set_bits;

            for dec_int in create_vec_of_dec_int_to_add{
                let binary_string = format!("{:0bits$b}", dec_int, bits = bits_needed.try_into().unwrap());
                vec_of_final_bits.push(binary_string);
            }

            // println!("{:?}", vec_of_final_bits);

            let mut iterator = 0; 

            for mut vec in &mut self.two_d_array{
                vec[0] = vec_of_final_bits[iterator].clone();
                iterator += 1;
            }   

    }
}   

impl ArrayRepresentationOfCache{
    fn has_cache_got_empty_tag_fully_associative(&self) -> bool{
        
        for (index, element) in self.two_d_array[0].iter().enumerate(){
            if element.to_string() == "empty".to_string(){
                return true;
                
            }
        }
        return false;
    }
}

impl ArrayRepresentationOfCache{
    fn is_tag_in_cache_fully_associative(&self, block_id: String)-> Option<usize>{
        for (index, element) in self.two_d_array[0].iter().enumerate(){
            if element.to_string() == block_id.to_string(){
                return Some(index);
            }
        }
        return None;
    }
}

impl ArrayRepresentationOfCache{
    fn modify_two_d_array_to_be_correct_rows_and_correct_col_for_fully_associative(&mut self){
        if self.two_d_array.len() > 1{
            self.two_d_array.pop();
        }
        self.two_d_array[0].remove(0);
    }
}




impl ArrayRepresentationOfCache {
    fn insert_into_cache_if_fully_associative(&mut self, set_bits: String, tag_bits: String, type_of_instruction: String) {
        let mut cache_hit = false;
        let full_block_id = tag_bits + &set_bits;
        if type_of_instruction == "M"{
            self.cache_hits += 1;
        }
        let has_cache_got_empty_tags = self.has_cache_got_empty_tag_fully_associative();
        let is_block_id_in_cache = self.is_tag_in_cache_fully_associative(full_block_id.clone());

        if is_block_id_in_cache.is_some(){ //Covers situation when block_id is already in the cache and therefore all the block addresses are as well
            self.cache_hits += 1;
            let index_of_block_id = is_block_id_in_cache.unwrap();
            self.two_d_array[0].remove(index_of_block_id);
            self.two_d_array[0].insert(0, full_block_id.clone());
            
            
            return;
        }

        else if has_cache_got_empty_tags == true && is_block_id_in_cache.is_none(){
            self.cache_misses += 1;
            self.two_d_array[0].pop();
            self.two_d_array[0].insert(0, full_block_id).clone();
            return;
        }

        else if has_cache_got_empty_tags == false && is_block_id_in_cache.is_none(){
            // println!("This is the block_id when evicting: {}", full_block_id);
            self.cache_evictions += 1;
            self.cache_misses+=1;
            self.two_d_array[0].pop();
            self.two_d_array[0].insert(0, full_block_id.clone());
            return;
        }

        

        }
    }


let cache_sets: usize = 2_usize.pow(value_of_s.parse().unwrap()); //rows
let cache_lines: usize = value_of_E.to_string().parse().unwrap(); //columns add one as we need a column for the block stored within the cache 


    let mut test_of_cache_struct = ArrayRepresentationOfCache::new(value_of_s.parse().unwrap(), value_of_E.parse().unwrap(), cache_sets, cache_lines);

    // for instructions in vec_of_trace_file.clone(){
    //     println!("{}",instructions);
    // }
    
    // for binary in vec_of_binary_split_memory_addresses{
    //     let if_it_is_index_this_is_some = test_of_cache_struct.is_set_in_cache(binary.set_bits.clone(), binary.tag_bits.clone());
    //     println!("These are the set bits and tag bits we are looking for {}, {}", binary.set_bits, binary.tag_bits);
    //     if if_it_is_index_this_is_some.is_some(){
    //         let index_of_vector_where_set_found = if_it_is_index_this_is_some.unwrap();
    //         test_of_cache_struct.is_tag_in_cache(&binary.tag_bits.to_string(), &binary.set_bits.to_string(), index_of_vector_where_set_found);
    //     }
    //     println!();
    //     test_of_cache_struct.print_array();
    // }

    // for binary in vec_of_binary_split_memory_addresses{
    //     println!("Tag bits in address: {}",binary.tag_bits);
    //     println!("Set bits in address: {}",binary.set_bits);
    //     test_of_cache_struct.create_vector_with_blocks_after_tag_bits(binary.tag_bits, binary.set_bits);
    // }

    // for binary in vec_of_binary_split_memory_addresses{
        
    //     println!("Tag bits in address: {}",binary.tag_bits);
    //     println!("Set bits in address: {}",binary.set_bits);

    // }

    if value_of_s == "1" && value_of_E != "1"{
        test_of_cache_struct.modify_two_d_array_to_be_correct_rows_and_correct_col_for_fully_associative();
    }

    for binary in vec_of_binary_split_memory_addresses{
        if value_of_E == "1"{ //This means direct mapped cache
            test_of_cache_struct.create_two_d_array_with_index_if_dmc();
            test_of_cache_struct.dmc_process(binary.set_bits.clone(), binary.tag_bits.clone(), binary.type_of_mem_access.clone());
        }
        if value_of_s == "1" && value_of_E != "1"{
           test_of_cache_struct.insert_into_cache_if_fully_associative(binary.set_bits, binary.tag_bits, binary.type_of_mem_access);
        }
        // println!("This is what the cache looks like at this stage: ");
        // test_of_cache_struct.print_array();

    }

    // test_of_cache_struct.print_array();




    // test_of_cache_struct.create_vector_with_blocks_after_tag_bits("00001111".to_string(), "1111".to_string());

    println!("hits:{} misses:{} evictions:{}", test_of_cache_struct.cache_hits, test_of_cache_struct.cache_misses, test_of_cache_struct.cache_evictions);
    
    

    // test_of_cache_struct.print_array();
}