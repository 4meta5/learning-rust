/// Category Theory For Programmers
///
/// (1) define a generic higher order function that caches called values locally 
/// -- takes a pure function `f` and returns the same function except it's memoized (caches called values locally)
/// (2) memoize a random number generator

fn memoize<A, B, F>(f: F) -> impl Fn(A) -> B
where 
    F: Fn(A) -> B
{
    // this is where everyone else got stuck as well
}

/// Resources and References
/// (1) https://codereview.stackexchange.com/questions/204555/recursive-fibonacci-in-rust-with-memoization
/// (2) https://github.com/jaemk/cached
/// (3) https://github.com/tylerreisinger/cache-macro

/// TODO: understand `let val = (||$body)();` in context
///
/// Code snippet from (2)/src/macro.rs
#[macro_export]
macro_rules! cached {
    // Use default cached::Cache
    ($cachename:ident;
     fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        cached!(
            $cachename : $crate::UnboundCache<($($argtype),*), $ret> = $crate::UnboundCache::new();
            fn $name($($arg : $argtype),*) -> $ret = $body
        );
    };

    // Use a specified cache-type and an explicitly created cache-instance
    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy {
                __cell: $crate::once_cell::sync::OnceCell::INIT,
                __init: || {
                    ::std::sync::Mutex::new($cacheinstance)
                },
        };

        #[allow(unused_parens)]
        pub fn $name($($arg: $argtype),*) -> $ret {
            let key = ($($arg.clone()),*);
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some(res) = res { return res.clone(); }
            }
            let val = (||$body)();
            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, val.clone());
            val
        }
    };
}