use std::collections::BTreeSet;
use ot_set::OTSet;
struct Operation
{
    is_insert : bool,
    chr : char,
    index : usize,
    id : usize,
    user : usize
}

struct DocState {
    operations : Vec<Operation>,
    deletions : BTreeSet<usize>,
    doc_str : String,
    points : Vec<usize>
}

impl DocState
{
    fn new(start_string : String) -> DocState
    {
        let doc : DocState = DocState 
        { 
            operations : Vec::new(),
            deletions : BTreeSet::new(),
            doc_str : start_string, 
            points : Vec::new()
        };
        doc
    }
    
    fn add(&mut self, op : Operation)
    {
        if !op.is_insert
        {
            //if the set doesn't already contain the deletion
            if !self.deletions.contains(&op.index)
            {
                let user_index : usize = self.deletions.get_user_space_index(&op.index);
                //insert the opindex in the deletions Set
                self.deletions.insert(op.index);
                self.doc_str.remove(user_index);
                for i in 0..self.points.len() 
                {
                    if self.points[i] > user_index
                    {
                        self.points[i] -= 1;
                    }
                }
            }
        }else if op.is_insert
        {
            self.deletions.increment_indices_past_insert(&op.index);
            let user_index : usize = self.deletions.get_user_space_index(&op.index);
            self.doc_str.insert(op.index, op.chr);
            for i in 0..self.points.len() 
            {
                if self.points[i] > user_index
                {
                    self.points[i] += 1;
                }
            }
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
    