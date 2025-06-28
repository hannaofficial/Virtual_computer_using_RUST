// imports from  module circuits
mod circuits;
use circuits::{
    arithmetic::{add16, not16, and16, half_adder, HalfAdderOutput},
    muxes::mux16,
    gates::{and,nand,or,not}
};
use logical_foundation::BoolArray;







fn main() {
    let mut a = [false; 16]; // All zeros
    let mut b = [false; 16];
    a[1] = true;
    b[2] = true;
    let result:[bool;16] = add16(a, b) ;
    
    println!("{} + {} = {},\n not16: {} , \n and16: {}, \n mux16: {}", 
        BoolArray(a), 
        BoolArray(b), 
        BoolArray(result),
        BoolArray(not16(result)),
        BoolArray(and16(a,b)),
        BoolArray(mux16(a,b,true))
    );
        
    
    
}


