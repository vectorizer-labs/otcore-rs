extern crate rand;

// Note this useful idiom: importing names from outer (for mod tests) scope.
//use super::*;
use ot_set::OTSet;
use std::collections::BTreeSet;
use super::tests::rand::distributions::{Distribution, Uniform};



#[test]
fn getStringSpaceIndexWorks() 
{
    let mut rng = rand::thread_rng();

    let mut testSet : BTreeSet<usize> = BTreeSet::new();

    //up to 20 random numbers in our BTreeSet
    let idxRange : Uniform<usize> = Uniform::from(0..20);
    let ranAmount : usize = idxRange.sample(&mut rng);

    //generate values from 0 to 1,000,000,000                   
    let practicalValueRange : Uniform<usize> = Uniform::from(0..1000000000);

    for i in 0..ranAmount
    {
        testSet.insert(practicalValueRange.sample(&mut rng));
    }

    //the num we want to know the log space index of
    let randomQueryNum: usize = practicalValueRange.sample(&mut rng);


    assert_eq!(testSet.getStringSpaceIndex(1100000000),1100000000 - ranAmount);

}