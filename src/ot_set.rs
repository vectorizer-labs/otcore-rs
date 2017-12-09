use std::collections::BTreeSet;

pub trait OTSet
{
    fn get_user_space_index(&self , index : &usize) -> usize;
    fn get_doc_space_index(&self , index : &usize) -> usize;
    fn increment_indices_past_insert(&mut self, index : &usize);
    fn decrement_indices_past_insert(&mut self, index : &usize);
    
}

impl OTSet for BTreeSet<usize>
{
    
    //precondition: index doesn't exist in the set
    fn get_user_space_index(&self , index : &usize) -> usize
    {
        let mut set_iter = self.iter();
        let mut lower_indices : usize = 0;
        //loop through all the indices until we reach the point where we are greater than an index
        loop
        {
            match set_iter.next()
            {
                Some(n) => 
                {
                    if n < index { lower_indices += 1; }
                    else 
                    {
                        if n == index
                        { 
                            panic!("The index {} is equal to {}! This shouldn't be able to happen.", index, n) 
                        }
                        break; 
                    }
                },
                None => break
            }
        }
        return index - lower_indices;
    }
    
    //TODO: Optimize using better bst operation
    fn get_doc_space_index(&self , index : &usize) -> usize
    {
        let mut set_iter = self.iter();
        let mut lower_indices : usize = 0;
        //loop through all the indices until we reach the point where we are greater than an index
        loop
        {
            match set_iter.next()
            {
                Some(n) => 
                {
                    if n < index { lower_indices += 1; }
                    else { break; }
                },
                None => break
            }
        }
        return index + lower_indices;
    }
    
    //TODO: Optimize using better bst operation
    fn increment_indices_past_insert(&mut self, index : &usize)
    {
        let greater_than_set = self.split_off(index);
        let mut set_iter = greater_than_set.iter();
        //loop through all the indices incrementing all the ones that are greater than index 
        loop
        {
            match set_iter.next()
            {
                Some(n) => 
                {
                    self.insert(n+1);
                },
                None => break
            }
        }
    }
    
    //TODO: Optimize using better bst operation
    fn decrement_indices_past_insert(&mut self, index : &usize)
    {
        let greater_than_set = self.split_off(index);
        let mut set_iter = greater_than_set.iter();
        //loop through all the indices decrementing all the ones that are greater than index 
        loop
        {
            match set_iter.next()
            {
                Some(n) => 
                {
                    self.insert(n-1);
                },
                None => break
            }
        }
    }
}