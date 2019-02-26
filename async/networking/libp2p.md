# Libp2p Notes
> * [Rust implementation](https://github.com/libp2p/rust-libp2p) of [libp2p networking stack](https://libp2p.io)

* [Documentation for libp2p transport and protocol upgrade system](https://github.com/tomaka/libp2p-rs/blob/7aacb081d2e7db05b17c932370c926bb5e0d6230/libp2p-swarm/README.md)

## Why Libp2p
> [Why Libp2p](https://www.parity.io/why-libp2p/)

A fundamental shift in distributed computing is that the “client/server” paradigm no longer holds up. Let’s take a look at what your home router does. Every device in your home network has a private IP address. When you request data from a server, your router replaces your device’s private address with your home’s public IP address, and remembers which device to send the response to.

That works fine if all your devices are clients, but what about when a request from the outside world shows up at your router? It’s not a response to a request, it is a request, so the requestor thinks that you are a server. One of your devices is acting as a server, but your router doesn’t know which one. This is a problem called **NAT traversal**, and libp2p provides tools to help handle it.

### Main Libp2p Protocols
* secio, which is responsible for encrypting communications.
* mplex or yamux, which are protocols on top of secio that are responsible for multiplexing.