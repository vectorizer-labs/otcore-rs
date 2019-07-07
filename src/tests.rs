extern crate rand;

// Note this useful idiom: importing names from outer (for mod tests) scope.
//use super::*;
use list::List;
use operation::Operation;
use std::time::{SystemTime};

use tests::rand::distributions::{Distribution, Uniform};

#[test]
fn simple_random_insert() 
{
    let now = SystemTime::now();
    let mut docString : List<char> = List::new();

    let mut rng = rand::thread_rng();

    //Uniform number generator excludes max
    //so by starting at 1 we guarantee the first index will be 0
    let mut indexMax : usize = 1;

    for i in 0..100
    {
        let time = match now.elapsed() 
        {
            Ok(elapsed) => { elapsed.as_secs() }
            Err(e) => { panic!("Error: {:?}", e); }
        };

        let is_insert : bool = true;
        let obj : char = rand::random::<char>();

        let idxRange = Uniform::from(0..indexMax);

        let O : Operation<char> = Operation::new(is_insert, obj, idxRange.sample(&mut rng), i,  1 /*user id */, time as usize);
        docString.add(O);

        indexMax+=1;
    }

    print!("{}",docString.getString());

    //assert_eq!(add(1, 2), 3);
}

#[test]
fn simple_random_insert_and_delete() 
{
    let now = SystemTime::now();

    let testOpCount = 10000;

    let mut docString : List<char> = List::with_capacity(testOpCount);

    let mut rng = rand::thread_rng();

    println!("Starting OP gen loop at: {}ms ", now.elapsed().unwrap().as_millis());

    //insert one character at the beginning to act as a buffer
    let OFirst : Operation<char> = Operation::new(true, rand::random::<char>(), 0, 0,  1 /*user id */, now.elapsed().unwrap().as_secs() as usize);
    docString.add(OFirst);

    for i in 1..testOpCount
    {
        let time = now.elapsed().unwrap().as_secs();

        let stringLength = docString.list.len();
        
        //leave a buffer of one character at all times
        let is_insert : bool = match  stringLength > 1
        {
            
            //it could be a delete but we're gonna rng to decide
            true => {rand::random::<bool>() }
            //we don't have any chars to delete
            //so return true for an insert
            false => { true }
        };

        let obj : char = rand::random::<char>();

        let idxRange : Uniform<usize> =  Uniform::from(0..stringLength);

        let O : Operation<char> = Operation::new(is_insert, obj, docString.getLogSpaceIndex(idxRange.sample(&mut rng)), i,  1 /*user id */, time as usize);
        
        docString.add(O);
    }

    println!("Loop finished at: {}ms ", now.elapsed().unwrap().as_millis());

    println!("{}",docString.getString());

    println!("Done at: {}ms ", now.elapsed().unwrap().as_millis());
}