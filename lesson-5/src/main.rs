use std::fs;
use std::io::Error;

fn main() {
    let text = fs::read_to_string("logs.txt");

    // Result enum is used when we need to know if something worked or faied. Ok(value), Err(error)

    if Some((&text)).is_some() {
        println!("{:#?}", text.unwrap());
    } else {
        println!("no data")
    }
    
}


fn divide(a: f64, b:f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err((Error::other("cant divide by 0")))
    } else {
        Ok(a/b)
    }
}