use std::fs;
use std::io::Error;

// Many modules in the std lib have their own custom error types

fn main() {
    let text = fs::read_to_string("logs.txt");

    // Result enum is used when we need to know if something worked or faied. Ok(value), Err(error)

    // if Some((&text)).is_some() {
    //     println!("{:#?}", text.unwrap());
    // } else {
    //     println!("no data")
    // }

    match divide(5.0, 3.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division)
        }
        Err(error_of_division) => {
            println!("{}", error_of_division)
        }
    }

    match validate_email(String::from("asd@asd.com")){ 
        Ok(..) => println!("email is valid"),
        Err(reason_of_failed_validation) => {
            println!("{}", reason_of_failed_validation)
        }
    }
    
}


fn divide(a: f64, b:f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err((Error::other("cant divide by 0")))
    } else {
        Ok(a/b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("email must have an @"))
    }
}