use std::collections::BTreeSet;

#[allow(dead_code)]
pub struct Peer
{
    revision_id : usize, // a linearly incremented revision id
    context : BTreeSet<usize> //the context of revisions this peer exists in
}

#[allow(dead_code)]
impl Peer
{
    pub fn new() -> Peer
    {
        return Peer
        {
            revision_id : 0,
            context : BTreeSet::new()
        }
    }
    
    pub fn revision_id(&self) -> usize
    {
        return self.revision_id.clone();
    }
    
    pub fn increment_revision_id(&mut self)
    {
        self.revision_id +=1;
    }
    
    pub fn add_new_revision(&mut self, id : &usize)
    {
        self.context.insert(id.clone());
    }
    
    pub fn contains(&self, id : &usize) -> bool
    {
        return self.context.contains(id);
    }
    
    pub fn remove(&mut self, id : &usize)
    {
        self.context.remove(id);
    }
    
}

