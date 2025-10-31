// pub making functon public
use super::muxes::mux16;
// super means go one level up

//nand gate
pub fn nand(a:bool, b:bool) -> bool {
    // Notice we don't need a `return` keyword here. If the last
    // expression in a function has no semicolon, Rust automatically
    // returns it. This is a common style.
    !(a&b)  // ! is not in rust
}
// not gate
pub fn not(a:bool) -> bool {
    nand(a,a)
}

// AND gate
pub fn and(a:bool, b:bool) -> bool {
    not(nand(a, b))
}

// OR gate
// this is actually demorgan law
pub fn or(a:bool, b:bool) -> bool{
    not(and(not(a), not(b))) //nand(not(a), not(b))
}

// XOR Gate
pub fn xor(a:bool,b:bool) -> bool {
    or(and(not(a),b), and(a, not(b))) // (not a and b) or (a and not b)
}

pub fn dmux(in_val: bool, sel:bool)-> (bool, bool) {
   let  a = and(in_val, not(sel));
   let  b = and(in_val, sel);
    (a, b)
}

pub fn dmux8way (in_val: bool, address: [bool;3]) -> [bool; 8] {

    let (left, right) = dmux(in_val,address[0]);

    let (l1, l2) = dmux(left, address[1]);
    let (r1, r2) = dmux(right, address[1]);

    let (o1,  o2) = dmux(l1, address[2]);
    let (o3,  o4) = dmux(l2, address[2]);
    let (o5,  o6) = dmux(r1, address[2]);
    let (o7,  o8) = dmux(r2, address[2]);
    [o1, o2, o3, o4, o5, o6, o7, o8]   

}

fn mux8way16(inputs: [[bool;16];8], address: [bool;3]) -> [bool;16]{
    let m0 = mux16(inputs[0], inputs[1], address[2]);
    let m1 = mux16(inputs[2], inputs[3], address[2]);
    let m2 = mux16(inputs[4], inputs[5], address[2]);
    let m3 = mux16(inputs[6], inputs[7], address[2]);

    let mm0 = mux16(m0, m1, address[1]);
    let mm1 = mux16(m2, m3, address[1]);

    mux16(mm0, mm1, address[0])

}

