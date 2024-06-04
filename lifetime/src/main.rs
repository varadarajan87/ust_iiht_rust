// Define a trait
trait DisplayInfo {
    fn display_info(&self);
}

//Define a struct
struct InfoString {
    info: String,
}

// Implement the trait for the struct
impl DisplayInfo for InfoString {
    fn display_info(&self) {
        println!("{}", self.info);
    }
}

// Struct that implements the trait for using &str and lifetime
/*
   The 'a lifetime annotation in the InfoStr struct definition ensures that the 
   &str reference inside the struct is valid for as long as the struct is in use.
   This prevents the &str reference from becoming dangling, which would lead to 
   undefined behavior.
*/
struct InfoStr<'a> {
    info: &'a str,
}

// Implenent the DisplayInfo trait for InfoStr
impl<'a> DisplayInfo for InfoStr<'a> {
    fn display_info(&self) {
        println!("{}", self.info);
    }
}

/* 
    Rust will automatically coerce &InfoString / &InfoStr<'a> to 
    &dyn DisplayInfo because InfoString / InfoStr implements 
    DisplayInfo trait. This will help work with different types 
    that implement the same trait.
*/

fn main() {

    // Create an instance of InfoString on the stack
    let info_string = InfoString {
        info: String::from("Test String"),
    };
    
    // Create a trait object reference on the stack (object coerced)
    let stack_ref: &dyn DisplayInfo = &info_string;
    stack_ref.display_info(); // This should output Test String

    // Move the trait object to the heap
    let heap_box: Box<dyn DisplayInfo> = Box::new(info_string);
    heap_box.display_info(); // This should output Test String

    // Create an instance of InfoStr on the stack
    let info_str= InfoStr {
        info: "This is string slice"
    };

    // Create a trait object reference on the stack (object coerced)
    let stack_ref_str: &dyn DisplayInfo = &info_str;
    stack_ref_str.display_info(); // This should output This is a string slice

    // Move the trait object to the heap
    let heap_box_str: Box<dyn DisplayInfo> = Box::new(info_str);
    heap_box_str.display_info(); // This should output This is a string slice
}
