use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::char;

/*
*  An Operation is an IMMUTABLE! data type that can be applied to a document 
*/ 
#[derive(Clone,PartialEq)]
pub struct Operation
{
    is_insert : bool,//is this operation an insert or a remove?
    chr : char,// the character the operation applies to
    index : usize, //the character index where the operation occurs
    id : usize, // the linear order in which the operation came
    time_stamp : usize,// the epoch time stamp
    user_id : usize// the user id
    //these parameters must remain usize to take advantage of the platforms capabilities
    //despite the fact they are currently serialized to u32
    //file formats can change, but memory representation must also remain flexible to allow this
}

impl Operation
{
    pub fn new(ins : bool, ch : char, ix : usize, op_id : usize, user : usize, time : usize) -> Operation
    {
        Operation
        {
            is_insert : ins,  
            chr : ch,
            index : ix,
            id : op_id,
            time_stamp : time,
            user_id : user
        }
    }
    
    
    pub fn deserialize(op : Vec<u8>) -> Operation
    {
        let mut rdr = Cursor::new(op);
        
        //TODO : This whole block desperately needs error checking!!!
        let signed_index : isize = rdr.read_i32::<LittleEndian>().unwrap() as isize;
        let character : char = char::from_u32(rdr.read_u32::<LittleEndian>().unwrap()).unwrap();
        let user : usize = rdr.read_u32::<LittleEndian>().unwrap() as usize;
        let time : usize = rdr.read_u32::<LittleEndian>().unwrap() as usize;
        let op_id : usize = rdr.read_u32::<LittleEndian>().unwrap() as usize;
        
        return Operation
        {
            is_insert : signed_index.is_positive(),
            chr : character,
            index : signed_index.abs() as usize,
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
            //return Operation::new(self.is_insert, self.chr, self.index+1, self.id, self.user_id,0);
        }else
        {
            if self.index < index
            {
                return;
            }
            self.index +=1;
            //mutating in place
            //return Operation::new(self.is_insert, self.chr,self.index + 1,self.id,self.user_id,0);
        }
    }
    
    //Serializes an OT operation to a 20 byte format
    //4 bytes - index represented as a signed integer
    //          signed to represent is_insert 
    //          + means insert; - means delete
    //4 bytes - represents the unicode char
    //4 bytes - the user id the op belongs to
    //4 bytes - epoch timestamp
    //4 bytes - operation ID
    //
    //if the index gets over the cap of 2^31 
    //god save our souls + we can just add an extra boolean
    //to the end of this encoding
    //and get the other 2 billion options back
    //storage will be the least of our worries if 
    //we reach that many operations
    //this also lets quickly count the number of operations
    //even with the extra boolean (disreagrding fixed block size)
    pub fn serialize(&self) -> Vec<u8>
    {
        let signed_index : i32 = match self.is_insert
        {
            true => self.index.clone() as i32,
            false => -(self.index.clone() as i32)
        };
        
        let mut wtr = vec![];
        
        //write index + is_insert
        wtr.write_i32::<LittleEndian>(signed_index).unwrap();
        
        //write char
        wtr.write_u32::<LittleEndian>(self.chr.clone() as u32).unwrap();
        
        //write user id
        wtr.write_u32::<LittleEndian>(self.user_id.clone() as u32).unwrap();
        
        //write time
        wtr.write_u32::<LittleEndian>(self.time_stamp.clone() as u32).unwrap();
        
        //write op id
        wtr.write_u32::<LittleEndian>(self.id.clone() as u32).unwrap();
        
        return wtr;
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
