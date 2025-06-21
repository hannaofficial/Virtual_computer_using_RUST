


// ---------------------------------------------------------------------------------------------------------------------------------------------------
// declaring storage for half addr output

//nand gate
fn nand(a:bool, b:bool) -> bool {
    // Notice we don't need a `return` keyword here. If the last
    // expression in a function has no semicolon, Rust automatically
    // returns it. This is a common style.
    !(a&b)  // ! is not in rust
}
// not gate
fn not(a:bool) -> bool {
    nand(a,a)
}

// AND gate
fn and(a:bool, b:bool) -> bool {
    not(nand(a, b))
}

// OR gate
// this is actually demorgan law
fn or(a:bool, b:bool) -> bool{
    not(and(not(a), not(b))) //nand(not(a), not(b))
}

// XOR Gate
fn xor(a:bool,b:bool) -> bool {
    or(and(not(a),b), and(a, not(b))) // (not a and b) or (a and not b)
}

use std::fmt;

struct HalfAdderOutput {
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

fn half_adder(a:bool, b:bool)-> HalfAdderOutput {
    HalfAdderOutput { sum: xor(a, b), carry: and(a, b) }
}

struct  FullAdderOutput{
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
fn full_adder(a:bool, b:bool, carry_in:bool) -> FullAdderOutput{
      let first: HalfAdderOutput= half_adder(a, b);
      let second: HalfAdderOutput  = half_adder(first.sum, carry_in);
      let carry_out: bool = or(first.carry, second.carry);
      FullAdderOutput { 
        sum:second.sum, carry: carry_out
     }

}


struct BoolArray([bool;16]); // tuple struct we use 
// you can also defined it as 
// struct BoolArray{
//              bits: [bool;16] , // name field
//                   }

impl fmt::Display for BoolArray {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{ // &represent refrences
    for bit in self.0.iter().rev(){ // self.0 means this is the first position of tuple struct here it is array ([2],[1])  position 0 pos 1
        // self is directly calling the struct but self.0 is the thing inside struct
        write!(f, "{}", if *bit {'1'} else {'0'})?; // *bit to get the value of bit ? is error jhandling
    }
    Ok(()) // same as just return in python for success 
  }  
}
// Display don't use storage and faster than converting to string and then display

fn add16(a:[bool;16], b:[bool; 16]) -> [bool; 16]{
    let mut current_carry: bool = false;
    let mut result: [bool; 16] = [false;16];
    for i in 0..16{
        let answer = full_adder(a[i], b[i], current_carry);
          result[i]=answer.sum;
          current_carry = answer.carry
    }
    result
}


fn main() {
    let mut a = [false; 16]; // All zeros
    let mut b = [false; 16];
    a[1] = true;
    b[2] = true; 
    
    println!("{} + {} = {}", 
        BoolArray(a), 
        BoolArray(b), 
        BoolArray(add16(a, b)));
    
    
}


