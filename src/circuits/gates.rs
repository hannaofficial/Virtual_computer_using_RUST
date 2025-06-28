// pub making functon public

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