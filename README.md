# swaystatus

This repository holds a [Rust][] rewrite of my shell script which is used to
populate **swaybar** (part of [sway][]) with system information.

## Implementation

- information sections should be represented as 'structs'
- each section can have different update intervals
- sections should be able to update without blocking other things
- one central place which handles all 'structs' and publishes the data
- 'syscalls' should be reduced to a minimum
- errors should be handled

## License

The project is licensed under the MIT license. See [LICENSE](LICENSE) for more
information.

[Rust]: https://www.rust-lang.org/
[sway]: https://github.com/swaywm/sway
