use std::env;
use std::error::Error;
use std::fmt;
use std::net::TcpStream;

/*
    Trait for checking websites.
    Defines a check method that returns a Result.
*/
trait WebSiteChecker {
    fn check(&self) -> Result<(), Box<dyn Error>>;
}

/*
    Struct to represent a website.
    Struct holds the website name.
*/
struct WebSite {
    name: String,
}

/*
    
    Implement the WebSiteChecker trait for the WebSite struct.
    The check method connects to the website and reports if it is
    reachable.

*/
impl WebSiteChecker for WebSite {
    fn check(&self) -> Result<(), Box<dyn Error>> {
        let address = format!("{}:80", self.name);
        match TcpStream::connect(&address) {
            Ok(_) => {
                println!("{} is reachable.", self.name);
                Ok(())
            }
            Err(e) => {
                println!("{} is not reachable: {}", self.name, e);
                Err(Box::new(e))
            }
        }
    }
}

/*

    Define a function that can check any type that implements the WebSiteChecker trait.
    This function accepts any type that implements the WebSiteChecker trait. 
    Also, check_website function shows how to pass owned types.

    T: WebSiteChecker is a generic parameter, can accept any T which implements WebSiteChecker trait.
    More flexible and re-usable. 

*/
fn check_website<T: WebSiteChecker>(checker: T) {
    match checker.check() {
        Ok(_) => println!("Website check passed."),
        Err(_) => println!("Website check failed"),
    }
}

/*
    
    Default error type for missing command-line argument.

*/
#[derive(Debug)]
struct MissingArgumentErr;

/*

    Display trait is implemeneted to provide easy way to get error message.
    fmt defines how error should be formatted as string.

*/
impl fmt::Display for MissingArgumentErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing website name argument")
    }
}

/*

    Implementing the Error trait for MissingArgumentErr allows
    it to be used as an error type in Rust error ecosystem.

    This trait does not require any methods to be implemented if
    Debug and Display are already implemented. Essentially serves 
    as a marker trait to indicate that the type can be used as an error.

*/

impl Error for MissingArgumentErr {}

fn main() -> Result<(), Box<dyn Error>> {

    // Obtain command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure there are correct number of arguments
    if args.len() != 2 {
        return Err(Box::new(MissingArgumentErr));
    }

    // Obtain the website name from command-line argument
    let website_name = &args[1];

    // Create an instance for the website
    let website = WebSite {
        name: website_name.to_string(), // to_string creates an owned String from the borrowed &str
    };

    check_website(website);



    Ok(())
    
}
