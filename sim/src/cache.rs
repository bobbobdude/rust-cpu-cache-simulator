pub struct ArrayRepresentationOfCache{
    pub value_of_s_as_usize: usize,
    pub value_of_e_as_usize: usize,
    pub rows_or_cache_sets: usize,
    pub cols_or_cache_lines: usize,
    pub two_d_array: Vec<Vec<String>>,
    pub cache_hits: i32,
    pub cache_misses: i32, 
    pub cache_evictions: i32
}

//https://rust-unofficial.github.io/patterns/idioms/ctor.html

impl ArrayRepresentationOfCache{ 
    pub fn new(value_of_s_as_usize: usize, value_of_e_as_usize: usize, rows_or_cache_sets: usize, cols_or_cache_lines: usize)-> Self{//Equivalent of constructor in Java
        let two_d_array = vec![vec!["empty".to_string();cols_or_cache_lines + 1];rows_or_cache_sets]; //Plus 1 as we need to account for the extra initial block and the set bits 
        let initial_value_of_all_counters = 0;
        Self {value_of_s_as_usize, value_of_e_as_usize, rows_or_cache_sets, cols_or_cache_lines, two_d_array, cache_hits:initial_value_of_all_counters, cache_misses:initial_value_of_all_counters, cache_evictions:initial_value_of_all_counters}
    }
}

impl ArrayRepresentationOfCache{
    pub fn print_array(&self){
        for vec in &self.two_d_array{
            println!("{:?}", vec)
        }
        println!()
    }
}

impl ArrayRepresentationOfCache{
    pub fn print_hits_misses_evictions(&self){
        //hits:4 misses:5 evictions:3 for example 
        println!("hits:{} misses:{} evictions:{}", self.cache_hits, self.cache_misses, self.cache_evictions);

    }
}

impl ArrayRepresentationOfCache{ 
    pub fn dmc_process(&mut self, set_bits: String, tag_bits: String, type_of_instruction: String){ //returns none if the set is not found, resulting in a certain cache miss. 
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
    pub fn create_two_d_array_with_index_if_dmc(&mut self){
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
pub fn has_cache_got_empty_tag_fully_associative(&self) -> bool{
    
    for (index, element) in self.two_d_array[0].iter().enumerate(){
        if element.to_string() == "empty".to_string(){
            return true;
            
        }
    }
    return false;
}
}

impl ArrayRepresentationOfCache{
pub fn is_tag_in_cache_fully_associative(&self, block_id: String)-> Option<usize>{
    for (index, element) in self.two_d_array[0].iter().enumerate(){
        if element.to_string() == block_id.to_string(){
            return Some(index);
        }
    }
    return None;
}
}

impl ArrayRepresentationOfCache{
pub fn modify_two_d_array_to_be_correct_rows_and_correct_col_for_fully_associative(&mut self){
    if self.two_d_array.len() > 1{
        self.two_d_array.pop();
    }
    self.two_d_array[0].remove(0);
}
}




impl ArrayRepresentationOfCache {
pub fn insert_into_cache_if_fully_associative(&mut self, set_bits: String, tag_bits: String, type_of_instruction: String) {
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
        self.two_d_array[0].insert(0, full_block_id.clone());
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
