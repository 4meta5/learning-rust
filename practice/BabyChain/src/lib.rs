// look at parity bitcoin and simplify the logic

mod block;
mod transaction;

trait Printer {
    fn print(&self, transaction: Vec<Transaction>) -> Self;
    // fn proof(&self, hash: u64) -> u64;
}

/// implement printer for Block