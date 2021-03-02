
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle{
    height: u32,
    width: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.height*self.width
    }
}

fn build_user(email: String, username: String) -> User{
    let temp_user = User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    };
    return temp_user;
}

fn area_rectangle(width: u32, height: u32)-> u32{
    width*height
}

fn main() {
    let mut user1 = User{
        email: String::from("dipansh@thebest.com"),
        username: String::from("dipansh"),
        active: true,
        sign_in_count: 1,
    };

    println!("Username : {}",user1.username);

    //changing values in a struct

    user1.username = String::from("dipansharora");
    println!("Username : {}",user1.username);

    //build_user
    let user2 = build_user(String::from("arora@gmail.com"),String::from("Arora"));
    println!("user2 email: {}",user2.email);

    let height = 30;
    let width = 50;

    let area = area_rectangle(width, height);
    println!("Area of rectangle : {}", area);

    let rect1 = Rectangle{
        width : 10,
        height: 15,
    };

    println!("Area of rect1: {}", rect1.area());
}
