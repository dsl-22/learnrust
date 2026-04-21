fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // value of s moves into function
                                    // ... and is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Becuase i32 implements the copy trait,
                                    // x does not move into the function,
                                    // so x is still valid after using it
} // Here x goes out of scope, then s. Because the value of s was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // some_string then goes out of scope and 'drop' is called, memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // some_integer goes out of scope. Nothing special happens
