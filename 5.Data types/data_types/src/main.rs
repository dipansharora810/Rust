fn main() {

    //tuples
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    //arrays
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];


    //array of 5 elements with initial value of 3
    let mut a  = [3; 5];

    a[4] = 4;
    println!("a[4] = {}", a[4]);

    let mut i = 0;
    loop{
        println!("a[{}] = {}",i,a[i]);
        i= i+1;

        if i == 5 {
            break;
        }
    }

    another_function(5);

    println!("{}",plus_one(99));

    if 5>10 {
        println!("LOl");
    }
    else{
        println!("5 is not greater than 10");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //Loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [1,2,3,4,5];

    for ele in a.iter(){
        println!("ele = {}",ele)
    }


}
    //functions
fn another_function(x:i32){
    println!("HAHAHHAHAHA");
    println!("x = {}",x);
}

//takes an int and return an int
fn plus_one(x:i32)->i32{
    x+1
}