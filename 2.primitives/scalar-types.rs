fn main(){
    /*  
      scalar types
      integers - u8, i8, u16, i16, u32, i32 (default), u64, i64, u128, i128
      float - f32, f64 (default)
      boolean - bool (1 byte)
      character - char (4 bytes)
      unit type - ()
    */
    let b: bool = true;

    let f: f64 = 1.0;  // Regular annotation
    let i = 5i32; // Suffix annotation

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed but not the type
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    // mutable = 12341234123412341234123; // Error! Type can't be changed.

    // Variables can be overwritten with shadowing.
    let mutable = true;
}