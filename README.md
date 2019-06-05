[![Build Status](https://travis-ci.com/critical-path/macaddress-rs.svg?branch=master)](https://travis-ci.com/critical-path/macaddress-rs)

## Introduction

Media access control (MAC) addresses play an important role in local-area networks.  They also pack a lot of information into 48-bit hexadecimal strings!

The macaddress library makes it easy to evaluate the properties of MAC addresses and the [extended identifiers](https://standards.ieee.org/products-services/regauth/tut/index.html) of which they are subclasses.


## Installing macaddress

macaddress is available on GitHub at https://github.com/critical-path/macaddress-rs.

To install it, run the following commands from your shell.

```console
[user@host ~]$ git clone git@github.com:critical-path/macaddress-rs.git
[user@host ~]$ cd macaddress-rs
[user@host macaddress-rs]$ cargo build
```

Alternatively, add it to the list of dependencies in `Cargo.toml`.

```vim
[dependencies]
macaddress = { git = "https://github.com/critical-path/macaddress-rs.git" }
```


## Using macaddress

Bring the `MediaAccessControlAddress` struct into scope in `main.rs`, `lib.rs`, or some other relevant file.

```rust
use macaddress::macaddress::MediaAccessControlAddress;
```

Instantiate `MediaAccessControlAddress` by calling the `new` method and passing in a MAC address in plain, hyphen, colon, or dot notation.

```rust
let digits = String::from("a0b1c2d3e4f5");
let mac = MediaAccessControlAddress::new(&digits).unwrap();
```

```rust
let digits = String::from("a0-b1-c2-d3-e4-f5");
let mac = MediaAccessControlAddress::new(&digits).unwrap();
```

```rust
let digits = String::from("a0:b1:c2:d3:e4:f5");
let mac = MediaAccessControlAddress::new(&digits).unwrap();
```

```rust
let digits = String::from("a0b1.c2d3.e4f5");
let mac = MediaAccessControlAddress::new(&digits).unwrap();
```

To determine whether the MAC address is a broadcast, a multicast (layer-two), or a unicast address, call the `is_broadcast`, `is_multicast`, and `is_unicast` methods.

```rust
let broadcast = mac.is_broadcast();
println!("{}", &broadcast);
```

```rust
let multicast = mac.is_multicast();
println!("{}", &multicast);
```

```rust
let unicast = mac.is_unicast();
println!("{}", &unicast);
```

To determine whether the MAC address is a universally-administered address (UAA) or a locally-administered address (LAA), call the `is_uaa` and `is_laa` methods.

```rust
let uaa = mac.is_uaa();
println!("{}", &uaa);
```

```rust
let laa = mac.is_laa();
println!("{}", &laa);
```

To determine whether the MAC address is an extended unique identifier (EUI), an extended local identifier (ELI), or unknown, call the `kind` method.

```rust
let kind = mac.kind();
println!("{}", &kind);
```

To determine whether the MAC address has an organizationally-unique identifier (OUI) or a company ID (CID), call the `has_oui` and `has_cid` methods.

```rust
let oui = mac.has_oui();
println!("{}", &oui);
```

```rust
let cid = mac.has_cid();
println!("{}", &cid);
```

To view the binary equivalent of the MAC address, call the `to_binary_representation` method.  The most-significant digit of each octet appears first.

```rust
let binary = mac.to_binary_representation();
println!("{}", &binary);
```

To return the MAC address's two "fragments," call the `to_fragments` method.  For an EUI, this means the 24-bit OUI as the first fragment and the remaining interface-specific bits as the second fragment.  For an ELI, this means the 24-bit CID as the first fragment and the remaining interface-specific bits as the second fragment.

```rust
let fragments = mac.to_fragments();
println!("{:?}", &fragments);
```

To return the MAC address in different notations, call the `to_plain_notation`, `to_hyphen_notation`, `to_colon_notation`, and `to_dot_notation` methods.

```rust
let plain = mac.to_plain_notation();
println!("{}", &plain);
```

```rust
let hyphen = mac.to_hyphen_notation();
println!("{}", &hyphen);
```

```rust
let colon = mac.to_colon_notation();
println!("{}", &colon);
```

```rust
let dot = mac.to_dot_notation();
println!("{}", &dot);
```


## Testing macaddress

To conduct testing, run the following commands from your shell.

```console
[user@host macaddress-rs]$ cargo test --lib
```


## Building the documentation

To build the documentation for macaddress, run the following commands from your shell.

```console
[user@host macaddress-rs]$ cargo doc
```
