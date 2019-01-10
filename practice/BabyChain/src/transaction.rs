// A transaction that include sender and receiver address
#[derive(Hash, Default, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub from: String,
}