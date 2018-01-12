/*
*  An Operation<T> is an IMMUTABLE! data type that can be applied to a document 
*/ 
#[derive(Clone,PartialEq)]
pub struct Operation<T: Clone>
{
    is_insert : bool,//is this operation an insert or a remove?
    object : T,// the object the operation applies to
    index : usize, //the list index where the operation occurs
    id : usize, // the linear order in which the operation came
    time_stamp : usize,// the epoch time stamp
    user_id : usize// the user id
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
    
    pub fn transform_insertion(&mut self, transform_tuple : ( usize, usize))
    {
        let index = transform_tuple.0;
        let user = transform_tuple.1;
        if self.is_insert
        {
            if self.index < index || (self.index == index && self.user_id < user)
            {
                return;
            }
            self.index+=1;
            //mutating in place
            //return Operation<T>::new(self.is_insert, self.chr, self.index+1, self.id, self.user_id,0);
        }else
        {
            if self.index < index
            {
                return;
            }
            self.index +=1;
            //mutating in place
            //return Operation<T>::new(self.is_insert, self.chr,self.index + 1,self.id,self.user_id,0);
        }
    }
    
    pub fn id(&self) -> usize
    {
        return self.id.clone();
    }
    
    pub fn get_index(&self) -> &usize
    {
        return &self.index;
    }
    
    pub fn get_object(&self) -> T
    {
        return self.object.clone();
    }
    
    pub fn get_user_id(&self) -> &usize
    {
        return &self.user_id;
    }
    
    pub fn is_insert(&self) -> bool
    {
        return self.is_insert;
    }
}
