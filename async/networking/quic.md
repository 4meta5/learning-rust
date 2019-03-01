# QUIC Protocol
> protocol by Google sitting between `UDP` and `HTTP` which seems to minimize stream congestion (in the context of limited bandwidth) by decreasing handshake latency and invoking session tickets to instantaneously initiate encrypted connections thereafter.

* [QUINN](https://github.com/djc/quinn) -- futures-based QUIC implementation by `djc` and `Ralith`
* [Quiche](https://github.com/cloudflare/quiche) -- QUIC transport protocol and HTTP/3 by `cloudflare`