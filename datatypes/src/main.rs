fn main() {
    //let guess = "42".parse().expect("Not a number!");
    let _guess : u32 = "42".parse().expect("Not a number!");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // this prin [3,3,3,3,3]

    // acceder mediante indexion
    let c = [1, 2, 3, 4, 5];
    let first = c[0];
    let second = c[1];
}
