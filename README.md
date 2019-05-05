# hello_world
Rust learning notes
1. Day

    First of all, i started to learn Rust by myself and wrote a hello-world program
    as all beginner did. Then according to the book from Rust homepage
    i developed a guess-number rust-program.

2. Day

    Data types learning: Integer types with signed and unsigned, floating-point types
    boolean, character as well as tuple and array as compound types.
    Array in Rust has a fixed length, also tuple.
    
    Ownership rules in rust: 
    
     - Each value in Rust has a variable that’s called its owner.
          
     - There can only be one owner at a time.
          
     - When the owner goes out of scope, the value will be dropped.
     
    Borrowing rule:
    
     - We cannot have a mutable reference while we have a immutable one.
     
     - Multi mutable references are not acceptable in a scope, but immutable
     reference are acceptable.
     
     - References must always be valid
     
     - string literal and &str are immutable