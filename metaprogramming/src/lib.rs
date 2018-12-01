
pub mod simple;

pub mod recurrence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn hello_test() {
        assert_eq!(println!("hello"), hello!());
    }

    #[test] // pretty dumb but so is the macro itself
    fn vec_str_test() {
        assert_eq!(vec!["a", "b", "c"], vec_strs!("a", "b", "c"));
    }
}