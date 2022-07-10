fn main() {
    println!("Testing some variables...");

    // Variable type inferred, mutable variable
    let mut x = 1;
    println!("x = {}", x);
    x = 5;
    println!("Now x = {}", x);

    // Byte integer
    let x: u8 = b'A';
    println!("Byte x is {}", x);
    // Variable redeclaring
    let x: u8 = 0b0000_1001;
    println!("Now byte x is {}", x);

    let f_num: f64 = 4.6;
    println!("Floating point f_num is {}", f_num);

    let mut b_var: bool = false;
    b_var = true;

    let c = 'z';
    let z: char = 'Z';

    // -- Compound types --

    // Tuple -> multiple values, fixed length, eterogeneous types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;    // Tuple destructuring
    println!("The value of y is: {y}");
    println!("The value of z is {}", tup.2); // Tuple position accessing

    let unit: () = (); // Unit tuple: default return type for "void" functions

    // Array -> multiple values, fixed length, homogeneous types.
    // Allocated on the stack!
    let a = [1, 2, 3, 4];
    let a: [i32; 4] = [1, 2, 3, 4]; // With type declaration
    let a = [3; 5]; // Initialize an array with a single value -> [3, 3, 3, 3, 3]

    // Array accessing
    let first = a[0];
    let second = a[1];
    // -> index > array size panics at runtime
}
