use std::collections::BTreeSet;       
use ot_set::OTSet;
use operation::Operation;
use peer::Peer;


pub struct DocState<T : Clone> {
    pub operations : Vec<Operation<T>>,
    pub deletions : BTreeSet<usize>,
    pub doc_str : Vec<T>
}

#[allow(dead_code)]
impl<T: Clone> DocState <T>
{
    
    pub fn new() -> DocState<T>
    {
        let doc : DocState<T> = DocState 
        {
            operations : Vec::new(),
            deletions : BTreeSet::new(),
            doc_str : Vec::new()
        };
        doc
    }
    
    
    
    pub fn add(&mut self, op : Operation<T>)
    {
        //if its an insert operation
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
            self.doc_str.insert(op.get_index().clone(), op.get_object());
        }
        self.operations.push(op);
    }
    
    pub fn remove(&mut self, op : Operation<T>)
    {
        //assert!(self.operations.contains(&op), "You shouldn't be able to get here unless you try to remove an operation that doesn't exist in the set."); 
        //TODO: Move this assert to testing
        
        if !op.is_insert() //deletion
        {
            //if the set does contain the deletion
            if self.deletions.contains(op.get_index())
            {
                let user_index : usize = self.deletions.get_user_space_index(&op.get_index());
                //insert the opindex in the deletions Set
                self.deletions.remove(op.get_index());
                self.doc_str.insert(user_index, op.get_object()); //search for the object that was deleted and re-insert it here
            }
            
        }else //insertion
        {
            let user_index : usize = self.deletions.get_user_space_index(&op.get_index());
            self.deletions.decrement_indices_past_insert(&user_index);
            //remove the Object at the index
            self.doc_str.remove(user_index);
        }
        //self.operations.remove(op.get_index().clone());need to remove the OP **************
    }
    
    
    //////////////////////////////////////////////////////////////////////////////////////////////////////////
    ///BEGIN PEER MERGE OPERATIONS
    
    pub fn merge_op(&mut self, peer : &mut Peer, mut op : Operation<T>) -> Option<Operation<T>>
    {
        
        //we already have this, roll rev forward
        if self.is_existing_revision(peer, op.id()) { return None; }
            
        // we already have this, but can't roll rev forward
        if self.is_existing_but_cannot_roll_forward(peer, op.id()) { return None; }
        
        // we don't have it, need to merge
        //instead of returning a new op we just modify op in place
        self.merge(&peer, &mut op);
        
        //if its already up to date, 
        //just increment the revision ID and add the new OP
        if peer.revision_id().eq(&self.get_operations_len())
        {
            //peer revision_id +=1
            peer.increment_revision_id();
        }
        else
        {
            peer.add_new_revision(&op.id());
        }
        
        //add the newly transformed operation to self
        self.add(op.clone());
        
        return Some(op);
    }
    
    // we already have this, but can't roll rev forward
    //checks if this is this case
    fn is_existing_but_cannot_roll_forward(&mut self, peer : &mut Peer, op_id : usize) -> bool
    {
        for i in peer.revision_id().clone()..self.operations.len()
        {
            if self.operations[i].id() == op_id
            {
                peer.add_new_revision(&op_id);
                return true;
            }
        }
        return false;
    }
    
    //we already have this, roll rev forward
    //checks if this is the case
    fn is_existing_revision(&mut self, peer : &mut Peer, op_id : usize) -> bool
    {
        if peer.revision_id() < self.get_operations_len() && self.get_operation(peer.revision_id()).id() == op_id
        {
            peer.increment_revision_id();
            while peer.revision_id() < self.get_operations_len() 
                && peer.contains(&self.operations[peer.revision_id()].id())
            {
                let rm_revision = &self.operations[peer.revision_id()].id();
                peer.remove(rm_revision);
                peer.increment_revision_id();
            }
            return true;
        }
        return false;
    }
    
    
    // we don't have it, need to merge
    fn merge(&mut self, peer : &Peer, op : &mut Operation<T>)
    {
        let mut insert_list : Vec<(usize,usize)> = Vec::new();
        let mut s : BTreeSet<usize> = BTreeSet::new();
        let mut t : BTreeSet<usize> = BTreeSet::new();
        
        //Go backwards through the operations 
        for ix in (self.get_operations_len()-1..peer.revision_id()).rev()
        {
            let doc_space_index : usize = s.get_doc_space_index(&self.get_operation(ix).id());
            //if the operation is an insert
            if self.get_operation(ix).is_insert()
            {
                //if our context does not contain this operation
                if !peer.contains(&self.get_operation(peer.revision_id()).id())
                {
                    //we need to add it
                    insert_list.push(
                        (
                        t.get_user_space_index(&self.get_operation(ix).id()),
                        self.get_operation(ix).get_user_id().clone()
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
    
    
    //GETTERS BEGIN
    pub fn operations(&self)-> &Vec<Operation<T>>
    {
        return &self.operations;
    }
    
    pub fn doc_str(&self) -> &Vec<T>
    {
        return &self.doc_str;
    }
    
    pub fn xform_ix(&self, index : &usize) -> usize
    {
        return self.deletions.get_doc_space_index(index);
    }
    
    pub fn get_operation(&self, index : usize) -> &Operation<T>
    {
        return &self.operations[index];
    }
    
    pub fn get_operations_len(&self) -> usize
    {
        return self.operations.len();
    }

}
    