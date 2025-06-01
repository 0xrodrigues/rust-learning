

fn main() {
    let result = divide(Some(5), Some(2));
    
    match result {
        Some(x) => println!("Result: {x}"),
        None    => println!("Cannot divide by 0"),
    }

    if result.is_some() {
        println!("Result is some");
    } else {
        println!("Result is none");
    }

}

fn divide(numerator: Option<u32>, denominator: Option<u32>) -> Option<u32> {
    return Some(numerator? / denominator?);
}