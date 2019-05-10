# rustlearningbymyself
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
     
3. Day
    
    Struct learning: `#[derive(Debug)]` could give struct a character. For example Debug
    could let the struct print by {:?}.
    
    `match` or `if let` can be used, when enum values have data inside them.
    
    `Vec<T>` is likely a mutable list. `Vec::new()` creates a leer list or `vec![1,2,3]`
    a list with elements 1, 2, 3. `push` is the update method for list. `&v[2]` or `v.get(2)`
    can read a element of vectors but `get` returns `Option<&T>`.
    
    `String` is mutable but `str` not. `push_str(&str)` appends a string to another.
    
    `HashMap` stores a mapping of keys to values. `let my_hashmap: HashMap<_, _> = vec1_keys.iter().zip(vec2_values.iter()).collect();`
    creates a hashmap from two vectors.
    
4. Day

    Error handling: the `?` Operator can only be used in functions that return `Result`.
    The decision on `panic!` or `Result` depends on, if an error should be
    recovered or not. In most situation `Result` should be used because we do not 
    want the program to crush. It should be fixed.
    
    Using `panic!` and `Result` in the appropriate situation will make my code
    more reliable in the face of inevitable problems.