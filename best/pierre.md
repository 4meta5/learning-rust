# Let's Design Better APIs
> by [tomaka](https://github.com/tomaka)

*Remember: you're optimizing code for readability. Programming is not so much about implementing some specific logic as it is about making it in a straight-forward, understandable, and easy-to-change way.*

* [Documentation](#docs)
* [Design Principles](#principles)

## Documentation at the Root of every Module <a name = "docs"></a>

Every module should have documentation explaining
* the context of why this module is necessary
* what the code in the module does
* a general overview of how to use the code in the module

The objective of crate-level documentation is to give a brief overview of the code's general architecture without requiring the reader to parse thousands of lines of code.

### Document Corner Cases of All Methods

*Whenever you document a method, try to think of all the possible states your program can be in, and what kind of weird input could be passed to the method*

## Design Principles <a name = "principles"></a>

* [Split Error Conditions](#split)
* [Use Strong Typing](#strong)
* [Separate IO from non-IO](#io)
* [Avoid Inner Mutability](#inner)
* [Avoid Shared State](#shared)
* [Split Logic from Threading](#logic)
* [Split Data Structures from Logic](#data)

### Split Error Conditions between Multiple Function Calls

Imagine a function that returns an error for two possible error states

```rust
impl SomeDataStructure {
    /// Modifies the value of an entry
    ///
    /// Returns an error if the entry doesn't exist, or if the value isn't valid
    pub fn set_entry_value(&mut self, entry: K, value: V) -> Result<(), SetValueErr>;
}

// Example usage
my_object.set_entry_value(entry, value);
```

Instead, we ought to split it:

```rust
impl SomeDataStructure {
    /// Returns an object that represents an entry in the data structure
    pub fn entry_mut<'a>(&'a mut self, entry: K) -> Option<Entry<'a>>;
}

impl<'a> Entry<'a> {
    /// Modifies the value of the entry
    ///
    /// Returns an error if the value isn't valid
    pub fn set_value(&mut self, value: V) -> Result<(), SetValueErr>;
}

// Example usage:
if let Some(entry) = my_object.entry_mut(key) {
    entry.set_value(value)?;
}
```
Rather than having a single method that can fail for two different reasons, write two methods that can fail for a single reason each. This significantly simplifies some APIs.

## Use strong typing <a name = "strong"></a>

Reduce the complexity of a method by using strong typing i.e. don't write

```rust
network.connect("127.0.0.1:80")?;
```

Instead, write

```rust
let ip_addr: IpAddr = "127.0.0.1:80".parse()?;
network.connect(ip_addr)?;
```

This separates parsing the IP address from connecting to it.

Another example would be if the method returns an error when you pass 0, require a nonzero type to be passed as a parameter rather than a plain number.

## Don't Mix I/O and non-I/O <a name = "io"></a>

If you wanted to succinctly implement an HTTP server, your code might resemble the following:

```rust
pub struct HttpSocket {
    inner_socket: TcpStream,
}

impl HttpSocket {
    /// Reads some data from the socket and returns the next `Header` if possible
    pub fn get_next_header(&mut self) -> Poll<Header, HttpError> {
        let next_line = try_ready!(self.inner_socket.read_until("\n"));

        let mut elems_iter = next_line.splitn(2, ':');
        let mut header = elems_iter.next().unwrap();
        let mut value = elems_iter.next().ok_or(|| HttpError::BadHeaderFormat);
        Ok( Header {
            header,
            value,
        })
    }
}
```

This code performs IO and parsing in the same code, thereby increasing the difficulty of testing. 

Instead,

```rust
impl HttpStateMachine {
    /// Call this method when you read something from the socket
    pub fn on_read(&mut self, data: &[u8]) -> Result<(), HttpError>;

    /// Returns data that is ready to be written on the socket, if any. Should be called on `on_read`
    pub fn next_to_write(&mut self) -> Option<Vec<u8>>;

    /// Returns the list of known headers
    pub fn known_headers(&self) -> ...;
}

pub struct HttpSocket {
    inner_socket: TcpStream,
    state_machine: HttpStateMachine,
}

impl HttpSocket {
    fn update(&mut self) -> Poll<_, HttpError> {
        let buf = self.inner_socket.read_buf()?;
        self.state_machine.on_read(&buf)?;
        while let Some(buf) = self.state_machine.next_to_write() {
            self.inner_socket.write_buf(buf)?;
        }
        ...
    }
}
```

This approach makes the code in `HttpSocket` more clean and straightforward to read, easier to test, more reusable, and it separates IO and non-IO errors.

### Avoid Inner Mutability

*inner mutability* in this context refers to some object whose state can modify itself "automatically" when using it only through `&self` ie

```rust
let a = network.is_node_banned(&node_id);
let b = network.is_node_banned(&node_id);
// if a is not necessarily always equal to b, then you have a racy API
```

Including inner mutability adds corner cases thereby making the code more difficult to reason through

```rust
if network.is_node_banned(&node_id) {
    network.unban_node(&node_id);
    // What if the temporary ban expired between the call to `is_node_baned` and `unban_node`
}
```

People work around these issues by silecing corner cases. This makes it harder to write tests among other things....

A better solution is explicitly ask the object to update itself:

```rust
let a = network.is_node_banned(&node_id);
let b = network.is_node_banned(&node_id);
assert_eq!(a, b); // Always guaranteed.

network.update_bans();
let c = network.is_node_banned(&node_id);
// c is not necessarily equal to a/b
```

In the above code, node can only be unbanned when `update_bans` is called. Another advantage of this pattern is that you can define `update_bans` such that it returns the list of bans that have expired, thereby allowing us to maintain some other state in sync and remove the need for callbacks.

### Avoid Shared State and Mutexes

Manipulating some `Arc<Mutex<...>>` => interior mutability. Every time you lock the `Mutex`, the state of the locked object may have changed -- code must account for all the possible things that could have happened outside of its control since the previous locking.

The *alternative* requires some hierarchization of code. Instead of storing objects in `Arc`s with no clear owner, define a precise owner that ahdnles communications and state updates between pieces of code.

## Split Logic From Threading Strategy <a name = "time"></a>

Don't mix core logic with anything non-deterministic including threads (or time in general).

Prefer explicitly calling `update(&mut self, Instant::now())` on a state machine rather than having the state machine read `Instant::now()` itself.

## Split Data Structures from Logic <a name = "data"></a>

When the logic is complex, it is difficult to debug. Mixing data structures with logic exacerbates...

