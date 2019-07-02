use std::collections::BTreeSet;
use std::ops::Bound::Included;

pub trait OTSet
{
    fn getStringSpaceIndex(&self , index : usize) -> usize;
    fn getLogSpaceIndex(&self , index : usize) -> usize;
    fn incrementIndicesPastIndex(&mut self, index : &usize);
    fn decrementIndicesPastIndex(&mut self, index : &usize);
    
}

impl OTSet for BTreeSet<usize>
{
    //xi_inv
    //this function counts the number of indices
    //that haven't been tombstoned(deleted)
    //before the given index
    //(thus giving us the effective user visible space index)
    //precondition: index doesn't exist in the set
    fn getStringSpaceIndex(&self , index : usize) -> usize
    {
        let mut indexRange = self.range((Included(0 as usize), Included(index)));
        let mut lower_indices : usize = 0;
        //loop through all the indices until we reach the point where we are greater than an index
        loop
        {
            match indexRange.next()
            {
                Some(_n) => { lower_indices += 1; }
                None => break
            }
        }
        return index - lower_indices;
    }
    
    //xi
    //this function counts the number of tombstones
    //before the given index in O(logn) time
    fn getLogSpaceIndex(&self , index : usize) -> usize
    {
        let mut indexRange = self.range((Included(0 as usize), Included(index)));
        let mut lower_indices : usize = 0;
        //loop through all the indices until we reach the point where we are greater than an index
        loop
        {
            match indexRange.next()
            {
                Some(_n) => { lower_indices += 1; }
                None => break
            }
        }
        return index + lower_indices;
    }
    
    //TODO: Optimize using better bst operation
    fn incrementIndicesPastIndex(&mut self, index : &usize)
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
    fn decrementIndicesPastIndex(&mut self, index : &usize)
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