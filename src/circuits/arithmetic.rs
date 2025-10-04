
use super::gates::{and, xor, or, not};  // super goes on module level up
//use crate::BoolArray;  // crate goes always to root folder  | If I need func from lib.rs in internal module then use this
// if I need function from lib.rs in main.rs then I need logical_foundation::BoolArray 
use std::fmt;

pub struct HalfAdderOutput {
    sum:bool,
    carry:bool,
}

impl fmt::Display for HalfAdderOutput {
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result { // f is like a notepad or page  &mut is used so that &(format is temporairy borrowed ) mul is f can be editable 
        write!(
            f,"sum: {}, carry: {}", self.sum, self.carry // here f is change to sum: ... therefore we need mut
        )

    }
} // this is used for displaying the half_adder function inside print fucntion

pub fn half_adder(a:bool, b:bool)-> HalfAdderOutput {
    HalfAdderOutput { sum: xor(a, b), carry: and(a, b) }
}

pub struct  FullAdderOutput{
    sum:bool,
    carry:bool
}

impl  fmt::Display for FullAdderOutput {
    fn fmt(&self, f:&mut fmt::Formatter)-> fmt::Result{
        write!(
            f, "sum: {}, carry : {}", self.sum, self.carry
        )
    }
}
    
// sum=xor(a,b)  and carry : and(a,b)
// inutitionally little bit diff from human way of addition
pub fn full_adder(a:bool, b:bool, carry_in:bool) -> FullAdderOutput{
      let first: HalfAdderOutput= half_adder(a, b);
      let second: HalfAdderOutput  = half_adder(first.sum, carry_in);
      let carry_out: bool = or(first.carry, second.carry);
      FullAdderOutput { 
        sum:second.sum, carry: carry_out
     }
}

pub fn add16(a:[bool;16], b:[bool; 16]) -> [bool; 16]{
    let mut current_carry: bool = false;
    let mut result: [bool; 16] = [false;16];
    for i in 0..16{
        let answer = full_adder(a[i], b[i], current_carry);
          result[i]=answer.sum;
          current_carry = answer.carry
    }
    result
}

pub fn not16(a:[bool;16])-> [bool;16]{
    let mut not16_result:[bool;16] = [false;16];
    for i in 0..16{
        not16_result[i] = not(a[i]);
    }
    not16_result

}

pub fn and16(a:[bool;16], b:[bool;16])-> [bool;16]{
    let mut and16_result:[bool;16] = [false;16];
    for i in 0..16{
        and16_result[i] = and(a[i],b[i]);
    }
    and16_result
}