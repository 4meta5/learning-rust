# Coding Rust Cheatsheet (1 of 3)

Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.

These are notes from [The Rust Book](https://doc.rust-lang.org/book/), but they also may draw from [Steve Donovan's Gentle Intro](https://stevedonovan.github.io/rust-gentle-intro/readme.html). **These notes only cover the first 11 chapters of The Book** 

) They start from the middle of the Chapter 2 ```guessing_game``` tutorial.
* [Guessing Game](#guess)
* [Immutability](#immutability)
    * [Variables and Type Annotations](#variables)
    * [Shadowing](#shadowing)
    * [Ownership](#ownership)
* [Basics](#basics)
    * [The Stack and the Heap](#stackheap)
    * [Using Modules to Reuse and Organize Code](#modules)
* [Data Structures](#datastructures)
    * [Structs](#structs)
    * [Enums and Pattern Matching](#enums)
    * [Common Collections](#collections)
* [Error Handling](#error)
* [Generics](#generic)
    * [Traits](#traits)
    * [Lifetimes](#lifetimes)
* [Testing](#testing)
    * [Running Tests](#runtests)
    * [Unit Tests](#unittests)
    * [Integration Tests](#integrationtests)

## Guessing Game (Ch.2) <a name="guess"></a>
Switching from an ```expect``` call to a ```match``` expression is how you generally move from crashing on an error to handling the error.


## Immutability <a name="immutablity"></a>
* [Variables and Type Annotations](#variables)
* [Shadowing](#shadowing)
* [Ownership](#ownership)

### Variables and Type Annotations <a name="variables"></a>
* Variables are by default immutable.
* Rust has two compound types: tuples and arrays. In Rust, both are fixed length.
* Type annotations enable static checking. 

Consider comparing performance for different data structures between mutable implementations and *functional*-style code that relies on immutability (which is safer for concurrency).

### Shadowing <a name="shadowing"></a>
Shadowing is different than marking a variable as ```mut```, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the ```let``` keyword. By using ```let```, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed. [Chapter 3 of The Book](https://doc.rust-lang.org/book/2018-edition/ch03-01-variables-and-mutability.html)

### Ownership <a name="ownership"></a>
Some languages (ie Java) have garbage collection that constantly looks for memory that is no longer being used. In other languages (C++), the programmer must explicity allocate and deallocate (free) memory. In **Rust**, memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.

Rules: <br>
1. Each value in Rust has a variable that's called its *owner*.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

For **string literals**, the textis hardcoded directly into the final executable, enabling us to store this data on the stack. Although its immutability limits what we can do with this data type, storing string literals on the stack means that string literals are fast and efficient. Conversely, **String type** supports a mutable, growable piece of text by allocating memory on the heap, unknown at compile time, to hold the contents. This means that the memory must be requested from the OS at runtime. 

In Rust, garbage collection is managed by deallocating memory in the heap whenever a variable goes out of scope. When a variable goes out of scope, Rust calls ```drop``` to deallocate its memory. This pattern of deallocating resources at the end of an item's lifetime is known as *Resource Acquisition is Initializaion (RAII)*.

If you've heard the terms *shallow copy* and *deep copy* while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. However, Rust only enables moves and not shallow copies because the pointer is invalidated as soon as the data is removed. This prevents *double free error*.

Rust has a special annotation called the ```Copy``` trait that we place on types like integers that are stored on the satck. If a type has the ```Copy``` trait, an older variable is still usable after assignment. Rust won't let us annotate a type with the ```Copy``` trait if the type, or any of its parts, has implemented the ```Drop``` trait. Here are some of the types that are ```Copy```: <br>
* All the integer types, such as ```u32```
* The Boolean type, ```bool```, with values ```true``` and ```false```.
* All the floating point types, such as ```f64```.
* The character type, ```char```.
* Tuples but only if they contain types that are also ```Copy```. For example, ```(i32, i32)``` is ```Copy```, but not ```(i32, String)```.

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by ```drop``` unless the data has been moved to be owned by another variable.

---

**References and Borrowing** <br>

We use **references** to pass a value to a function without transferring ownership.

```
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```
Using references as function parameters is known as **borrowing**. References are immutable by default, but mutable references are possible. The only limitation is that you can only have one mutable reference per piece of data in a scope. 
> We can use curly brackets to create a new scope, allowing for multiple mutable references, just not *simultaneous* ones.

This allows for mutation, but in a very controlled fashion. This allows Rust to prevent **data races** at compile time. A data race is a race condition in which: <br>
* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There's no mechanism being used to synchronize access to the data.

---

**Slices**<br>
```
let hello = &s[0..5];
let world = &s[6..11];
```
```
let hello = &s[0..=4];
let world = &s[6..=10];
```
They're the same; the top is noninclusive for the last number and the second syntax is inclusive for the second number.

String literals are immutable because ```&str``` (string literal type) is an immutable reference.

## Basic Programming Concepts <a name="basics"></a>
* [The Stack and the Heap](#stackheap)
* [Using Modules to Reuse and Organize Code](#modules)

### The Stack and the Heap <a name="stackheap"></a>
In a systems programming language like Rust, whether a value is on the stack or the heap influences how you can interact with it.

The **stack** stores values in the order it gets them and removes the values in the opposite order (LIFO = last in, first out). The stack is fast because of the way it accesses the data: it never has to search for a place to put new ata or a place to get data from because it can only pop or push values from the top of the stack. All data on the stack must take up a known, fixed size.

When the size of data is unknown at compile time, this data is stored on the **heap**. To put data on the heap, you request some space from the OS and the OS provides a pointer to the address of that location. This process is called *allocating on the heap*. Accessing data in the heap is slower than accessing data on the stack because you have to follow a point to get there. Allocating a large amount of space on the heap can also take time

### Using Modules to Reuse and Organize Code <a name="modules"></a>
A **module** is a namespace that contains definitions of functions or types, and you can choose whether those defintions are visible outside their module using the ```pub``` keyword. 
* The ```mod``` keyword declares a new module.
* By default, functions, types, constants, and modules are private. 
* The ```use``` keyword brings modules or the definitions inside modules, into scope so it's easier to refer to them.

The rules of modules with respect to files:
* If a module named ```foo``` has no submodules, you should put the declarations for ```foo``` in a file named *foo.rs*.
* If a module named ```foo``` does have submodules, you should put the declarations for ```foo``` in a file named *foo/mod.rs*.

**Privacy Rules**: <br>
* If an item is **public**, it can be accessed through any of its parent modules.
* If an item is **private**, it can be accessed only by its immediate parent module and any of the parent's child modules.

**Bringing Names into Scope with the ```use``` Keyword** <br>
```
pub mod laugh {
    pub mod at {
        pub mod me {
            pub fn now() {}
        }
    }
}

use laugh::at::me;

fn main() {
    me::now();
}
```
The ```use``` keyword brings only what we've specified into scope: it does not bring children of modules into scope.

Because enums also form a similar namespace to modules, we can bring an enum's variants into scope with ```use``` as well. For any kind of ```use``` statement, if you're bringing multiple items from a namespace into scope, you can use this syntax:
```
enum Fruits {
    Apple,
    Orange,
    Grape,
}

use Fruits::{Apple, Orange};

fn main() {
    let apple = Apple;
    let orange = Orange;
    let grape = Fruits::Grape;
}
```

We can bring all items in a namespace into scope at once using the ```*``` (glob) syntax.
```
use Fruits::*;
```

To access a parent module, you use ```super``` to move up one module in the hierarchy from the current module
```
super::individual::work();
```

We can also use leading colons to let Rust know that we want to start from the root and list the whole path:
```
::individual::work();
```

## Data Structures <a name="datastructures"></a>
* [Structs](#structs)
* [Enums and Pattern Matching](#enums)
* [Common Collections](#collections)

### Structs <a name="structs"></a>
This shows how the struct update syntax looks.
```
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

**Methods** are similar to functions: except they're defined within the context of a struct and their first parameter is always ```self``` (which represents the instance of the struct the method is being called on).
```
struct Block {
    hash_pointer: u32,
    merlkle_root_hash: u32,
}

impl Block {
    fn printMerkle(&self) -> u32 {
        println!("Merkle root hash is {}", self.merkle_root_hash);
        self.merkle_root_hash
    }
}
```

The main benefit of using methods instead of functions, in addition to using method syntax and not having to repeat the type of ```self``` in every method’s signature, is for organization. We’ve put all the things we can do with an instance of a type in one ```impl``` block rather than making future users of our code search for capabilities of ```Rectangle``` in various places in the library we provide.

Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. Methods let you specify the behavior that instances of your structs have, and associated functions let you namespace functionality that is particular to your struct without having an instance available.

## Enums and Pattern Matching <a name="enums"></a>
```
enum Mood {
    Happy,
    Sad,
    Mad,
}
```
Mood is now an enumeration type with variants ```{Happy, Sad, Mad}```. The variants of the enum are namespaced under the identifier (```Mood```). This allows us to define special functions that take any variant of the same enum by specifying the enum type for the function's input.

The way we create instants of the variats is with double colons:
```
let amars_happy = Mood::Happy;
let amar_mad = Mood::Mad;
```

Advantage for **Enums over Structs**: Enums can have variants of different types and amounts of associated data.

A common code pattern is to define structs and use these structs to store data in the variants of an enum:
```
struct ProofOfStake {
    cap_req:        u32,
    inflation_rate: u32,
    tps:            u32,
    -- more code -- 
}

struct ProofOfWork {
    hash_rate:      u32,
    avg_fee:        u32,
    tps:            u32,
    -- more code --
}

enum BlockchainConsensus {
    Ethereum(ProofOfStake),
    Bitcoin(ProofOfWork)
}
```

**Option Enum in Rust**<br>
In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:
> I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

```
enum Option<T> {
    Some(T),
    None,
}
```

Because the ```Option<T>``` enum is so useful, it is does not need to be brought into scope and you can use its variants directly instead of referencing the identifier.
```
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```
When we have a ```Some``` value, we know that a value is present and the value is held within the ```Some```. When we have a ```None``` value, in some sense, it means the same thing as null: we don’t have a valid value. So why is having ```Option<T>``` any better than having null?

In short, because ```Option<T>``` and ```T``` (where ```T``` can be any type) are different types, the compiler won’t let us use an ```Option<T>``` value as if it were definitely a valid value. For example, this code won’t compile because it’s trying to add an ```i8``` to an ```Option<i8>```:
```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

**How is this better than having a null value?**<br>
In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value ```Option<T>```. Then, when you use that value, you are required to explicitly handle the case when the value is ```null```. Everywhere that a value has a type that isn’t an ```Option<T>```, you can safely assume that the value isn’t ```null```. This was a deliberate design decision for Rust to limit ```null```’s pervasiveness and increase the safety of Rust code.

---
**Match Expressions**
```
enum StudentYear {
    Freshman,
    Sophomore,
    Junior,
    Senior(Job),
}

fn class_number(studentYear: StudentYear) -> i8 {
    match studentYear {
        StudentYear::Freshman => 1,
        StudentYear::Sophomore => 2,
        StudentYear::Junior => 3,
        StudentYear::Senior => 4,
    }
}
```
Each of the match arms has two parts <br>
1. Pattern - the enum variant in this case
2. Arm - code to run if the pattern is matched

**Concise Control Flow with ```if let```**<br>
The ```if let``` syntax lets you combine ```if``` and ```let``` into a less verbose way to handle values that match one pattern while ignoring the rest. 
```
if let StudentYear::Senior = studentYear {
    println!("Student is in their {}th year", 4);
} else {
    println!("Student is NOT in their {}th year", 4);
}
```

## Common Collections <a name="collections"></a>

Collections can contain multiple values and the data that collections point to are **stored in the heap**.

[Collections](https://doc.rust-lang.org/std/collections/index.html) can be grouped into four major categories: <br>
* Sequences:  Vec, VecDeque, LinkedList <br>
* Maps:       HashMap, BTreeMap <br>
* Sets:       HashSet, BTreeSet <br>
* Misc:       BinaryHeap <br>


> "To get this out of the way: you should probably just use Vec or HashMap. These two collections cover most use cases for generic data storage and processing. They are exceptionally good at doing what they do. All the other collections in the standard library have specific use cases where they are the optimal choice, but these cases are borderline niche in comparison. Even when Vec and  HashMap are technically suboptimal, they're probably a good enough choice to get started." ~ [std::collections documentation](https://doc.rust-lang.org/std/collections/index.html)

Likewise, we'll discuss the three most common collections seen in Rust programs:<br>
* A **vector** allows you to store a variable number of values next to each other.
* A **String** is a collection of characters.
* A **hash map** allows you to associate a value with a particular key. It's a particular implementation of the more general data structure called a *map*.

---
```Vec<T>``` (vector) can only store values of the same type.  To create a new, empty vector, we call the ```Vec::new``` function.
```
let v: Vec<i32> = Vec::new(); //create a new, empty vector for holding values of type i32
```

We can also use the ```vec!``` macro to create a ```Vec<T>``` that has initial values. We can access the items in a vector using either the ```get``` method or the indexing syntax: <br>
```
let v = vec![1, 2, 3, 4, 5];
let s = &v[4];             //indexing syntax
let v_index = 2;

match v.get(v_index) {
    Some(_) => { println!("Reachable element at index: {}", v_index); },
    None => { println!("Unreachable element at index: {}", v_index); }
}
```

We can use an [enum](#enums) to store multiple types in a vector.

---
Strings are implemented as a collection of bytes. Rust only has one string type in the core language, the string slice ```str``` that is usually seen in its borrowed form ```&str```. Conversely, the ```String``` type is provided by Rust's standard library, and is a growable, mutable, owned UTF-8 encoded string type. To create a new, empty ```String```,
```
let mut s = String::new();
```
We can use the ```to_string``` method to create a ```String``` from a string literal.
```
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```
We can also use ```String::from``` to create a ```String``` from a string literal.
```
let s = String::from("initial contents");
```
We can grow a ```String``` by using the ```push_str``` mtethod to append a string slice.
```
let mut s = String::from("foo");
s.push_str("bar");
```
We can also use the ```+``` operator to add two strings, but this follows certain rules
```
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
```
Specifically...
```
fn add(self, s: &str) -> String {
```
Note that the compiler uses *deref coercion* to coerce ```&String``` into a ```&str```.

**Rust strings don't support indexing**. But why not? Let's consider **internal representation**. A ```String``` is a wrapper over a ```Vec<u8>```. UTF-8 complicates indexing because bytes don't always correspond to single letters. Therefore, Rust provides different ways of interpreting the raw string data that computers store so that each program can chose the interpretation it needs. 

The final reason that Rust doesn't allow us to index into a ```String``` to get a character is that indexing operations are expected to always take constant time ```(O(1))```. But it isn't possible to guarantee that performance with a ```String```, because Rust would have to iterate through all the contents to determine how many valid characters were therein.

---
The type ```HashMap<K, V>``` stores a mapping of keys of type ```K``` to values of type ```V```.

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10)
```
 This isn't as common as ```String``` or ```Vec<T>``` so it must be brought into scope with the ```use``` keyword.

 Like vectors, hash maps are homogeneous; all of the keys must have the same type, and all of the value must also have the same type.

 Another way of constructing hash maps is by using the ```collect``` method on a vector of tuples, where each typle consists of a key and a value. The ```collect``` method gathers data into a number of collection types, including ```HashMap```. 
 ```
use std::collections::HashMap;

let colors  = vec![String::from("Blue"), String::from("Yellow")];
let numbers = vec![8, 1];

let colorToNumbers: HashMap<_, _> = colors.iter().zip(numbers.iter()).collect();
 ```
The type annotation ```HashMap<_, _>``` is needed here because it's possible to ```collect``` into many different data structures; the underscores enable Rust to infer the types based on the data in the vectors.

**Hash Maps and Ownership**: For types that implement the ```Copy``` trait, like ```i32```, the values are copied into the hash map. For owned values like ```String```, the values will be moved and the hash map will be the owner of those values.

We can get a value out of the hash map by providing its key to the ```get``` method:
```
use std::collections::HashMap;

let mut ppg = HashMap::new();

ppg.insert(String::from("LBJ"), 30);
ppg.insert(String::from("Jordan"), 40);
ppg.insert(String::from("Amar"), 100);

let Jordan = String::from("Jordan");
let JordanPoints = ppg.get(&Jordan);
-- we also need to handle the Option<&V> returned with match presumably --
```
We can iterate over each key/value pair in a hash map using a ```for``` loop:
```
use std::collections::HashMap;

let mut totalETH = HashMap::new();

totalEth.insert(String::from("Amar"), 100);
totalEth.insert(String::from("Omar"), 1000000);

for (key, value) in &totalEth {
    println!("{}: {}", key, value);
}
```

To update a Hash Map, we can <br>
* replace the old value with the new value <br>
* keep the old value and ignore the new value (only adding if the key doesn't already have a value) <br>
* combine the old value and the new value <br>

## Error Handling <a name="error"></a>

Rust doesn't have *exceptions* like other languages. Instead, Rust has the type ```Result<T, E>``` for recoverable errors and the ```panic!``` macro that stops execution when the program encounters an unrecoverable error. 

---
When the ```panic!``` macro executes, your program will print a failure message, unwind, clean up the stack, and then quit. **Unwinding** in this context means that Rust walks back up the stack and cleans up the data from each function it encounters. The alternative to this is to immediately *abort*, which ends the program without cleaning up. 

We can set the ```RUST_BAKCTRACE``` environment variable to get a backtrace of exactly what happened to cause an error. A *backtrace* is a list of all the functions that have been called to get to this point. Backtraces in Rust work as they do in other languages: start from the bottom and read until you see files you wrote to find the error.

---
**Recoverable Errors with ```Result```**<br>
In many cases, it is better to respond to a function failure instead of requiring the program to stop entirely. An example of when this may be useful would be when you're automating the editing of files and need to ensure that if a file does not exist, then a new file can be created and this new file will be edited accordingly.

The ```Result``` enum is defined as having two variants, ```Ok``` and ```Err``` <br>
```
enum Result<T, E> {
    Ok(T)
    Err(E)
}
```
The ```T``` and ```E``` are [generic type parameters](#generic). ```T``` represents the value that will be returned in the ```Ok``` variant in the successful case and, in the event of failure, ```E``` is returned in the ```Err``` variant. Let's consider how this would work in the context of validating transactions (like a light client).
```
use blockchain::spv::Transaction;

fn main() {
    let t = Transaction::open(tx_data);

    let t = match t {
        Ok(tx_data) => tx_data,
        Err(error) => {
            panic!("There was a problem verifying the transaction data: {:?}", error)
        },
    };
}
```
> Like the ```Option``` enum, the ```Result``` enum and its variants have been imported in the prelude (so it is unnecessary to specify ```Result:::``` before the ```Ok``` and ```Err``` variants in the ```match``` arms). 

We can definitely improve the previous code by adding other match arms to property handle the error. 

```
use blockchain::spv::Transaction;
use std::io::ErrorKind;

fn main() {
    let t = Transaction::open(tx_data);

    let t = match t {
        Ok(tx_data) => tx_data,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match Transaction::create("fake tx data") {
                Ok(fc) = fc,
                Err(e) => panic!("Tried to create transaction, but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the transaction: {:?}", other_error),
        },
    };
}
```
The type of the value returned by ```Transaction::open``` is ```io:Error``` (a struct provided by the standard library). This struct has a method ```kind``` that we can call to get an ```io::ErrorKind``` value. The enum ```io:ErrorKind``` is provided by the standard library and has variants representing the different kinds of errors that might result from an ```io``` operation. 

> The ```Result<T, E>``` type has many methods that accept a closure, and are implemented as ```match``` statements. **This will be discussed later and I WILL ADD AN INTERNAL LINK HERE**.

The ```Result<T, E>``` has helper methods, including ```unwrap``` and ```expect```. 

If the ```Result``` value is the ```Ok``` variant, ```unwrap``` will return the value inside the ```Ok```. If the ```Result``` is the ```Err``` variant, ```unwrap``` will call the ```panic!``` macro for us.
```
use blockchain::spv::Transaction;

fn main() {
    let t = Transaction::open("tx_data").unwrap();
}
```

```expect``` is similar to ```unwrap``` except it allows us to specify the ```panic!``` error message. This enables us to provide more specific error messages to track down the source of a panic.
```
use blockchain::spv::Transaction;

fn main() {
    let t = Transaction::open("tx_data").unwrap();
}
```

---
**Propagating Errors** <br>
If writing a function that relies on calls to other functions, we can return the error to the calling code. This is more commonly referred to as *propagating* the error and gives more control to the calling code. We can use the ```?``` operator to return errors to the calling code.

```
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("firecracker.md")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

If the value of ```Result``` is an ```Ok```, the value inside the ```Ok``` will get returned from this expression and the program will continue. If the value is an ```Err```, the ```Err``` will be returned from the whole function as if we had used the ```return``` keyword so the error value gets propagated to the calling code.

> The error values taken by ```?``` go through the ```from``` function, defined in the ```From``` trait in the standard library (which is used to convert errors from one type into another). When ```?``` calls the ```from``` function, the error type is converted into the error type defined in the return type of the current function. As long as each error type implements the ```from``` function to define how to convert itself to the returned error type, ```?``` takes care of the conversion automatically.

The ```?``` operator can only be used in functions that have a return type of ```Result```, because it is defined to work in the same way as the ```match``` expression. 

When we choose to return a ```Result``` value, we give the calling code options rather than making the decision for it. The calling code can choose to attempt to recover in a way that's appropriate for its situation or, conversely, it could decide than an ```Err``` value in this case is unrecoverable (and thereforem call ```panic!``` and turn the recoverable error into a recoverable one). With this in mind, returning ```Result``` is a more responsible choice when defining how a function might fail.

[To ```panic!``` or Not to ```panic!```](https://doc.rust-lang.org/book/2018-edition/ch09-03-to-panic-or-not-to-panic.html#examples-prototype-code-and-tests)

In general, it's advisable to allow the code to panic whenever the code could end up in a bad state. In this context, a *bad state* is when some assumption, guarantee, contract or invariant has been broken i.e.
* The bad state is not something that's expected to happen occasionally
* The code after this point relies on not being the bad state
* There's no good way to encode this information in the types you use

Functions often have *contracts*: their behavior is only guaranteed if the inputs meet particular requirements. Panicking when the contract is violated makes sense because a contract violation always implies a caller-side bug and it's not the type of error that you would want the calling code to explicitly handle.

---
**Validating Unique Types** <br>
We can make a new type and put the validations in a function to create an instance of the type rather than repeating the validations everywhere. 

```
pub struct Topic {
    rank: u32,
}

impl Topic {
    pub fn new(rank: u32) -> Topic {
        if rank < 1 || rank > 100 {
            panic!("The rank must be between 1 and 100 becaus we say so, got {}", rank);
        }

        Topic {
            rank
        }
    }

    pub fn rank(&self) -> u32 {
        self.value
    }
}
```

## Generics <a name="generic"></a>

Similar to how the function body can operate on an abstract ```list``` instead of specific values, generics allow code to operate on abstract types.

---
**Function Definitions**: <br>

We place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. To parameterize the types, we need to name the *type parameter*. You can use any identifier as a type parameter name.

Let's say we first have the following code that we want to clean up by using generics
```
fn smallest_int(list: &[i32]) -> i32 {
    let mut smallest = list[0];

    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn smallest_char(list: &[char]) -> char {
    let mut smallest = list[0];

    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
```

To define the generic ```smallest``` function
```
fn smallest<T>(list: &[T]) -> T {
```
The function body could be the exact same as the previous two functions (except we have not specified if ```T``` has not implemented the ```std::cmp::PartialOrd``` trait). The ```smallest``` function is generic over some type ```T``` and will return a value of the same type ```T```.

---
**In Struct Definitions**<br>

```
struct Point<T> {
    x: T.
    y: T,
}

fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point { x: 1.0, y: 4.0};
}
```
However, this implementation requires that ```x``` and ```y``` are the same type. We can further generalize this generic struct by using multiple generic type parameters.  

```
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mix = Point {x: 5, y: 3.2};
}
```

---
**Enum Definitions**<br>

Reconsider the ```Option<T>``` enum:
```
enum Option<T> {
    Some(T),
    None,
}
```
This definition makes more sense now. ```Option<T>``` is an enum that is generic over type ```T``` and has two variants: ```Some``` (which holds one value of type ```T```) and a ```None``` variant that doesn't hold any value. Enums can also use multiple generic types like the ```Result``` enum
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
The ```Result``` enum is generic over two types: ```T``` and ```E```, and has two variants: ```Ok``` (which holds a value of type ```T```) and ```Err``` (which hyolds a value of type ```E```).

---
**Performance of Code Using Generics**<br>
Rust implements generics in such a way that your code doesn't run any slower using generic types than it would with concrete types. Rust accomplishes this by performing *monomorphization* of the code that uses generics at compile time. **Monomorphization** is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

> Because Rust compiles generic code into code that specifies the type in each instance,we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust's generics extremely efficient at runtime.

### Traits <a name="traits">
A *trait* tells the Rust compiler about functionality a particular type has and can share with other types. We use traits to define shared behavior in an abstract way. We use trait bounds to specify that a generic can be any type that has certain behavior.

Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose. As an example, let's assume that we have two structs: ```Twitter``` and ```Reddit``` that hold posts from each. If we want to display summaries of the data from each struct, we can use a ```trait``` to express this behavior in a clean and compact way.
```
pub trait Summary {
    fn summarize(&self) -> String;
}
```
Here we declare a trait using ```trait``` keyword and the the trait's name ```Summary```.

After each method signature, instead of providing an implementation with the trait's name, we use a semicolon. Similar to ```interface``` in other languages, each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the ```Summary``` trait will have the method ```summarize``` defined with this signature exactly.

> A trait can have multiple methods in its body. The method signatures are listed one per line such that each method signature ends in a semicolon.

---
**Implementing a Trait on a Type**<br>
Let's actually implement the ```Summary``` trait on the types for our project. 
```
pub struct Reddit {
    pub title: String,
    pub description: String,
    pub subreddit: String,
    pub OP: String,
    pub content: String,
}

impl Summary for Reddit {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.title, self.OP, self.subreddit)
    }
}

pub struct Twitter {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Twitter {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

After implementing this trait, we can call the methods on instances of ```Reddit``` and ```Tweet``` types in the same file i.e.

```
let tweet = Twitter {
    username: String::from("asingchrony"),
    content: String::from("Bullish on ETH in short-term"),
    reply: false,
    retweet: false,
}

println!("1 new tweet: {}", tweet.summarize());
```

If our crate was called ```aggregator``` and someone else wanted to use our crate's functionality, then they would need to import the trait into their scope first. This can be done by specifying ```use aggregator::Summary```, which then would enable them to implement ```Summary``` for their type. The ```Summary``` trait would also need to be a public trait for another crate to implement it, which it is because we put the ```pub``` keyword before ```trait```.

We can implement a trait on a type only if either the trait or the type is local to our crate. Therefore, we **cannot implement external traits on external types**. This restriction is better known as *coherence*. It is also commonly referred to as the *orphan rule* because the parent type is not present. This essentially guarantees against two crates implementing the same trait for the same type (in which case, Rust would not be able to determine which implementation to use).

Sometimes it's useful to establish default behavior for some or all methods in a trait. Under this paradigm, as we implement the trait on a particular type, we can keep or override each metho's default behavior.

```
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

If a type uses the default implementation of a trait's methods, we can specify an empty ```impl``` block with ```impl Summary for Twitter {}```. With this in mind, we can still call the summarize method on an instance of ```Twitter```
```
let tweet = Twitter {
    username: String::from("smerklenetwork"),
    content: String::from("Join CS1501"),
    reply: false,
    retweet: false,
};

println!("New tweet available: {}", tweet.summarize());
```

> Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation. Even so, it is not possible to call the default implementation from an overriding implementation of that same method.

---
**Traits as arguments**<br>
We previously implemented the ```Summary``` trait on the types ```Twitter``` and ```Reddit```. We can define a function ```notify``` that calls the ```summarize``` method on its parameter ```item```, which is of some type that implements the ```Summary``` trait. This is better known as the ```impl Trait``` syntax.
```
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
In the body of ```notify```, we can call any methods on ```item``` that come from the ```Summary``` trait (ie ```summarize```).

---
**Trait Bounds**<br>
The ```impl Trait``` syntax works for shorter examples, but it is better to handle longer forms with a *trait bound*. 
```
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news: {}", item.summarize());
}
```
This is equivalent to the previous example. We place trait bounds with the declaration of the generic type parameter, after a colon and inside angle brackets. 
> It is usually preferrable to use trait bounds when the implementation is more complex and ```impl Trait``` syntax when it is shorter (or more simple).

We can specify multiple trait bounds on a generic type using the ```+``` syntax. As an example, if we wanted to use display formatting on the type ```T``` in a function as well as the ```summarize``` method, we can use ```T: Summary + Display``` to say ```T``` can be any type that implements ```Summary``` and ```Display```.
```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

Rust also has an alternate syntax for specifying the trait bounds inside a ```where``` clause at the function signature. We can use ```where``` clause like this
```
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```
This makes it so that the function signature is less cluttered.

> Note that we can also use the ```impl Trait``` syntax in the return position. This enables us to return something that implements the given trait but it does not specify which specific type. 
```
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("A6staking"),
        content: String::from("staking on cosmos"),
        reply: false,
        retweet: false,
    }
}
```

This only works if you have a single type that you're returning (single type that implements the given trait).

Here is a version of the ```smallest``` function that will compile as long as the types of the values in the slice that we pass into the function implement the ```PartialOrd``` and ```Copy``` traits, like ```i32``` and ```char``` do.

```
fn smallest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut smallest = list[0];

    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
```
This works for any generic type that implements the ```PartialOrd``` and ```Copy``` traits.

We can also use trait bounds to conditionally implement methods. Consider this example
```
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called *blanket implementations*.

### Lifetimes <a name="lifetimes"></a>
A reference's **lifetime** is the scope for which that reference is valid.

Rust has a *borrow checker* that compares scopes to determine whether all borrows are valid. Here's an example with some annotations that indicate variable lifetimes.
```
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```
At compile time, Rust compares the size of the two lifetimes and notices that ```r``` has a lifetime of ```'a``` but that it refers to memory with a lifetime of ```'b```. The program is rejected by the compiler because ```'b``` is shorter than ```'a```. In other words, the **compiler does not allow situations in which the subject of the reference does not love as long as the reference**. This eliminates the possibility of dangling references.

---
**Generic Lifetimes in Functions** <br>
Functions can accept references with any lifetime by specifying a generic lifetime parameter. Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

The names of lifetime parameters must start with an apostrophe(```'```) and are usually all lowercase annd relatively short (like generic types). Most Rustaceans use the name ```'a```. Weplace the lifetime parameter annotations after the ```&``` of a reference, using a space to separate the annotation from the reference's type.
```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```
> One lifetime annotation by itself has basically no meaning. Indeed, lifetime annotations are used to tell Rust how generic lifetime parameters of multiple references relate to each other. 

---
Let's consider how to implement the smallest function such that we specify that all the references in the function signature must have the same lifetime ```'a```.

```
fn smallest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
This function signature ensures that for some lifetime ```'a```, the function takes two parameters, both of which are string slices that must live at least as long as lifetime ```'a```. These constraints are enforced by Rust. Essentially, we're specifying that the borrow checker should reject any values that don't adhere to this constraint (lifetime of at least ```'a```).

So how does this work behind the scenes? When we pass concrete references to ```smallest```, the concrete lifetime that is substituted for ```'a``` is the concrete lifetime equal to the smaller of the lifetimes of ```x``` and ```y```. Because we've annotated the returned reference with the same lifetime parameter ```'a```, the returned reference will also be valid for the length of the smaller of the lifetimes of the ```x``` and ```y```.

> When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference returned does *not* refer to one of the parameters, it must refer to a value created within this function (which would be a dangling reference because the value will go out of scope at the end of the function).

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they're connected, Rust has enough information to enable memory-safe operations and disallow operations that could result in dangling pointers or data races.

---
**Lifetime Annotations in Struct Definitions** <br>
If we define structs that hold references, we need to add a lifetime annotation on every reference in the struct's definition. 

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() [
    let book = String::from("If X and Y are sets s.t. X is equivalent to a subset of Y, we say that Y dominates X");
    let first_sentence = book.split('.')
        .next()
        .expect("Could not find a '.'");
    let m = ImportantExcerpt { part: first_sentence};
]
```
This struct has one field, ```part```, that holds a string slice, which is a reference by defintion. As with generic types, we declare the name of the generic lifetime parameter inside angle brakckets after the name of the struct so that we an use the lifetime parameter in the body of the struct definition. This means that an instance of ```ImportantExcerpt``` cannot outlive the reference it holds in its ```part``` field.

---
**Lifetime Elision**<br>

The patterns programmed into Rust's analysis of references are called the *lifetime elision rules*. These detail cases in which the borrow checker can infer the program's lifetimes instead of requiring the programmer to explicitly declare lifetime.
> The elision rules still don't provide full inference. 

Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on return values are called *output lifetimes*. The compiler uses three **elision rules** to figure out what lifetimes references have when there aren't explicit annotations. 
1. each parameter that is a reference gets its own lifetime parameter
2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: ```fn foo<'a>(x: &'a i32) -> &'a i32```
3. if there are multiple input lifetime parameters, but one of them is ```&self``` or ```&mut self``` because this is a method, the lifetime of ```self``` is assigned to all output lifetime parameters

---
**Lifetime Annotations in Method Definitions**<br>
Lifetime names for struct fields always need to be declared after the ```impl``` keyword and then used after the struct's name, because those lifetimes are part of the struct's type. Here are two examples:

```
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```
The lifetime parameter declaration after ```impl``` and use after the type name is required, but we're not required to annotate the lifetime of the reference to ```self``` because of the first elision rule.

```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```
Because there are two input lifetimes, Rust applies the first elision rule and gives both ```&self``` and ```announcement``` their own lifetimes. Because one of the parameters is ```&self```, the return type gets the lifetime of ```&self``` and all lifetimes have been accounted for.

---
**The Static Lifetime**<br>
The ```'static``` lifetime denotes the entire duration of the program. All string literals have the ```'static``` lifetime, which we could annotate as
```
let s: &'static str = "I have a static lifetime.";
```
The text of this string is stored directly in the program's binary (which is always available).

## Testing <a name="testing"></a>

The bodies of test functions typically perform these three actions:
1. Set up any necessary data or state
2. Run code that is being tested
3. Assert the results are what you expect

Usually in our src/lib.rs or src/main.rs files, we'll have a test module with this syntax
```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
```
The ```#[test]``` attribute above the function signature indicates that this is a test function so that the test runner can treat this function as a test. 

> We could also have non-test functions in the ```tests``` module to help set up common scenarios or perform common operations.

The ```assert!``` macro is very useful when you want to ensure that some condition in a test evaluates to ```true```. Specifically, we provide the ```assert!``` macro with an argument that evaluates to a Boolean. If the value passed is ```true```, then nothing happens, but if it is ```false``` then the ```assert!``` macro calls the ```panic!``` macro (which then causes the test to fail).

It is common to add the line ```use super::*;``` in the ```tests``` module. The ```tests``` module is a regular module (and it is an inner module) so we need to bring the code in the outer module into the scope of the inner module.

A common test code pattern is to compare the result of the code under the test to the value you expect the code to return (and checking if they're equal). Although we could do this with the ```assert!``` macro, we can use the following pair of macros to test equality or inequality, respectively -- ```assert_eq!``` and ```assert_ne!```. It is useful to use the ```assert_ne!``` when you are testing state changes.

For structs and enums that we define, we must implement ```PartialEq``` to assert values of those types are equal or not equal. In addition, you'd need to implement ```Debug``` to print the values when the assertion fails. Because both traits are derivable traits, we can implement them in most cases by just adding the ```#[derive(PartialEq, Debug)]``` annotation to your struct or enum definition.

We cn also add the attribute ```should_panic``` to our test function. This attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn't panic.
```
pub struct Topic {
    score: u32,
}

impl Topic {
    pub fn new(score: u32) -> Topic {
        if score < 1 || score > 100 {
            panic!("The topic score must be between 1 and 100, but we go {}", score);
        }

        Topic {
            score
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn score_over_100() {
        Topic::new(101);
    }
}
```
We place the ```#[should_panic]``` attribute after the ```#[test]``` attribute and before the test function it applies to.

Another common testing code pattern is to use ```Result<T, E>``` in tests.
```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four))
        }
    }
}
```

### Running Tests <a name="runtests"></a>
The default behavior of the binary produced by ```cargo test``` is to run all the tests in parallel and capture output generated during test runs, thereby preventing the output from being displayed and making it easier to read the output related to the test results.

To pass commmand line arguments to ```cargo test```, we list the arguments that go to ```cargo test``` followed by the separator ```--``` and then the ones that go to the test binary. Running ```cargo test --help``` displayes the options you can use with ```cargo test``` and running ```cargo test -- --help``` displays the options that can be used after the separator ```--```.

Sometimes it is useful to run tests consecutively instead of in parallel (especially if you're manipulating some common object and monitoring the state changes for testing reasons). In this case, we can send the ```--test-threads``` flag and the number of threads you want to use to the test binary.
```
cargo test -- --test-threads=1
```

By default, Rust's test library captures anything printed to standard output. To prevent this (if we're printing stuff within our test functions), we can pass the ```--nocapture``` flag:
```
cargo test -- --nocapture
```

We can also pass the name of any test function to ```cargo test``` to run only that test:
```
cargo test test_function_name
```
We can also filter to run multiple tests by only passing part of a test name such that any test whose name contains that value will be run.

Rather than listing as arguments all tests that you don't want to run, we can annotate the time-consuming tests using the ```ignore``` attribute to exclude them. 
```
#[test]
#[ignore]
fn expensive_test() {
    // some code that we don't want to actually test at the moment
}
```
If we want to run only the ignored tests, we can use the ```--ignored``` flag
```
cargo test -- --ignored
```

### Unit Tests <a name="unittests"></a>
Unit tests focus on testing one module in isolation at a time and can test private interfaces. By testing each unit of code in isolation, we can pinpoint where code is and is not working as expected. You can put unit tests in each file with the code that is being tested. The convention is to create a module named ```tests``` in each file to contain the test functions and to annotate the module with ```cfg(test)```. The attribute ```cfg``` stands for *configuration* (so it tells Rust that the following item should only be included given a certain configuration option).

The ```#[cfg(test)]``` annotation on the tests module ensures that the test code only runs when you run ```cargo test```.

### Integration Tests <a name="integrationtests"></a>
Integration tests are entirely external to your library and use your code in the same that any other external code would, using only the public interface and potentially exercising multiple modules per test.

To create integration tests, you set up a *tests* directory at the top level of the project directory (same level as *src/*). Because we are testing the library functionality, we must import our library into integration tests like this
```
extern create adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```
We do not need to annotate any code in our file with ```#[cfg(test)]```. Cargo treats the ```tests``` directory as special and compiles the files when we run ```cargo test```.

To run all the tests in a particular integration test file, use the ```--test``` argument of ```cargo test``` followed by the file name. Note that we cannot run integration tests on a binary crate. Library crates expose functions that other crates can call and use while binary crates run on their own.