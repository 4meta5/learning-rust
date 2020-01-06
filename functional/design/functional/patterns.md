# Design Patters (CH 7)

**The single responsibility** principle states that program's logical components should do one thing and do that one thing well.

* [Functor](#func)
* [Monad](#burrito)
* [Combinator](#comb)
* [Lazy Evaluation](#lazy)

## Functor Pattern <a name = "func"></a>

A *function* defines a transformation, accepts data, and returns the result of the transformation.

A *functor* defines data, accepts a function, and returns the result of the transformation.

`map` in Rust is a *structure-preserving map* and it is the simplest example of a common functor. 

```rust
fn main() {
    let m: Vec<u64> = vec![1, 2, 3];
    let n: Vec<u64> = m.iter().map(|x| {x * x}).collect();
    println!("{:?}", m);
    println!("{:?}", n);
}
```

However, functors do not need to be structure-preserving.

The defining properties of a functor include:
* a collection of objects `C`
* a mapping function that will transform objects in `C` into objects in `D`

> **contravariant** vs **covariant** functors

## Monad Pattern <a name = "burrito"></a>

A monad defines `return` and `bind` operations for a type. 
* The `return` operation is like a constructor to make the monad. 
* The `bind` operation incorporates new information and returns a new monad.

There are *[several rules](https://en.wikipedia.org/wiki/Monad_(functional_programming))* which can basically be summarized as: monads should behave well when the operators are chained.

This sounds a lot like combinators and adaptors -- indeed, Rust maintains several semi-monads in the standard library.

```rust
fn main () {
    let v1 = Some(2).and_then(|x| Some(x + x)).and_then(|y| Some(y*y));
    println!(":?", v1);
    
    let v2 = None.or_else(|| None).or_else(|| Some(222));
    println!("{:?}", v2);
}
```

Monadic `bind` operations are polymorphic -- they should permit returning monads of different types from the current monad. 

**History**
Monads were originally developed to express side-effects in purely functional languages. If the effects are passed as input and output through pure functions, every function would need to declare every state variable and pass it along `=>` long list of parameters.

Monads *solve* by hiding state inside itself, which becomes essentially a larger, more complex function than what the programmer interacts with.

### Universal Logger Example

The monadic `return` and `bind` can be used to wrap state and computation inside of a monad that will log all intermediate results:

```rust
use std::fmt::{Debug};

struct LogMonad<T>(T);

impl<T> LogMonad<T> {

    fn _return(t: T) -> LogMonad<T>
        where T: Debug
    {
        println!("{:?}", t);
        LogMonad(t)
    }

    fn bind<R, F>(&self, f: F) -> LogMonad<R>
        where 
        F: FnOnce(&T) -> R,
        R: Debug
    {
        let r = f(&self.0);
        println!("{:?}", r);
        LogMonad(r)
    }
}

fn main() {
    LogMonad::_return(4)
        .bind(|x| x+x)
        .bind(|y| y*y)
        .bind(|z| format!("{}{}{}", z, z, z));
}
```

### Lazy Evaluation via Monad

Monads are useful for chaining together code that can't be written in a normal code block. 

Code blocks are always evaluated eagerly.

Lazy evaluation allows you to delay defining code until it is referenced -- this is different from the typical eager evaluation of Rust code.

```rust
struct LazyMonad<A, B>(Box<Fn(A) -> B>);

impl<A: 'static, B: 'static> LazyMonad<A, B> {
    fn _return(u: A) -> LazyMonad<B, B> {
        LazyMonad(Box::new(move |b: B) b)
    }

    fn bind<C, G: 'static>(self, g: G) -> LazyMonad<A, C>
        where
        G: Fn(B) -> C
    {
        LazyMonad(Box::new(move |a: A| g(self.0(a))))
    }

    fn apply(self, a: A) -> B {
        self.0(a)
    }
}

fn main() {
    let notyet = LazyMonad::_return(())
                            .bind(|x| x+ 2)
                            .bind(|y| y*3)
                            .bind(|z| format!("{}{}", z, z));
    
    let nowdoit = notyet.apply(222);
    println!("nowdoit {}", nowdoit);
}
```

## Combinator <a name = "comb"></a>

A **combinator** is a function that takes other functions as arguments and returns a new function.

An easy example might be the composition operator, which chains two functions together:

```rust
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
    where
    F: 'static + Fn(A) -> B,
    G: 'static + Fn(B) -> C
{
    move |x| g(f(x))
}

fn main() {
    let fa = |x| x+1;
    let fb = |y| y*2;
    let fc = |z| z/3;
    let g = compose(compose(fa, fb),fc);
    println!("g(1) = {}", g(1));
    println!("g(12) = {}", g(12));
    println!("g(123) = {}", g(123));
}
```

### Parser Combinators

A parse combinator uses both the monad and combinator patterns. 
* The monadic `bind` functions are used to bind data from parsers that are later returned as a parse result.
* The combinators join parsers into a sequence, failover, or other patterns

```rust
#[macro_use]
extern crate chomp;
use chomp::prelude::*;

#[derive(Debug, Eq, PartialEq)]
struct Name<B: Buffer> {
    first: B,
    last: B,
}

fn name<I: U8Input>(i: I) -> SimpleResult<I, Name<I::Buffer>> {
    parse!{i;
        let first = tale_while1(|c| c!= b' ');
        token(b' ');// skipping this char
        let last = take_while1(|c| c!= b'\n');

        ret Name{
            first: first,
            last: last,
        }
    }
}

fn main() {
    let parse_result = parse_only(name, "Amar Singh\n".as_bytes()).unwrap();
    println!("first: {} last: {}", 
    String::from_utf8_lossy(parse_result.first),
    String::from_utf8_lossy(parse_result.last));
}
```

Each `let` binding corresponds to a combinator. Each semicolon corresponds to a combinator. The functions `take_while1` and `token` are both combinators that introduce parser monads. When the macro ends, we are left with an expression that processes the input to parse a result.

...another example included in the book

## Lazy Evaluation <a name = "lazy"></a>

Iterators are lazy -- they don't do anything until you collect or otherwise iterate over them.

```rust
use std::rc::Rc;

#[derive(Clone)]
struct LazyList<A: Clone> {
    buffer: Rc<Vec<A>>,
    index: usize,
}

impl<A: Clone> LazyList<A> {
    fn new(buf: Vec<A>) -> LazyList<A> {
        LazyList {
            buffer: Rc::new(buf),
            index: 0
        }
    }
    fn new(&self) -> Option<(LazyList<A>,A)> {
        if self.index < self.buffer.len() {
            let new_item = self.buffer[self.index].clone();
            let new_index = self.index + 1;
            Some((LazyList {
                buffer: Rc::clone(&self.buffer),
                index: new_index
            },new_item))
        } else {
            None
        }
    }
}

fn main() {
    let ll = LazyList::new(vec![1,2,3]);
   let (ll1,a1) = ll.next().expect("expect 1 item");
   println!("lazy item 1: {}", a1);

   let (ll2,a2) = ll1.next().expect("expect 2 item");
   println!("lazy item 2: {}", a2);

   let (ll3,a3) = ll2.next().expect("expect 3 item");
   println!("lazy item 3: {}", a3);

   let (ll2,a2) = ll1.next().expect("expect 2 item");
   println!("lazy item 2: {}", a2);
}
```

*By wrapping side-effects into lazy evaluated expressions, then turning them into monads, we create side-effect units. These units can then be manipulated and composed in a more functional style.*

### Functional Reactive Programming (FRP)

Elm and React are influenced heavily by this concept.

FRP is an extension of the side-effect/state monad example. Event handling, state transitions, and side effects can be turned into units of reactive programming.

```rust
struct ReactiveUnit<St, A, B> {
    state: Arc<Mutex<St>>,
    event_handler: Arc<Fn(&mut St, A) -> B>
}

impl<St: 'static, A: 'static, B: 'static>ReactiveUnit<St, A, B> {
    fn new<F>(st: St, f: F) -> ReactiveUnit<St,A,B>
        where F: 'static + Fn(&mut St, A) -> B
    {
        ReactiveUnit {
            state: Arc::new(Mutex::new(st)),
            event_handler: Arc::new();
        }
    }

    fn bind<G,C>(&self, g: G) -> ReactiveUnit<St, A, C>
        where G: 'static + Fn(&mut St, B) -> C
    {
        let ev = Arc::clone(&self.event_handler);
        ReactiveUnit {
            event_handler: Arc::new(move |st: &mut St,a| {
                let r = ev(st,a);
                let r = g(st,r);
                r
            })
        }
    }

    fn plus<St2: 'static,C: 'static>(&self, other: ReactiveUnit<St2,B,C>) -> ReactiveUnit<(Arc<Mutex<St>>,Arc<Mutex<St2>>),A,C> {
      let ev1 = Arc::clone(&self.event_handler);
      let st1 = Arc::clone(&self.state);
      let ev2 = Arc::clone(&other.event_handler);
      let st2 = Arc::clone(&other.state);
      ReactiveUnit {
         state: Arc::new(Mutex::new((st1,st2))),
         event_handler: Arc::new(move |stst: &mut (Arc<Mutex<St>>,Arc<Mutex<St2>>),a| {
            let mut st1 = stst.0.lock().unwrap();
            let r = ev1(&mut st1, a);
            let mut st2 = stst.1.lock().unwrap();
            let r = ev2(&mut st2, r);
            r
         })
      }
   }

   fn apply(&self, a: A) -> B {
      let mut st = self.state.lock().unwrap();
      (self.event_handler)(&mut st, a)
   }
}
```

There's a lot more; pick up here...