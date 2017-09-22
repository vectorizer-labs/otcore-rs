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
        //instead of returning a new op we just modify op in place
        self.merge(&doc_state, &mut op);
        
        
        if self.revision_id == doc_state.get_operations_len()
        {
            self.revision_id +=1;
        }else
        {
            self.context.insert(op.get_id().clone());
        }
        
        //add the newly transformed operation to the given doc_state
        doc_state.add(op);
    }
    
    // we already have this, but can't roll rev forward
    fn is_existing_but_cannot_roll_forward(&mut self, doc_state : &DocState, op_id : &usize) -> bool
    {
        for i in self.revision_id..doc_state.get_operations_len()
        {
            if doc_state.get_operation(i).get_id() == op_id
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
        if self.revision_id < doc_state.get_operations_len() && doc_state.get_operation(self.revision_id).get_id() == op_id
        {
            self.revision_id += 1;
            while self.revision_id < doc_state.get_operations_len() 
                && self.context.contains(doc_state.get_operation(self.revision_id).get_id())
            {
                self.context.remove(doc_state.get_operation(self.revision_id).get_id());
                self.revision_id += 1;
            }
            return true;
        }
        return false;
    }
    
    
    // we don't have it, need to merge
    fn merge(&self, doc_state : &DocState, op : &mut Operation)
    {
        let mut insert_list : Vec<(usize,usize)> = Vec::new();
        let mut s : BTreeSet<usize> = BTreeSet::new();
        let mut t : BTreeSet<usize> = BTreeSet::new();
        
        //Go backwards through the operations 
        for ix in (doc_state.get_operations_len()-1..self.revision_id).rev()
        {
            let doc_space_index : usize = s.get_doc_space_index(doc_state.get_operation(ix).get_id());
            //if the operation is an insert
            if doc_state.get_operation(ix).is_insert()
            {
                //if our context does not contain this operation
                if !self.context.contains(doc_state.get_operation(self.revision_id).get_id())
                {
                    //we need to add it
                    insert_list.push(
                        (
                        t.get_user_space_index(doc_state.get_operation(ix).get_id()),
                        doc_state.get_operation(ix).get_user_id().clone()
                        )
                    );
                    t.insert(doc_space_index);
                }
                s.insert(doc_space_index);
            }
        }
        
        for i in (insert_list.len()-1..0).rev()
        {
            op.transform_insertion(insert_list[i]);
        }
    }
    
}

