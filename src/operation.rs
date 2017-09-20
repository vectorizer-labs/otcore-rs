/*
*  An Operation is an immutable data type that can be applied to a document 
*/ 
pub struct Operation
{
    is_insert : bool,
    chr : char,
    index : usize,
    id : usize,
    user_id : usize
}

impl Operation
{
    pub fn new(ins : bool, ch : char, ix : usize, op_id : usize, user : usize) -> Operation
    {
        Operation
        { 
            is_insert : ins,  
            chr : ch,
            index : ix,
            id : op_id,
            user_id : user
        }
    }
    
    pub fn transform_insertion(self, transform_tuple : ( usize, usize)) -> Operation
    {
        let index = transform_tuple.0;
        let user = transform_tuple.1;
        if self.is_insert
        {
            if self.index < index || (self.index == index && self.user_id < user)
            {
                return self;
            }
            return Operation::new(self.is_insert, self.chr, self.index+1, self.id, self.user_id);
        }else
        {
            if self.index < index
            {
                return self;
            }
            return Operation::new(self.is_insert, self.chr,self.index + 1,self.id,self.user_id);
        }
    }
    
    pub fn get_id(&self) -> &usize
    {
        return &self.id;
    }
    
    pub fn get_index(&self) -> &usize
    {
        return &self.index;
    }
    
    pub fn get_char(&self) -> &char
    {
        return &self.chr;
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
