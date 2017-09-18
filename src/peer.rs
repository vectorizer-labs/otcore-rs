use std::collections::BTreeSet;
use ot_set::OTSet;
use doc_state::DocState;
use operation::Operation;

#[allow(dead_code)]
struct Peer
{
    revision_id : usize, // a linearly incremented revision id
    context : BTreeSet<usize> //the context of revisions this peer exists in
}

#[allow(dead_code)]
impl Peer
{
    fn new() -> Peer
    {
        return Peer
        {
            revision_id : 0,
            context : BTreeSet::new()
        }
    }
    
    fn merge_op(&mut self, mut doc_state : DocState, mut op : Operation)
    {
        
        //we already have this, roll rev forward
        if self.is_existing_revision(&doc_state, op.get_id()) { return; }
            
        // we already have this, but can't roll rev forward
        if self.is_existing_but_cannot_roll_forward(&doc_state, op.get_id()) { return; }
        
        // we don't have it, need to merge
        op = self.merge(&doc_state, op);
        
        if self.revision_id == doc_state.operations.len()
        {
            self.revision_id +=1;
        }else
        {
            self.context.insert(op.get_id().clone());
        }
        doc_state.operations.push(op);
    }
    
    // we already have this, but can't roll rev forward
    fn is_existing_but_cannot_roll_forward(&mut self, doc_state : &DocState, op_id : &usize) -> bool
    {
        for i in self.revision_id..doc_state.operations.len()
        {
            if doc_state.operations[i].get_id() == op_id
            {
                self.context.insert(op_id.clone());
                return true;
            }
        }
        return false;
    }
    
    //we already have this, roll rev forward
    fn is_existing_revision(&mut self, doc_state : &DocState, op_id : &usize) -> bool
    {
        if self.revision_id < doc_state.operations.len() && doc_state.operations[self.revision_id].get_id() == op_id
        {
            self.revision_id += 1;
            while self.revision_id < doc_state.operations.len() 
                && self.context.contains(doc_state.operations[self.revision_id].get_id())
            {
                self.context.remove(doc_state.operations[self.revision_id].get_id());
                self.revision_id += 1;
            }
            return true;
        }
        return false;
    }
    
    
    // we don't have it, need to merge
    fn merge(&self, doc_state : &DocState, mut op : Operation) -> Operation
    {
        let mut insert_list : Vec<(usize,usize)> = Vec::new();
        let mut s : BTreeSet<usize> = BTreeSet::new();
        let mut t : BTreeSet<usize> = BTreeSet::new();
        
        //for all the operations that are potentially in conflict 
        //merge them
        for ix in (doc_state.operations.len()-1..self.revision_id).rev()
        {
            let doc_space_index : usize = s.get_doc_space_index(doc_state.operations[ix].get_id());
            //if the operation is an insert
            if doc_state.operations[ix].is_insert 
            {
                //if our context does not contain this id
                if !self.context.contains(doc_state.operations[self.revision_id].get_id())
                {
                    insert_list.push(
                        (
                        t.get_user_space_index(doc_state.operations[ix].get_id()),
                        doc_state.operations[ix].get_user_id().clone()
                        )
                    );
                    t.remove(&doc_space_index);
                }
                s.remove(&doc_space_index);
            }
        }
        
        for i in (insert_list.len()-1..0).rev()
        {
            op = op.transform_insertion(insert_list[i]);
        }
        
        return op;
    }
    
}

