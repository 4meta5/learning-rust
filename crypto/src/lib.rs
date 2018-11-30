/// this will be interesting

extern crate blake2_rfc;
extern crate ring;
extern crate untrusted;

pub mod ed25519;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
