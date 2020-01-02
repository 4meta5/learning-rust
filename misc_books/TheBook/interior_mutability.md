# Interior Mutability

**Interior mutability** allows you to mutate data even when there are immutable references to that data (*mutating the value inside an immutable value*). To mutate data, the pattern uses `unsafe` code inside a data structure to bend the borrowing rules.

`RefCell<T>` represents single ownership over the data it holds. 

With references and `Box<T>`, the borrowing rules' invariants are enforced at compile time. With `RefCell<T>`, these invariants are enforced *at runtime*. With references, if you break these rules, you'll get a compiler error; with `RefCell<T>`, if you break these rules, the program will panic and exit at runtime.

* advantage fo checking the borrowing rules at compile time are that errors are caught sooner and there is no impact on runtime performance
* advantage of checking at runtime is that certain memory-safe scenarios are allowed

*When to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:
* `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners
* `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime
* Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

## Example

Create a library that tracks a value against a maximum value,s ends messages based on how close to the maximum value the current value is.

**Use Case Idea**: To check whether the DilutionSafety bound is soon to be exceeded in `SunshineDAO`; checking whether the maximum number of charities/polls have been invoked for `ComPartido`

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger 
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90%");
        } else if percentage_of_max  0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

**I think the above pattern could be used for setting the `PorposalFee` dynamically according to the changes in `DilutionBound`...great pattern!**

> test with `RefCell` => https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

## Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`


