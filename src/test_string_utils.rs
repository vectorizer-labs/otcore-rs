#![allow(dead_code)]
extern crate rand;
    
use self::rand::distributions::IndependentSample;
use operation::Operation;
use test;
use std::char;  

//generates a new random insert operation
pub fn rand_insert_op() -> Operation
{
    //initialize random
    let mut rng = rand::thread_rng();
    
    //op values
    let test_char = rand_readable_char();

    let test_string_length_range = rand::distributions::Range::new(0, test::TEST_SIZE-1);
    let test_ix = test_string_length_range.ind_sample(&mut rng);

    println!("Index : {}", test_ix);

    let op_val_range = rand::distributions::Range::new(0, u32::max_value());
    let test_op_id = op_val_range.ind_sample(&mut rng) as usize;
    let test_user_id = op_val_range.ind_sample(&mut rng) as usize;

    Operation::new(true, test_char, test_ix, test_op_id, test_user_id,0)
}

//generates a new random insert operation
pub fn rand_remove_op() -> Operation
{
    //initialize random
    let mut rng = rand::thread_rng();

    //op values
    let test_char = rand_readable_char();

    let test_string_length_range = rand::distributions::Range::new(0, test::TEST_SIZE-1);
    let test_ix = test_string_length_range.ind_sample(&mut rng);

    let op_val_range = rand::distributions::Range::new(0, u32::max_value());
    let test_op_id = op_val_range.ind_sample(&mut rng) as usize;
    let test_user_id = op_val_range.ind_sample(&mut rng) as usize;

    Operation::new(false, test_char, test_ix, test_op_id, test_user_id,0)
}

//Generates a char over the full range of Unicode support (support for the full 32 bits)
pub fn rand_string(size : usize) -> String 
{
    (0..size).map(|_| rand_char()).collect()
}

//Generates a readable char for testing sanity
fn rand_readable_char() -> char
{
    //initialize random
    let mut rng = rand::thread_rng();

    let test_char_range = rand::distributions::Range::new(0, 254/*Magic number MAX ASCII*/);
    let test_char = test_char_range.ind_sample(&mut rng);
    match char::from_u32(test_char)
    {
        Some(ch) => return ch,
        None => panic!("Couldn't parse randomly generated char!")
    }
}

//Generates a char over the full range of Unicode support (support for the full 32 bits)
fn rand_char() -> char
{
    //initialize random
    let mut rng = rand::thread_rng();

    let test_char_range = rand::distributions::Range::new(0, 1114111/*Magic number MAX UNICODE POINT*/);
    let test_char = test_char_range.ind_sample(&mut rng) as u32;
    match char::from_u32(test_char)
    {
        Some(ch) => return ch,
        None => panic!("Couldn't parse randomly generated char!")
    }
}

//Generates a readable String for testing sanity
pub fn rand_readable_string(size : usize) -> String
{
    (0..size).map(|_| rand_readable_char()).collect()
}