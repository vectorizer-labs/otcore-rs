use std::collections::BTreeSet;
use ot_set::OTSet;
use operation::Operation;


pub struct DocState {
    operations : Vec<Operation>,
    deletions : BTreeSet<usize>,
    doc_str : Vec<char>
}

#[allow(dead_code)]
impl DocState
{
    ///
    ///Creates a new DocState
    ///
    pub fn new(start_string : String) -> DocState
    {
        let doc : DocState = DocState 
        { 
            operations : Vec::new(),
            deletions : BTreeSet::new(),
            doc_str : doc_str_from_string(start_string)
        };
        doc
    }
    
    pub fn new_empty() -> DocState
    {
        let doc : DocState = DocState 
        { 
            operations : Vec::new(),
            deletions : BTreeSet::new(),
            doc_str : Vec::new()
        };
        doc
    }
    
    pub fn add(&mut self, op : Operation)
    {
        if !op.is_insert()
        {
            //if the set doesn't already contain the deletion
            if !self.deletions.contains(op.get_index())
            {
                let user_index : usize = self.deletions.get_user_space_index(&op.get_index());
                //insert the opindex in the deletions Set
                self.deletions.insert(op.get_index().clone());
                self.doc_str.remove(user_index);
            }
        }else
        {
            self.deletions.increment_indices_past_insert(&op.get_index());
            self.doc_str.insert(op.get_index().clone(), op.get_char().clone());
        }
        self.operations.push(op);
    }
    
    pub fn remove(&mut self, op : Operation)
    {
        assert!(self.operations.contains(&op), "You shouldn't be able to get here unless you try to remove an operation that doesn't exist in the set.");
        if !op.is_insert() //deletion
        {
            //if the set does contain the deletion
            if self.deletions.contains(op.get_index())
            {
                let user_index : usize = self.deletions.get_user_space_index(&op.get_index());
                //insert the opindex in the deletions Set
                self.deletions.remove(op.get_index());
                self.doc_str.insert(user_index, op.get_char().clone()); //search for the character that was deleted and re-insert it here
            }
            
        }else //insertion
        {
            let user_index : usize = self.deletions.get_user_space_index(&op.get_index());
            self.deletions.decrement_indices_past_insert(&user_index);
            //remove the char at the index
            self.doc_str.remove(user_index);
        }
        //self.operations.remove(op.get_index().clone());need to remove the OP **************
    }
    
    /*pub fn get_str_pointer(&self) -> &String
    {
        return &self.doc_str;
    }*/
    
    pub fn get_string(&self) -> String
    {
        let mut s: String = String::new();
        for ch in self.doc_str.clone()
        {
            s.push(ch);
        }
        
        return s;
    }
    
    pub fn xform_ix(&self, index : &usize) -> usize
    {
        return self.deletions.get_doc_space_index(index);
    }
    
    pub fn get_operation(&self, index : usize) -> &Operation
    {
        return &self.operations[index];
    }
    
    pub fn get_operations_len(&self) -> usize
    {
        return self.operations.len();
    }

}

pub fn doc_str_from_string(start_string : String) -> Vec<char>
{
    let mut final_vec : Vec<char> = Vec::new();
    for ch in start_string.chars()
    {
        final_vec.push(ch.clone());
    }
    return final_vec;
}
    