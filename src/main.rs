// imports from  module circuits
pub mod circuits;
use circuits::{
    arithmetic::{add16, not16, and16, half_adder, HalfAdderOutput},
    muxes::mux16,
    gates::{and,nand,or,not}
};
use logical_foundation::BoolArray;

use std::fmt;

// mux here is if sel is false then a else b

struct AluControlBits {
    zx: bool, // just a naming convention z represent zero, n: negates, f: function to choose and/or, no: negates the output
    nx: bool,// x, y are just varaible
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
}
struct AluOutput {
    out: [bool; 16],
    zr: bool, // stands zero result
    ng: bool, // negative flag here 0 means pos and 1 means neg
}

impl fmt::Display for AluOutput {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let color_out = if self.ng { "\x1b[31m" } else { "\x1b[32m" }; // Red for negative, green for positive
        
        let color_zr = if self.zr { "\x1b[32m" } else { "\x1b[31m" }; // Green if zero result, Red otherwise
        let color_ng = if self.ng { "\x1b[31m" } else { "\x1b[32m" }; // Red for negative, Green otherwise


        write!(
            f, 
            "out: {}{}\x1b[0m, zr: {}{}\x1b[0m, ng: {}{}\x1b[0m",
            color_out,
            BoolArray(self.out),
            color_zr,
            self.zr,
            color_ng,
            self.ng
        )
        //Stop using any colors or text styles, reset to normal.
        //The \x1b[0m comes after printing the colored value to turn off the color, so the rest of the text (like , zr: {}, ng: {}) appears in the normal terminal color.
    }
    
}

// mux16 is just the selector between a and b based on sel
fn alu(x: [bool;16],y:[bool;16],control:AluControlBits) -> AluOutput{
    let x1 = mux16(x, [false;16], control.zx ); // first four line are just to process the input
    let x2 = mux16(x1, not16(x1), control.nx);  // whether should take it or not || negate it or not
    let y1 = mux16(y, [false;16], control.zy);
    let y2 = mux16(y1, not16(y1), control.ny);
    let and_out = and16(x2,y2);
    let add_out = add16(x2, y2);
    let f_out = mux16(and_out, add_out, control.f); // controlling whether to add or and between process input x2,y2
    let final_output = mux16(f_out, not16(f_out), control.no); // whether to negate output or not

    let ng = final_output[15];
    let mut zr = false;
    for i in 0..16{
        zr = or(zr,final_output[i]);
        if zr {
        break;
              }
    }
    let zr = not(zr); // this is to check whether all the 16 bit are false or not if false then zr is true

    AluOutput { out: final_output, zr: zr, ng: ng }

    //In binary, -1 is represented as all 1's
    // x - y = x + NOT(y) + 1  remember good concept not(y) directly doesn't make the -y the not(y) should be added to +1 come from carry in of adder function that is discarded by the cpu in reallife
    //But because the ALU’s adder discards the final carry, it’s equivalent to adding +1 automatically.
}







fn main() {
    // implementing x - 1 using alu
    let mut x = [false;16];
     x[0]=true;
     x[2]=true;

     let  y = [false;16];

     let control = AluControlBits {
        zx: false,  // Use x as-is
        nx: false,   // Don't negate x
        zy: true,    // Zero y input
        ny: true,    // Negate the zero (effectively -1)
        f: true,     // Use ADD operation
        no: false    // Don't negate output
    };
     

     let result = alu(x, y, control);

    println!("result {}",
            result 
        
    );
        
    
    
}


