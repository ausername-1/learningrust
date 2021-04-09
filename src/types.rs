pub fn run() {
    let x = 1;

    let y = 2.5;
    
    let z: i64 = 3494449949494;
    
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolen

    let isactive = true;

    //get boolean from expression

    let is_greater = 10 > 5;

    //unicode chars

    let a1 = 'a';

    println!("{:?}", (x, y ,z , isactive, is_greater, a1));
}