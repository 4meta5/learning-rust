/// implementation of asynchronous threshold-based SSSS
extern crate futures;
extern crate rand;

mod fields;
mod numberthry;

use numberthry::*;
use rand;
use std::fmt;
use futures::{stream, Future, IntoFuture, Stream, Poll, Async};

// develop an asynchronous ssss scheme
// using honeybadgerbft

