/*
*  An Operation<T> is an IMMUTABLE! data type that can be applied to a document 
*/ 
#[derive(Clone)]
pub struct Operation<T: Clone>
{
    pub is_insert : bool,//is this operation an insert or a remove?
    pub object : T,// the object the operation applies to
    pub index : usize, //the list index where the operation occurs
    pub id : usize, // the linear order in which the operation came
    pub time_stamp : usize,// the epoch time stamp
    pub user_id : usize// the user id
    //these parameters must remain usize to take advantage of the platforms capabilities
    //despite the fact they are currently serialized to u32
    //file formats can change, but memory representation must also remain flexible to allow this
}

impl<T: Clone> Operation<T>
{
    pub fn new(ins : bool, obj : T, ix : usize, op_id : usize, user : usize, time : usize) -> Operation<T>
    {
        Operation
        {
            is_insert : ins,  
            object : obj,
            index : ix,
            id : op_id,
            time_stamp : time,
            user_id : user
        }
    }

    pub fn equals(&self, O : &Operation<T>) -> bool
    {
        return self.id == O.id && self.user_id == O.user_id && self.time_stamp == O.time_stamp;
    }

    //set the index of an operation
   pub fn set_index(mut self, index : usize) -> Operation<T> { self.index = index; self }

   pub fn transform_ins(&mut self, ix : usize, user_id : usize)
   {
    if self.is_insert
    {
      if self.index < ix || (self.index == ix && self.user_id < user_id) { return;}
      else { self.index+=1; } 
    } 
    else // O is a delete
    { 
      if self.index < ix { return; }
      else { self.index +=1; }
    }
  }
}
