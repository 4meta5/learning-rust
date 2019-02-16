# Cryptography

* [Data Structures](./primitives)
* [Algebra](./algebra)

* [Shamir's Secret Sharing Scheme](./erasure/ssss)
    * for conceptual background on information theory, see [notes metalink](https://github.com/AmarRSingh/notes/tree/master/Cryptography/InformationTheory)

## Stable Cryptography in Rust

* [Rust RNG crate](https://github.com/rust-random/rand)
* [`DaGenix/rust-crypto`](https://github.com/DaGenix/rust-crypto) --- cryptographic algorithms implemented in Rust
* [`brycx/orion`](https://github.com/brycx/orion) -- easy and usable rust crypto
* [Blockchain Crypto MPC in `C`](https://github.com/unbound-tech/blockchain-crypto-mpc) -- FFI
* [Verifiable Delay Function Implementation by POANetwork](https://github.com/poanetwork/vdf)

### Blockchain <a name = "blockchain"></a>

* [`matterinc/plasma_winter`](https://github.com/matterinc/plasma_winter) -- an account model Plasma with with zkSNARK proved state transitions
* [`matterinc/plasma_cash_history_snark`](https://github.com/matterinc/plasma_cash_history_snark) -- history compaction zkSNARK circuit for Plasma Cash and Cashflow history compaction

* [`nearprotocol/nearcore`](https://github.com/nearprotocol/nearcore)
* [`mimblewimble/grin`](https://github.com/mimblewimble/grin)
* [`nervosnetwork/ckb`](https://github.com/nervosnetwork/ckb)
* [`cryptape/cita`](https://github.com/cryptape/cita)
* [CKB's VM](https://github.com/nervosnetwork/ckb-vm)
* [Exonum: private/permissioned blockchain framework](https://github.com/exonum/exonum)
* [`holochain-rust`](https://github.com/holochain/holochain-rust)
* [`cryptape/bft-rs`](https://github.com/cryptape/bft-rs)
* [`nwtnni/paxos`](https://github.com/nwtnni/paxos)

* [BOLT: Blind Off-chain Lightweight Transactions](https://github.com/ZcashFoundation/libbolt)

#### Substrate
> [Parity Samples](https://github.com/parity-samples)

* [Substrate Documentation](https://substrate.readme.io/docs)
* **[Substrate Runtime Module Library](https://github.com/paritytech/substrate/tree/master/srml)**
* [Substrate runtime recipes](https://substrate.readme.io/docs/substrate-runtime-recipes)
* [Substrate Collectables Workshop](https://github.com/shawntabrizi/substrate-collectables-workshop)
* [Substrate Proof of Existence](https://github.com/shawntabrizi/substrate-proof-of-existence)
* [Substrate TCR](https://github.com/gautamdhameja/substrate-tcr)
* [Substrate Events Listener](https://github.com/gautamdhameja/substrate-events-listener)
* [srml example use w/ comments](https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs)
* [AdEx Notes](https://hackmd.io/p_v1M8WGRyy9PggYiKA_Xw#)

### Zero Knowledge

* **[Demo how to make SNARKs for Edcon](https://github.com/matterinc/Edcon2019_material)**

* [bellman-examples](https://github.com/arcalinea/bellman-examples)
* [`republicprotocol/zksnark-rs`](https://github.com/republicprotocol/zksnark-rs) -- groth16 impl
* [`ebfull/powersoftau`](https://github.com/ebfull/powersoftau) -- communal zk-SNARK MPC for Public Parameters
* [Rust-language assets for Zcash](https://github.com/zcash/librustzcash)
* [Implementation of the Jubjub elliptic curve group](https://github.com/zkcrypto/jubjub)
* [ZkVM: A Blockchain VM with Cloaked Assets and ZK Smart Contracts](https://github.com/interstellar/slingshot)
* [Sonic - quickly verifiable, compact zk proofs of arbitrary computations](https://github.com/zknuckles/sonic)
    * [MPC implementation for a Structured Reference String (SRS) for SONIC proofs](https://github.com/matterinc/alpha_line)
* [Zokrates](https://github.com/Zokrates/ZoKrates) -- toolbox for zkSNARKS on Ethereum
* dizk, zexe
* bulletproofs
* [Filecoin Proving Subsystem (FPS)](https://github.com/filecoin-project/rust-proofs)

* [roll_up](https://github.com/barryWhiteHat/roll_up) in Rust would be *cool*

## Misc Resources

* [Auditing Rust Crypto the First Hours](https://research.kudelskisecurity.com/2019/02/07/auditing-rust-crypto-the-first-hours/) by Kudelski Security

* [`baidu/rust-sgx-sdk`](https://github.com/baidu/rust-sgx-sdk) -- for writing Intel SGX applications in Rust