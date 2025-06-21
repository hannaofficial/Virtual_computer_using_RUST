//


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
    not(and(not(a), not(b)))
}

// XOR Gate
fn xor(a:bool,b:bool) -> bool {
    or(and(not(a),b), and(a, not(b)))
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


fn main() {
    println!("(true, true) -> {}", half_adder(true, true)); 
    println!("(false, true) -> {}", half_adder(false, true)); 
    println!("(true, false) -> {}", half_adder(true, false)); 
    println!("(false, false) -> {}", half_adder(false, false)); 
}


