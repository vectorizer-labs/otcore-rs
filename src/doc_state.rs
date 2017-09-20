use std::collections::BTreeSet;
use ot_set::OTSet;
use operation::Operation;


pub struct DocState {
    pub operations : Vec<Operation>,
    pub deletions : BTreeSet<usize>,
    doc_str : String
}

#[allow(dead_code)]
impl DocState
{
    fn new(start_string : String) -> DocState
    {
        let doc : DocState = DocState 
        { 
            operations : Vec::new(),
            deletions : BTreeSet::new(),
            doc_str : start_string
        };
        doc
    }
    
    fn add(&mut self, op : Operation)
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
    
    pub fn get_str_pointer(&self) -> &String
    {
        return &self.doc_str;
    }
    
    pub fn get_string(&self) -> String
    {
        return self.doc_str.clone();
    }
    
    pub fn xform_ix(&self, index : &usize) -> usize
    {
        return self.deletions.get_doc_space_index(index);
    }
}
    