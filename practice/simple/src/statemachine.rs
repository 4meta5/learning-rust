/// Rust State Machine Patterns 10/12/2016 by Hoverbear
/// [link](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/)
/// For creating type-safe `Future`s from state machines 
/// [fitzgen/state_machine_future](https://github.com/fitzgen/state_machine_future)

// Bottle Filling Example
// very simple -- just for practice
// OVER TIME, I'll CHANGE THIS SO IT'S BETTER
trait SharedFunctionality {
    fn get_shared_value(&self) -> usize;
}

struct Waiting {
    waiting_time: std::time::Duration,
    // value shared by all states
    shared_value: usize,
}

impl Waiting {
    fn new() -> Self {
        Waiting {
            waiting_time: std::time::Duration::new(0,0),
            shared_value: 0,
        }
    }

    // Consumes the value!
    fn to_filling(self) -> Filling {
        Filling {
            rate: 1,
            shared_value: 0,
        }
    }
}

impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

struct Filling {
    rate: usize,
    shared_value: usize,
}

impl SharedFunctionality for Filling {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

struct Done {
    shared_value: usize,
}

impl SharedFunctionality for Done {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

enum State {
    Waiting(Waiting),
    Filling(Filling),
    Done(Done),
}

// We can use `From` (and tacitly `Into` as well)
impl From<Waiting> for Filling {
    fn from(val: Waiting) -> Filling {
        Filling {
            rate: 1,
            shared_value: val.shared_value,
        }
    }
}