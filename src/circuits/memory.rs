#[derive(Clone, Copy, Default)]
struct Bit {
    state: bool,
}

impl Bit {
    pub fn new()->Self{
        Self {state: false} // this are default value given to each struct as like init in python self.value = value 

    }

    pub fn clock(&mut self, in_val: bool,  load:bool)-> bool{
        //  Read the current state to use as the output for this cycle.
        let output = self.state;
        //  Determine the *next* state based on the inputs.
        if load {
           self.state = in_val
        }
        // if load is false we do nothing 
        output
        
        
    }
}

#[derive(Default)]
pub struct Register{
    bits: [Bit; 16]
}

impl Register{
    pub fn new()-> Self {
        //  #[derive(Default)] above lets us do his easily
        Self::default()  // here defaut value gonna be false
    }

    pub fn clock(&mut self, in_val: [bool;16], load: bool)->[bool; 16]{
        let mut output = [false; 16];
            for i in 0..16{
                output[i] = self.bits[i].clock(in_val[i], load);
            }
        output
    }
}


// #[derive(Default)]
pub struct RAM8{
    registers: [Register; 8],
}

impl RAM8 {
    pub fn new() -> Self{
        Self::default()
    }


    
}


// struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }
// impl Color {
//     pub fn new(r: u8, g: u8, b: u8) -> Self { // pub means this func can be used in other module by default every file is private
//         Self { r, g, b } // here Self refer to the stuct Color
//     }

//     pub fn black() -> Self {
//         Self { r: 0, g: 0, b: 0 }
//     }

//     pub fn white() -> Self {
//         Self { r: 255, g: 255, b: 255 }
//     }
// }