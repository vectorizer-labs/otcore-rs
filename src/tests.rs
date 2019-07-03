#[cfg(test)]

// Note this useful idiom: importing names from outer (for mod tests) scope.
//use super::*;
use super::doc_state::DocState;
use super::operation::Operation;
use std::time::{Duration, SystemTime};


#[test]
fn simple_random_insert() 
{
    let now = SystemTime::now();
    let mut dc : DocState<char> = DocState::new();

    let mut rng = rand::thread_rng();

    for i in 0..100
    {
        let time = match now.elapsed() 
        {
            Ok(elapsed) => { elapsed.as_secs() }
            Err(e) => { println!("Error: {:?}", e); (0) }
        };


        let is_insert : bool = true;
        let obj : char = 'h';
        //let ix = 0;

        let O : Operation<char> = Operation::new(is_insert, obj, i, i, 1, time as usize);
        dc.add(O);
    }

    print!("{}",dc.string);

    //assert_eq!(add(1, 2), 3);
}

#[test]
fn test_bad_add() 
{
    // This assert would fire and test will fail.
    // Please note, that private functions can be tested too!
    //assert_eq!(bad_add(1, 2), 3);
}