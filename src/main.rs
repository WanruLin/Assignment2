use ndarray::Array2;
use std::{collections::BTreeMap};
use serde::{Serialize, Deserialize};
use std::time::Instant;
use std::io::Write;
use std::io::Read;
use rand::{Rng, thread_rng};

fn main() {

    println!("//////////////////////////////////random bv generate////////////////////////////////");
    let mut bv_list = Vec::new();
    for i in (100..=10000).step_by(100) {
        let bv:Vec<i32> = generate_random_bitvec(i); 
        bv_list.push(bv);
    }

    println!("//////////////////////////////////task1 start////////////////////////////////");
    let mut size_rs = Vec::new();
    let mut time_rs = Vec::new();
    for bv in bv_list.iter() {
        println!("bitvec: {:?}",bv);
        let rank_support = construct_rank_support(bv.clone());
        let n = bv.len() -1;
        println!("{}",n);
        let index = generate_random_index(100,1,n as i32);
        //start time 
        let start = Instant::now();
        for i in 0..index.len() {
            let i2 = index[i] as i64;
            //println!("index = {}",i2);
            println!("rank1({}) = {}", i,rank_support.rank1(i2));
            //rank_support.rank1(*i as i64);
            
        }
        //end time
        let end = Instant::now();
        let elapsed = end - start;
        time_rs.push(elapsed.as_secs_f32());
        let s = rank_support.overhead();
        size_rs.push(s);
    } 
    println!("time rank support: {:?}",time_rs);
    println!("size rank support: {:?}",size_rs);
    println!("//////////////////////////////////task1 end////////////////////////////////");

    println!("//////////////////////////////////task2 start////////////////////////////////");
    let mut size_ss = Vec::new();
    let mut time_ss = Vec::new();  
    for bv in bv_list.iter() {
        let mut select_support1 = construct_select_support(bv.clone());
        select_support1.select1(2);
        println!("bitvector:{:?}", bv);
        let sum_bv = bv.iter().fold(0, |acc, &x| acc + x) -2 ;
        let rank = generate_random_index(100,1, sum_bv);
        
        //start time 
        let start = Instant::now();
        for i in 0..rank.len() {
            let r = rank[i];
            //println!("rank:{}",r);
            select1_function(&mut select_support1, r as usize);
            select_support1.p = 0;
            select_support1.r = select_support1.bitvector.len() as i64;
            
        }
        //end time
        let end = Instant::now();
        let elapsed = end - start;
        time_ss.push(elapsed.as_secs_f32());
        let s = select_support1.overhead();
        size_ss.push(s);
    } 
    println!("time select support: {:?}",time_ss);
    println!("size select support: {:?}",size_ss);
    println!("//////////////////////////////////task2 end////////////////////////////////");

    
    println!("////////////////////////////////////////task3 start/////////////////////////////////////");
    //random sparse array generate
    let n:Vec<usize> = vec![1000,10000,100000,1000000];
    let sparsity:Vec<f32> = vec![1.0,5.0,10.0];
    let mut data_list = Vec::new();
    for i in 0..n.len(){
        for j in 0..sparsity.len() {
            let d = generate_vector(n[i], sparsity[j] as f32);
            data_list.push(d);
        }
    }
    //println!("{:?}",data_list);

    let mut indice_data = Vec::new();
    let mut indice_elem = Vec::new();
    let mut elem_list:Vec<Vec<String>>= Vec::new();
    for i in 0..data_list.len() {
        let indice:Vec<usize> = (0..data_list[i].len()).collect();
        indice_data.push(indice);
        let count1 = data_list[i].iter().fold(0, |acc, &x| acc + x) as usize;
        let indice2:Vec<usize> = (0..=count1).collect();
        indice_elem.push(indice2);
        let elem = generate_elems(count1);
        elem_list.push(elem);

    }

    let mut sparse_array_list:Vec<SparseArray> = Vec::new();

    for i in 0..data_list.len(){
        let mut sparse_array = SparseArray::create(data_list[i].len());
        sparse_array.data = (indice_data[i].clone()).into_iter().zip(data_list[i].clone()).collect();
        sparse_array.elements = (indice_elem[i].clone()).into_iter().zip(elem_list[i].clone()).collect();
        sparse_array_list.push(sparse_array);
    }

    let mut time_receptor = Vec::new();
    let mut size_receptor = Vec::new();

    for i in 0..sparse_array_list.len() {
        println!(">>>>>>>>> sparse array[{}]",i);
        let sparse_array = &mut sparse_array_list[i];
        sparse_array.num_elem();
        let count1_sparse_array_data = data_list[i].iter().fold(0, |acc, &x| acc + x) -2 ;
        println!("count1 is {}",count1_sparse_array_data);
        let rank = generate_random_index(10,1, count1_sparse_array_data);
        let indice = generate_random_index(10,1, data_list[i].len() as i32);

        //start time
        let start = Instant::now();
        for i in 0..10{
            let rnk = rank[i] as usize;
            let index = indice[i] as usize;
            println!("get_at_rank(r = {}) = {}", rnk, sparse_array.get_at_rank(rnk, "test_get_at_rank".to_string()));
            println!("get_at_index(i = {}) = {}", index, sparse_array.get_at_index(index, "test_get_at_index".to_string()));
            println!("get_index_of(r = {}) = {}", rnk, sparse_array.get_index_of(rnk));
            println!("num_elem_at(i = {}) = {}", index, sparse_array.num_elem_at(index));
            
        }
        //end time
        let end = Instant::now();
        let elapsed = end - start;
        time_receptor.push(elapsed.as_secs_f32());
        println!(">>>>>>>>running time of this SparseArray is {:?}",elapsed.as_secs_f32());
        let size = sparse_array.size();
        size_receptor.push(size);
        println!(">>>>>>>>size of this SparseArray is {:?}",sparse_array.size());

    }
    println!("time sparse array: {:?}",time_receptor);
    println!("size sparse array: {:?}",size_receptor);



    println!("////////////////////////////////////////task3 end/////////////////////////////////////");
    
    
    
    println!("///////////////////////compare to the size when all 0s were stored as 'empty'///////////////////////");
    let mut string_list: Vec<Vec<String>> = Vec::new();
    for i in n.clone() {
        for sp in 0..sparsity.len(){
            let string = generate_elems(i*(sparsity[sp] as usize)/100);
            string_list.push(string);
        }
    }
    for i in 0..string_list.len() {
        for _ in 0..(n[i/3]-string_list[i].len()){
            let empty = "emp".to_string();
            string_list[i].push(empty);

        }
    }
    
    let mut sparse_array_list2:Vec<SparseArray> = Vec::new();

    for i in 0..data_list.len(){
        let mut sparse_array = SparseArray::create(data_list[i].len());
        sparse_array.data = (indice_data[i].clone()).into_iter().zip(data_list[i].clone()).collect();
        sparse_array.elements = (indice_data[i].clone()).into_iter().zip(string_list[i].clone()).collect();
        sparse_array_list2.push(sparse_array);
    }

    for i in 0..sparse_array_list2.len() {
        let size = sparse_array_list2[i].size();
        println!(">>>>size is: {}",size);
    }


}


//////////////////////////////some useful functions for the evaluation//////////////////////////////

fn generate_random_index(size: usize,start: i32, end: i32) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut vec = Vec::with_capacity(100);
    for _ in 0..size {
        let num = rng.gen_range(start..=end);
        vec.push(num);
    }
    vec
}

fn generate_random_bitvec(size: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        let num = rng.gen_range(0..=1) as i32; // generate a random number 0 or 1
        vec.push(num);
    }
    vec
}

fn generate_elems(count: usize) -> Vec<String> {
    let original = &vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
    let mut repeated = vec![];
    for _ in 0..count {
        for s in original {
            repeated.push(s.clone());
        }
    }
    repeated
}


fn generate_vector(size: usize, percent_ones: f32) -> Vec<i32> {
    let num_ones = (size as f32 * percent_ones / 100.0).round() as usize;
    let mut vec = vec![0; size];
    let mut ones_count = 0;

    while ones_count < num_ones {
        let index = rand::random::<usize>() % size;
        if vec[index] == 0 {
            vec[index] = 1;
            ones_count += 1;
        }
    }

    vec
}



// all 3 tasks

//////////////////////////////////////////// task 1 Rank code ////////////////////////////////////////////

#[derive(Debug,Serialize,Deserialize)]
struct RankSupport{
    
    bitvector:Vec<i32>,
    n:i64,
    chunk_length:i64, 
    subchunk_length:i64, 
    cum_rank: Vec<usize>,
    relative_cum_rank: Vec<usize>,
    rank_look_up_table: BTreeMap<Vec<i64>, Vec<i64>>,
    //number_of_chunk:i64,
    //number_of_subchunk:i64,
    //lookup_table_rank
}

impl RankSupport {
    
    fn rank1(&self, index:i64) -> usize{
        let bv = &self.bitvector;
        let n = self.n;
        let chunk_length = self.chunk_length;
        let subchunk_length = self.subchunk_length;
        let cum_rank = &self.cum_rank;
        let re_cum_rank = &self.relative_cum_rank;
        let rank_look_up_table = &self.rank_look_up_table;
        
        let pattern = get_pattern(bv.clone(), index, subchunk_length, n);
        let mut value_vec = vec![];
        match rank_look_up_table.get(&pattern) {
            Some(value) => value_vec = value.to_vec(),
            None => println!("none"),
        }
        //println!("rank table for the pattern: {:?}",value_vec);
        let pattern_start_pos = (((index as f64)/(subchunk_length as f64)).floor() as i64)*(subchunk_length);
        let pos = index-pattern_start_pos;
        let rank_from_lookup = value_vec[pos as usize];
        //println!("{}",rank_from_lookup);
        let rank_cum = cum_rank[(index as f64/chunk_length as f64).floor() as usize];
        //println!("{}",rank_cum);
        let rank_relative = re_cum_rank[(index as f64/subchunk_length as f64).floor() as usize];
        //println!("{}",rank_relative);
        let mut rank = rank_from_lookup as usize+rank_cum+rank_relative;
        if bv[index as usize] == 1 {
            rank = rank-1;
        }
        rank
    }

    fn overhead(&self) -> usize {
        
        let size1 = std::mem::size_of_val(&self.bitvector)*self.bitvector.len();
        let size2 = std::mem::size_of_val(&self.n);
        let size3 = std::mem::size_of_val(&self.chunk_length);
        let size4 = std::mem::size_of_val(&self.subchunk_length);
        let size5 = std::mem::size_of_val(&self.cum_rank)*self.cum_rank.len();
        let size6 = std::mem::size_of_val(&self.relative_cum_rank)*self.relative_cum_rank.len();
        
        let mut size7 = std::mem::size_of_val(&self.rank_look_up_table);
        for (key, value) in &self.rank_look_up_table {
            size7 += std::mem::size_of_val(key)*key.len();
            size7 += std::mem::size_of_val(value)*value.len();
        }

        (size1+size2+size3+size4+size5+size6+size7)*8 //*8: size in bit
    }

    
    fn save(&self, fname: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(self)?;
        let mut file = std::fs::File::create(fname)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn load(fname: &str) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(fname)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let deserialized: Self = serde_json::from_str(&buffer)?;
        Ok(deserialized)
    }
    


}



fn construct_rank_support(bv: Vec<i32>) -> RankSupport{
    let bv_len:i64 = bv.len() as i64;
    let n_float = bv_len as f64;
    let tmp = ((n_float.log2().ceil() as i64).pow(2) as f64)*0.5;
    //println!("tmp:{}",tmp);
    let sc_l = (n_float.log2()*0.5).ceil() as i64;
    let tmp2 = ((tmp/(sc_l as f64)) as f64).ceil() as i64;
    //println!("tmp2:{}",tmp2);
    let c_l = sc_l*tmp2 ;

    
    let mut cum_rank:Vec<usize> = Vec::new();
    let num_c = (((bv_len as f64)/(c_l as f64))).ceil() as i64;
    //println!("{}",num_c);
    for j in 0..num_c{
        let rank = get_cum_rank(bv.clone(),(j)*c_l);
        cum_rank.push(rank);

    }

    let mut relative_cum_rank:Vec<usize> = Vec::new();
    let num_sc = (((bv_len as f64)/(sc_l as f64))).ceil() as i64;
    //println!("{}",num_sc);
    for k in 0..num_sc{
        let i = k*sc_l/c_l;
        let rank = get_cum_rank(bv.clone(), k*sc_l)-get_cum_rank(bv.clone(), i*c_l);
        relative_cum_rank.push(rank);
    }

    let patterns = get_look_up_table(sc_l as usize);
    
    
    let result = RankSupport{
        bitvector: bv,
        n: bv_len,
        chunk_length: c_l,
        subchunk_length: sc_l,
        cum_rank: cum_rank,
        relative_cum_rank: relative_cum_rank,
        rank_look_up_table: patterns,
        //number_of_chunk:num_c,
        //number_of_subchunk:num_sc,
    };
    result

}

fn get_cum_rank(bv:Vec<i32>, i:i64) -> usize {
    //let bv_slice = bv.as_bitslice();
    let bv_slice = &bv[..(i as usize)];
    let rank = bv_slice.iter().fold(0, |acc, &x| acc + x); // compute the sum of the first four bits
    rank as usize
}

fn get_look_up_table(n: usize) -> BTreeMap<Vec<i64>, Vec<i64>> {
    let digits = [0, 1];
    let num_combinations = 2usize.pow(n as u32);
    //println!("{}",num_combinations);
    let mut combinations = Array2::<i64>::zeros((num_combinations, n));
    let mut look_up_rank = Array2::<i64>::zeros((num_combinations, n));

    for i in 0..num_combinations {
        for j in 0..n {
            combinations[[i, j]] = digits[(i >> j) & 1];
        }
    }
    //println!("{:?}",combinations);

    for i in 0..num_combinations {
        for j in 0..n {
            //println!("{}",i/n);
            let row = combinations.row(i).to_vec();
            look_up_rank[[i,j]] = row[0..j+1].iter().sum();
        }
    }

    let mut look_up_rank_map = BTreeMap::new();
    for i in 0..num_combinations{
        let row_look_up = look_up_rank.row(i).to_vec();
        let row_patterns = combinations.row(i).to_vec();
        look_up_rank_map.insert(row_patterns.to_owned(), row_look_up.to_owned());
    }

    look_up_rank_map
    
    
}


fn get_pattern(bv:Vec<i32>,index:i64,subchunk_length:i64,n:i64) -> Vec<i64>{
    let pattern_start_pos = (((index as f64)/(subchunk_length as f64)).floor() as i64)*(subchunk_length);
    let pattern_stop_pos = (((index as f64)/(subchunk_length as f64)).floor() as i64 +1)*(subchunk_length);
    let mut _pattern = vec![0;subchunk_length as usize];
    if pattern_stop_pos>n {
        let bv_slice = &bv[(pattern_start_pos as usize)..(n as usize)];
        /*
        let end = (pattern_stop_pos-n+1) as usize;
        let mut p = bv_slice.to_owned();
        for i in end..subchunk_length as usize {
            p.set(i, false);
        } */
        _pattern = bv_slice.to_owned();
        for _i in 0..(subchunk_length as usize-_pattern.len() as usize){
            _pattern.push(0);

        }
    }else {
        let bv_slice = &bv[(pattern_start_pos as usize)..(pattern_stop_pos as usize)];
        _pattern = bv_slice.to_owned();
    }

    let mut pattern_vec = Vec::new();
    for bit in _pattern.iter() {
        if *bit == 1 {
            pattern_vec.push(1 as i64)
        }else if *bit == 0 {
            pattern_vec.push(0 as i64)
        } 
    }
    //println!("pattern: {:?}",pattern_vec);
    pattern_vec
    
}



fn rank1(rank_support:&RankSupport, index:i64) -> usize  {
    let bv = &rank_support.bitvector;
    
    let n = rank_support.n;
    let chunk_length = rank_support.chunk_length;
    let subchunk_length = rank_support.subchunk_length;
    let cum_rank = &rank_support.cum_rank;
    let re_cum_rank = &rank_support.relative_cum_rank;
    let rank_look_up_table = &rank_support.rank_look_up_table;

    let pattern = get_pattern(bv.clone(), index, subchunk_length, n);

    
    let mut value_vec = vec![];
    match rank_look_up_table.get(&pattern) {
        Some(value) => value_vec = value.to_vec(),
        None => println!("none"),
    }
    //println!("rank table for the pattern: {:?}",value_vec);

    let pattern_start_pos = (((index as f64)/(subchunk_length as f64)).floor() as i64)*(subchunk_length);
    let pos = index-pattern_start_pos;
    let rank_from_lookup = value_vec[pos as usize];
    //println!("{}",rank_from_lookup);
    
    let rank_cum = cum_rank[(index as f64/chunk_length as f64).floor() as usize];
    //println!("{}",rank_cum);
    let rank_relative = re_cum_rank[(index as f64/subchunk_length as f64).floor() as usize];
    //println!("{}",rank_relative);

    let mut rank = rank_from_lookup as usize+rank_cum+rank_relative;
    if bv[index as usize] == 1 {
        rank = rank-1;
    }
    rank

}


//////////////////////////////////////////// task 2 select code ////////////////////////////////////////////

#[derive(Debug,Serialize,Deserialize)]
struct SelectSupport{
    bitvector:Vec<i32>,
    n:i64,
    chunk_length:i64, 
    subchunk_length:i64, 
    cum_rank: Vec<usize>,
    relative_cum_rank: Vec<usize>,
    rank_look_up_table: BTreeMap<Vec<i64>, Vec<i64>>,
    p: usize,
    r: i64,
    //number_of_chunk:i64,
    //number_of_subchunk:i64,
    //lookup_table_rank
}

impl SelectSupport {
    fn select1(&mut self,rank: usize)  {
        //select1(self, rank);
        select1_function(self, rank);
        
    }
    
    fn overhead(&self) -> usize {
        
        let size1 = std::mem::size_of_val(&self.bitvector)*self.bitvector.len();
        let size2 = std::mem::size_of_val(&self.n);
        let size3 = std::mem::size_of_val(&self.chunk_length);
        let size4 = std::mem::size_of_val(&self.subchunk_length);
        let size5 = std::mem::size_of_val(&self.cum_rank)*self.cum_rank.len();
        let size6 = std::mem::size_of_val(&self.relative_cum_rank)*self.relative_cum_rank.len();
        
        let mut size7 = std::mem::size_of_val(&self.rank_look_up_table);
        for (key, value) in &self.rank_look_up_table {
            size7 += std::mem::size_of_val(key)*key.len();
            size7 += std::mem::size_of_val(value)*value.len();
        }
        
        let size8 = std::mem::size_of_val(&self.p);
        let size9 = std::mem::size_of_val(&self.r);

        (size1+size2+size3+size4+size5+size6+size7+size8+size9)*8 //*8: size in bit
    }

    
    fn save(&self, fname: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(self)?;
        let mut file = std::fs::File::create(fname)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn load(fname: &str) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(fname)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let deserialized: Self = serde_json::from_str(&buffer)?;
        Ok(deserialized)
    }
    

    


}

fn construct_select_support(bv: Vec<i32>) -> SelectSupport{
    let bv_len:i64 = bv.len().try_into().unwrap();
    let n_float = bv_len as f64;
    let tmp = ((n_float.log2().ceil() as i64).pow(2) as f64)*0.5;
    //println!("tmp:{}",tmp);
    let sc_l = (n_float.log2()*0.5).ceil() as i64;
    let tmp2 = ((tmp/(sc_l as f64)) as f64).ceil() as i64;
    //println!("tmp2:{}",tmp2);
    let c_l = sc_l*tmp2 ;

    
    let mut cum_rank:Vec<usize> = Vec::new();
    let num_c = (((bv_len as f64)/(c_l as f64))).ceil() as i64;
    //println!("{}",num_c);
    for j in 0..num_c{
        let rank = get_cum_rank(bv.clone(),(j)*c_l);
        cum_rank.push(rank);

    }

    let mut relative_cum_rank:Vec<usize> = Vec::new();
    let num_sc = (((bv_len as f64)/(sc_l as f64))).ceil() as i64;
    //println!("{}",num_sc);
    for k in 0..num_sc{
        let i = k*sc_l/c_l;
        let rank = get_cum_rank(bv.clone(), k*sc_l)-get_cum_rank(bv.clone(), i*c_l);
        relative_cum_rank.push(rank);
    }

    let patterns = get_look_up_table(sc_l as usize);
    
    let result = SelectSupport{
        bitvector: bv,
        n: bv_len,
        chunk_length: c_l,
        subchunk_length: sc_l,
        cum_rank: cum_rank,
        relative_cum_rank: relative_cum_rank,
        rank_look_up_table: patterns,
        p: 0,
        r: bv_len,
        //number_of_chunk:num_c,
        //number_of_subchunk:num_sc,
    };
    result

}


fn select1_function(select_support: &mut SelectSupport,rank: usize) -> Option<i64> {
    let mut _p = select_support.p as i64;
    let mut _r = select_support.r - 1;
    //println!("p is {}",_p);
    //println!("r is {}",_r);
    let h = _p + (_r-_p)/2;
    //println!("h is {}",h);
    let bv = &select_support.bitvector;
    let cum_rank = &select_support.cum_rank;
    let relative_cum_rank = &select_support.relative_cum_rank;
    let rank_look_up_table = &select_support.rank_look_up_table;


    let rank_support = RankSupport{
        bitvector: bv.clone(),
        n: select_support.n,
        chunk_length: select_support.chunk_length,
        subchunk_length: select_support.subchunk_length,
        cum_rank: cum_rank.to_vec(),
        relative_cum_rank: relative_cum_rank.to_vec(),
        rank_look_up_table: rank_look_up_table.to_owned(),
    };

    let rank_half = rank1(&rank_support, h);
    let mut to_return = None;
    let mut _index = 1;
    //println!("h is {}",h);
    //println!("rank_half is {}",rank_half);
    if bv[h as usize] == 1 && rank_half == rank {
        to_return = Some(h);
    }else {
        if rank_half == rank+1 {
        
            for i in h.._r {
                
                if bv[i as usize] == 1 {
                    _index = i;
                    println!("select1({}) = {}",rank,i);
                    to_return =  Some(_index);
                    //return i;
                    
                    break;
                    //return i;
                }
            
            }
            
    
        }else if rank_half > rank+1 {
            //println!(">");
            _r = h;
            select_support.r = _r;
            select1_function(select_support, rank);
    
            //return _index;
    
        }else if rank_half < rank+1 {
            //println!("<");
            _p = h ;
            select_support.p = _p as usize;
            select1_function(select_support, rank);
    
            //return _index;
        }
    }
    
    //return _index;
    //_index
    //println!("to_return: {:?}", to_return);

    to_return

}


//////////////////////////////////////////// task 3 code ////////////////////////////////////////////

#[derive(Debug,Serialize,Deserialize)]
struct SparseArray {
    data: Vec<(usize, i32)>, // A vector of tuples containing index-value pairs
    size: usize,
    //rank_support: RankSupport,
    elements: Vec<(usize, String)>,
    finalized: bool,
}

impl SparseArray {
    fn create(size: usize) -> Self {
        SparseArray {
            data: Vec::new(),
            size,
            elements: Vec::new(),
            finalized: false,
        }
    }

    /*
    fn append_01(&mut self, index: usize, value: i32) {
        if self.finalized {
            panic!("Sparse array has been finalized");
        }

        if index >= self.size {
            panic!("Index out of bounds");
        }

        if value == 0 {
            self.data.retain(|&(i, _)| i != index); // Remove any existing element at the given index
        } else {
            // Add or update the element at the given index
            let index_value_pair = (index, value);
            if let Some(existing_index) = self.data.iter().position(|&(i, _)| i == index) {
                self.data[existing_index] = index_value_pair;
            } else {
                self.data.push(index_value_pair);
            }
        }
    }
    */

    fn append(&mut self, index: usize, elem: String) {
        /*
        if self.finalized {
            panic!("Sparse array has been finalized");
        }
         */

        if index >= self.size {
            panic!("Index out of bounds");
        }

        //self.elements.push((index, elem));
        match self.elements.iter_mut().find(|(i, _)| *i == index) {
            Some(name_entry) => {
                // Update existing name entry
                name_entry.1 = elem;
            }
            None => {
                // Append new name entry
                self.elements.push((index, elem));
            }
        }
    }

    fn get(&self, index: usize) -> i32 {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        self.data
            .iter()
            .find(|&&(i, _)| i == index)
            .map(|&(_, v)| v)
            .unwrap_or(0)
    }

    /*
    fn get_element(&self, index: usize) -> Option<String> {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        self.elements
            .iter()
            .find(|&&(i, _)| i == index)
            .map(|(_, v)| v.clone())

            
    }
     */

    fn finalize(&mut self) -> RankSupport {
        self.finalized = true;

        let mut bv = Vec::new();
        
        for i in 0..self.size{
            bv.push(self.get(i));
        }
        let rank_support = construct_rank_support(bv);
        rank_support
    }

    fn get_at_rank(&mut self, r:usize, elem: String) -> bool {
        let mut count1 = vec![];
        for i in 0..self.size {
            if self.get(i) == 1 {
                count1.push(i);
            }
        }
        if  r > count1.len() {
            return false;
        }else {
            self.append(r, elem);
            return true;
        }

    }

    fn get_at_index(&mut self,r:usize, elem:String) -> bool {
        let rank_support = self.finalize();
        
        if self.get(r)  == 1{
            let index = rank_support.rank1(r as i64);
            self.append(index, elem);

            return true;

        }else {
            return false;
        }
    }

    fn get_index_of(&mut self, r:usize) -> i64{
        let rank_support = self.finalize();

        let mut select_support = SelectSupport{
            bitvector: rank_support.bitvector,
            n: rank_support.n,
            chunk_length: rank_support.chunk_length,
            subchunk_length: rank_support.subchunk_length,
            cum_rank: rank_support.cum_rank,
            relative_cum_rank: rank_support.relative_cum_rank,
            rank_look_up_table: rank_support.rank_look_up_table,
            p: 0,
            r: rank_support.n,
            //number_of_chunk:num_c,
            //number_of_subchunk:num_sc,
        };

        let mut count1 = vec![];
        for i in 0..self.size {
            if self.get(i) == 1 {
                count1.push(i);
            }
        }

        if r > count1.len() {
            return i64::MAX;
        }else {
            let i = select1_function(&mut select_support, r);
            let mut result = 0;
            match i {
                Some(value) => {result = value; },
                None => println!("None"),
            }
            return result;
        }
        
    }

    fn num_elem_at(&mut self, r:usize) -> usize {
        let rank_support = self.finalize();
        let index = rank_support.rank1(r.try_into().unwrap()) ;
        
        if self.get(r)==1 {
            return index+1;
        }else {
            return index;
        }    
    }

    fn size(&self) -> usize{
        let size1 = std::mem::size_of_val(&self.data)*self.data.len();
        let size2 = std::mem::size_of_val(&self.size);
        let size3 = std::mem::size_of_val(&self.elements)*self.elements.len();
        let size4 = std::mem::size_of_val(&self.finalized);

        (size1+size2+size3+size4)*8
    }

    fn num_elem(&self) -> usize {
        let mut count1 = vec![];
        for i in 0..self.size {
            if self.get(i) == 1 {
                count1.push(i);
            }
        }
        count1.len()
    }
}
