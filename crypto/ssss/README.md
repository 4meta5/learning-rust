# Shamir's Secret Sharing Scheme

We can split a secret into `n` parts -- called secret shares -- and distribute them to different places/people. Later, we only need to collect `k` (`k > 0` and `k <= n`) secret shares to recover the original secret (`k` is the threshold).

There are a few implementations, some of which I may plan to explore in this directory:
* Linear Algebra Approach (*most likely to be implemented*)
    * implementation (./threshold) borrows from [snipco/rust-threshold-secret-sharing](https://github.com/snipsco/rust-threshold-secret-sharing/blob/master/src/lib.rs)
* Chinese Remainder Theorem 
    * [Asmuth-Bloom Scheme](https://en.wikipedia.org/wiki/Secret_sharing_using_the_Chinese_remainder_theorem#Asmuth-Bloom.27s_threshold_secret_sharing_scheme)
* * [Blakely's Scheme](https://en.wikipedia.org/wiki/Secret_sharing#Blakley.27s_scheme)

>  [More notes](https://github.com/AmarRSingh/notes/blob/master/Cryptography/SSSS.md)

## Currently Working On...
* Linear Algebra Approach in `./threshold`
    * want to add a consensus mechanism on top of it for performing SSS over a distributed set of nodes
    * use futures for this purpose

### References
* [Survey Paper](https://www.cs.bgu.ac.il/~beimel/Papers/Survey.pdf)