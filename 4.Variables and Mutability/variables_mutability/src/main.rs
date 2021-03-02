fn main() {
    let x = 5;
    println!("x = {}",x);

    //can't do this, here x is immutable so can't assign a value to x more than once.
    // x = 10;
    println!("x = {}",x);

    let mut y = 10;
    println!("y = {}",y);

    y = 20;
    println!("y = {}",y);

    //shadowing
    let z = 5;
    let z = z + 1;
    let z = z*5;

    println!("z = {}",z);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("spaces = {}",spaces);
}
