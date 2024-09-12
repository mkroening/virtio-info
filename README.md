# virtio-info

[![Crates.io](https://img.shields.io/crates/v/virtio-info)](https://crates.io/crates/virtio-info)
[![CI](https://github.com/mkroening/virtio-info/actions/workflows/ci.yml/badge.svg)](https://github.com/mkroening/virtio-info/actions/workflows/ci.yml)

This application prints informations about available virtio devices in Linux guests, such as the activated [feature bits].

[feature bits]: https://docs.oasis-open.org/virtio/virtio/v1.2/cs01/virtio-v1.2-cs01.html#x1-6600006

## Example

```console
$ virtio-info
/sys/bus/virtio/drivers/virtio_rng/virtio1 rng device active feature bits:
 28: INDIRECT_DESC
 29: EVENT_IDX
 32: VERSION_1

/sys/bus/virtio/drivers/virtio_net/virtio0 net device active feature bits:
  0: CSUM
  1: GUEST_CSUM
  2: CTRL_GUEST_OFFLOADS
  3: MTU
  5: MAC
  7: GUEST_TSO4
  8: GUEST_TSO6
  9: GUEST_ECN
 10: GUEST_UFO
 11: HOST_TSO4
 12: HOST_TSO6
 13: HOST_ECN
 14: HOST_UFO
 15: MRG_RXBUF
 16: STATUS
 17: CTRL_VQ
 18: CTRL_RX
 19: CTRL_VLAN
 21: GUEST_ANNOUNCE
 23: CTRL_MAC_ADDR
 28: INDIRECT_DESC
 29: EVENT_IDX
 32: VERSION_1
```

## Installation

This application can be installed via `cargo`:

```bash
cargo install virtio-info
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
