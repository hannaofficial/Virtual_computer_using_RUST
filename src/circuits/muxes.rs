pub fn mux16(a:[bool;16], b:[bool; 16], sel:bool) -> [bool;16]{
    let mut mux16_result:[bool;16] = [false;16];
    for i in 0..16{
        mux16_result[i] = if !sel {a[i]} else {b[i]};
    }
    mux16_result
}