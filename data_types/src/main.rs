fn main() {

    /*+------------ Tuples ----------------+*/
    let tup = (500, 6.4, 1);

    // prefixing these vars in the destruction of this
    // tuple so I don't get compile warnings
    let (_x,y,_z) = tup; 

    // We can still access the tuple by it's indices 
    let one = tup.0;
    let two = tup.1;
    let three = tup.2;

    println!("The value of y is: {y}");
    println!("The value of one is: {one}");
    println!("The value of two is: {two}");
    println!("The value of three is: {three}");

    /*+------------ Arrays ----------------+*/
    let a: [i32; 5] = [1,2,3,4,5];
    let b = [8; 5]; // start w/ initial val 8, 5 times
    let x1 = a[0];
    let x2 = b[1];


}
