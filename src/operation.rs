use std::mem::transmute;
//use std::time::{Duration, SystemTime};

/*
*  An Operation is an immutable data type that can be applied to a document 
*/ 
#[derive(Clone)]
pub struct Operation
{
    is_insert : bool,
    chr : char,
    index : usize,
    id : usize,
    time_stamp : usize,
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
            time_stamp : 0,
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
            //return Operation::new(self.is_insert, self.chr, self.index+1, self.id, self.user_id);
        }else
        {
            if self.index < index
            {
                return;
            }
            self.index +=1;
            //mutating in place
            //return Operation::new(self.is_insert, self.chr,self.index + 1,self.id,self.user_id);
        }
    }
    
    //Serializes an OT operation to a 12 byte format
    //4 bytes - index represented as a signed integer
    //          signed to represent is_insert 
    //          + means insert; - means delete
    //4 bytes - represents the unicode char
    //4 bytes - the user id the op belongs to
    //
    //if the index gets over the cap of 2^31 
    //god save our souls + we can just add an extra boolean
    //to the end of this encoding
    //and get the other 2 billion options back
    //storage will be the least of our worries if 
    //we reach that many operations
    //this also lets quickly count the number of operations
    //even with the extra boolean (disreagrding fixed block size)
    pub fn serialize(&self) -> [u8;16]
    {
        let signed_index : i32 = match self.is_insert
        {
            true => self.index as i32,
            false => -(self.index as i32)
        };
        
        let character : u32 = self.chr as u32;
        let user_id : u32 = self.user_id as u32;
        let epoch_time : u32 = self.time_stamp as u32;
        //because we created local variables on the stack 
        //we can rest easy that these unsafe operations won't fail
        //maybe IDK TODO: check that these won't fail :)
        let ix_bytes: [u8; 4] = unsafe { transmute(signed_index.to_be()) };
        let chr_bytes: [u8; 4] = unsafe { transmute(character.to_be()) };
        let uid_bytes: [u8; 4] = unsafe { transmute(user_id.to_be()) };
        let time_bytes: [u8; 4] = unsafe { transmute(epoch_time.to_be()) };
        
        return [
            ix_bytes[0],ix_bytes[1],ix_bytes[2],ix_bytes[3], //index
            chr_bytes[0],chr_bytes[1],chr_bytes[2],chr_bytes[3], //unicode character
            uid_bytes[0],uid_bytes[1],uid_bytes[2],uid_bytes[3], //user id
            time_bytes[0],time_bytes[1],time_bytes[2],time_bytes[3] //time
        ];
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
    
    pub fn get_readable_representation(&self) -> String
    {
        return format!("ID : '{}' \nINDEX : '{}' \nCHAR : '{}' CHAR_VALUE : '{}' \nUSER_ID : '{}' \nIS_INSERT : {} \n",
        self.get_id(),
        self.get_index(),
        self.get_char(),
        self.get_char().clone() as u32,    
        self.get_user_id(),
        self.is_insert());
    }
}
