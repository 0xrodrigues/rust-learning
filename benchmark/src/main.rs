use rust_decimal::dec;
use std::time::{SystemTime};

use chrono::offset::Utc; 
use chrono::DateTime;


struct Color(i32, i32, i32, i32);


struct Rectangle {
    length: u32,
    width: u32,
}

struct Customer { 
    fullname: String,
    email: String,
    active: bool,
    amount: rust_decimal::Decimal
}

struct Log {

    text: String,
    moment: SystemTime

}

fn main() {
    //let customer = get_user();
    //let log = get_log(SystemTime::now(), &customer.email);
    //println!("Log: {} at: {:?}", log.text, format!("{}", Into::<DateTime<Utc>>::into(log.moment).timestamp()));


    let rect1 = Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}


fn get_user() -> Customer {

    return Customer {
            fullname: String::from("Edson Rodrigues"),
            email: String::from("email@email.com"),
            active: true,
            amount: dec!(3.45)

        };

}

fn get_log(moment: SystemTime, customer_mail: &String) -> Log {
    return Log { text: String::from(format!("Criando usuario com email: {customer_mail}")), moment }
}


// String interpolation sample
// apples = 4
// print("I have ${apples} apples.") # string interpolation
// print("I have " + apples + " apples.") # string concatenation
// print("I have %s apples.", apples) # format string