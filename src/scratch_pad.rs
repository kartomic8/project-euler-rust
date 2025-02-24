struct ToDrop {
    my_value: String,
}
impl ToDrop {
    pub fn do_something(self) {
        println!("Doing something and taking ownership of self");
    }

    pub fn do_something_else(&self) {
        println!("Doing something and borrowing self");
    }
}
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("Dropping this object");
    }
}

fn called() {
    println!(" In Called");
    let to_drop = ToDrop {
        my_value: "Kartik".to_string(),
    };

    to_drop.do_something();
    // won't work.  ownership moved out of this function with previous call
    //to_drop.do_something_else();
    println!("Exiting called.")
}

fn call() {
    called()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_drop() {
        call()
    }
}
