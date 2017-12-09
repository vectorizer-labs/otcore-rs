#[cfg(test)]
#[allow(dead_code)]



extern crate rand;
use doc_state::DocState;
use operation::Operation;
use test_string_utils;

pub const TEST_SIZE : usize = 100;

#[test]
//tests get_string() for doc_state
fn doc_state_get_string() 
{
    let test_string = test_string_utils::rand_string(TEST_SIZE);
    let test_state = DocState::new(test_string.clone());
    assert_eq!(test_string, test_state.get_string(),
        "

        Starting String : {}

        Get String : {}

        ",
        test_string,
        test_state.get_string());
}

#[test]
//tests insertion for doc_state
fn doc_state_add_insertion()
{   
    let test_operation = test_string_utils::rand_insert_op(TEST_SIZE - 1);
    
    //generate the first half of the doc
    let first_half = test_string_utils::rand_readable_string(test_operation.get_index().clone() as usize);
    
    //generate the latter portion of the doc after the insert
    let second_half = test_string_utils::rand_readable_string((TEST_SIZE - test_operation.get_index().clone()) as usize);
    
    //generate the before string thats fed to the doc_state constructor
    let mut starting_doc_state_string : String = first_half.clone();
    starting_doc_state_string.push_str(&second_half.clone());
    
    
    let mut test_state = DocState::new(starting_doc_state_string);
    //println!("{} \n\n\n\n\n\n",test_operation.get_readable_representation());

    test_state.add(test_operation.clone());

    let after_string = test_state.get_string();
    
    let mut expected_string : String = first_half.clone();
    
    expected_string.push(test_operation.get_char().clone());
    expected_string.push_str(&second_half.clone());
    
    assert_eq!(expected_string, after_string,"Expected : '{}' \n\nAfter  : '{}'\n\n", expected_string, after_string);
}