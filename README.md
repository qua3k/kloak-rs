# kloak-rs: alternative implementation of kloak

kloak-rs is an alternative Rust implementation of [kloak](https://github.com/vmonaco/kloak).

It features many security improvements over the C codebase, including:
* Written in Rust, eliminating entire classes of vulnerabilities.
* Makes use of libraries such as `evdev` to avoid having to access `uinput` directly.
* Delays are securely generated with a ChaCha8 CSPRNG seeded by the kernel with `getrandom(2)`.

## eventcap

The `eventcap` tool is used to identify the device file corresponding to the keyboard device, and is typically `/dev/input/event*`.

### Usage

```
Usage:
    eventcap [DEVICE]

DEVICE is the device file that eventcap reads from.

Example:
    $ eventcap /dev/input/event4
    Reading from /dev/input/event4 (AT Translated Set 2 keyboard)
    Type: MISC    Code: 4    Value: 15
    Type: KEY     Code: 15   Value: 0
```
