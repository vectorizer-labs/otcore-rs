#[cfg(test)]
#[allow(dead_code)]

extern crate rand;
use doc_state::DocState;
use test_string_utils;

pub const TEST_SIZE : usize = 100;

#[test]
//tests the doc_state constructor and get_string()
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

#[test]
fn doc_state_add_remove()
{
    let test_operation = test_string_utils::rand_remove_op(TEST_SIZE - 1);
    //println!("{} \n\n\n\n\n\n",test_operation.get_readable_representation());
    
    //generate the first half of the doc
    let first_half = test_string_utils::rand_readable_string(test_operation.get_index().clone() as usize);
    
    //generate the latter portion of the doc after the insert
    let second_half = test_string_utils::rand_readable_string((TEST_SIZE - test_operation.get_index().clone()) as usize);
    
    //generate the before string thats fed to the doc_state constructor
    let mut starting_doc_state_string : String = first_half.clone();
    starting_doc_state_string.push(test_operation.get_char().clone());
    starting_doc_state_string.push_str(&second_half.clone());
    
    let mut test_state = DocState::new(starting_doc_state_string.clone());
    
    test_state.add(test_operation.clone());
    
    let after_string = test_state.get_string();
    
    let mut expected_string : String = first_half.clone();
    expected_string.push_str(&second_half.clone());
    
    //println!("Starting: '{}'\nResult  : '{}' ",starting_doc_state_string,after_string);
    
    assert_eq!(expected_string, after_string,"Expected : '{}' \n\nAfter  : '{}'\n\n", expected_string, after_string);
}

#[test]
fn remove_insertion_operation()
{
    let test_operation = test_string_utils::rand_insert_op(TEST_SIZE - 1);
    
    //generate an arbitrary test string
    let test_string = test_string_utils::rand_readable_string(test_operation.get_index().clone() as usize);
    
    let mut test_state = DocState::new(test_string.clone());
    
    test_state.add(test_operation.clone());
    
    test_state.remove(test_operation.clone());
    
    let after_string = test_state.get_string();
    
    assert_eq!(test_string, after_string,"Expected : '{}' \n\nAfter  : '{}'\n\n", test_string, after_string);
    
}

#[test]
fn remove_remove_operation()
{
    let test_operation = test_string_utils::rand_remove_op(TEST_SIZE - 1);
    println!("{} \n\n",test_operation.get_readable_representation());
    
    //generate the first half of the doc
    let first_half = test_string_utils::rand_readable_string(test_operation.get_index().clone() as usize);
    
    //generate the latter portion of the doc after the insert
    let second_half = test_string_utils::rand_readable_string((TEST_SIZE - test_operation.get_index().clone()) as usize);
    
    //generate the before string thats fed to the doc_state constructor
    let mut starting_doc_state_string : String = first_half.clone();
    starting_doc_state_string.push(test_operation.get_char().clone());
    starting_doc_state_string.push_str(&second_half.clone());
    
    let mut test_state = DocState::new(starting_doc_state_string.clone());
    
    test_state.add(test_operation.clone());
    
    test_state.remove(test_operation.clone());
    
    let after_string = test_state.get_string();
    
    //println!("Starting: '{}'\nResult  : '{}' ",starting_doc_state_string,after_string);
    
    assert_eq!(starting_doc_state_string.clone(), after_string,"Expected : '{}' \n\nAfter  : '{}'\n\n", starting_doc_state_string.clone(), after_string);
}