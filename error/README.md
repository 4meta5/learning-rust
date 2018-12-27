# Error Handling

* [OpenGL in Rust from Scratch with the Failure Crate](https://nercury.github.io/rust/opengl/tutorial/2018/02/15/opengl-in-rust-from-scratch-08-failure.html)

* [Starting with ErrorChain](https://brson.github.io/2016/11/30/starting-with-error-chain)

* [`ignore-result` crate](https://neosmart.net/blog/2018/rust-ignore-result/)


> [reddit comment from ghostopera](https://www.reddit.com/r/rust/comments/a9wbs8/comment/ecnosdi/?st=JQ6PFPK9&sh=9214a765):

Rust is still a bit awkward still when it comes to error handling. There is a lot that is great about it, but there are also many ways to handle it. It can be quite nice to work with, and also quite frustrating at times.

The following is how I tend to handle errors. You will note that I am not relying on crates as `error-chain` or `failure`. I do use the `Display` derive from `derive-more` as it's super useful.
```
#[derive(Debug, Display)]
#[display(fmt = "{}", kind)]
pub struct MyError
{
    pub kind: MyErrorKind,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

#[derive(Debug, Display)]
pub enum MyErrorKind
{
    #[display(fmt = "I couldn't do the thing")]
    Thing,
    #[display(fmt = "Failed to start server")]
    Startup,
}

// Granted, this and the next impl would be handy if wrapped up in a 
// derive macro.
impl Error for MyError
{
    fn source(&self) -> Option<&(dyn Error + 'static)>
    {
        self._source
            .as_ref()
            .map(|boxed| boxed.as_ref() as &(dyn Error + 'static))
    }
}

impl From<MyErrorKind> for MyError
{
    fn from(kind: MyErrorKind) -> Self
    {
        MyError {
            kind,
            _source: None,
        }
    }
}

// I'll often add these helpers to simplify map_err
impl MyError 
{
    fn thing(err: impl StdError + Send + Sync + 'static) -> Self
    {
        Error {
            kind: MyErrorKind::Thing,
            _source: Some(Box::new(err)),
        }
    }
}
```

So now I can do something like...

```
return Err(MyErrorKind::Startup.into());
```

Or

```
// I like using .map_err over relying on an impl From<T> as I can 
// provide context. This also gives me a point to match on if it's
// something I can recover from.
a_result
    .map_err(MyError::thing)?;
```

And then of course I can spit out all of the causes as a kind of 'backtrace' of my error during error logging

```
error!("Encountered an error: {}", err.to_string());
{
    let mut err: &Error = &err;
    while let Some(source) = err.source() {
        error!(" - Source: {}", source);
        err = source;
    }
}
```

You might ask, "But what if you actually do need to downcast the cause?" Really, I think it's best to destructure the cause into your own error kinds or what have you in the first place. Remove the reliance on internal implementations for the calling code.

The causing error will get wrapped up in a Box. Since by that point, there is very little point in trying to recover from that specific error... there isn't really a lot of value (imho) in keeping the type. Then for things like say... Diesel, when converting to my error type I have the error automatically get taken apart and split into a relevent kinds.  For example... to differentiate between a `NotFound` and a generic unrecoverable `Database` error.

Global context can be included in the error struct itself, and specific context can be provided in the kinds.

Interesting side note... I've found that those big Error enums that wrap up all the causes can introduce a notable increase in compile time vs sticking the cause on the heap like above.