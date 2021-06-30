/*
Primitive types in rust
Integers: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128 (u = unsigned, i = signed, nr= bits in mem)
Floats: f32,f64
Boolean (bool)
Character (char)
Tuples
Arrays (Fixed lengh, akin to C)
*/

// Rust is a statically typed langauge, which means that it must know the types of all
// vars at runtime, however, the compiler can usually infere the type, so it does not need to be explicitly stated

pub fn run(){
    //Ints default to "i32"
    let defint = 21;

    //Floats default to "f64"
    let deffloat = 2.5;

    // Add explicit type (i64)
    let expint:i64 = 2121;

    //Get max size for datatype (i32)
    let maxint32 = std::i32::MAX;
    let maxint64 = std::i64::MAX;

    println!("===============TYPES.RS===============");
    println!("This is an i32: {}
    This is an f64: {}
    This is an i64: {}
    This is mx i32: {}
    This is mx i64: {}", defint, deffloat, expint, maxint32, maxint64);

    //Boolean
    let is_implicit_true = true;
    let is_explicit_true:bool = true;

    let got_false_from_expr = 10 > 15;

    println!("{:?}", (is_implicit_true, is_explicit_true, got_false_from_expr));

    //Chars
    let char1 = 'c';
    let char_emoji = '😊';

    println!("{:?}", (char1, char_emoji));
}