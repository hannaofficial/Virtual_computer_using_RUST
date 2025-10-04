// src/lib.rs
use std::fmt;

//Make BoolArray available at crate root
//pub use self::BoolArray;
// Now both `crate::BoolArray` and `logical_foundation::BoolArray` work everywhere

#[derive(Clone, Copy)] // Keep this for efficiency instead of moving in x=y it will just copy so that we can see what is actually happening

pub struct BoolArray( pub [bool;16]); // tuple struct we use // we can use this to display any 16bool array
// you can also defined it as 
// struct BoolArray{
//              bits: [bool;16] , // name field
//                   }

impl fmt::Display for BoolArray {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{ // &represent refrences // &mut just borrowing the writing page for some time
    for bit in self.0.iter().rev(){ // self.0 means this is the first position of tuple struct here it is array ([2],[1])  position 0 pos 1
        // self is directly calling the struct but self.0 is the thing inside struct
        write!(f, "{}", if *bit {'1'} else {'0'})?; // *bit to get the value of bit ? is error handling
    }
    Ok(()) // same as just return in python for success 
  }  
}
// Display don't use storage and faster than converting to string and then display