pub fn mux16(a:[bool;16], b:[bool; 16], sel:bool) -> [bool;16]{
    let mut mux16_result:[bool;16] = [false;16];
    // We loop each bits instead of slecting all the b or a because of hardware issue
    for i in 0..16{
        mux16_result[i] = if !sel {a[i]} else {b[i]};
    }
    mux16_result
}