# swaystatus

This repository holds a [Rust][] rewrite of my shell script which is used to
populate **swaybar** (part of [sway][]) with system information.

## Output format

- a spacer of two colons (' :: ') is used between the modules
- modules added to the Publisher are published from left to right

```sh
# from left to right:
# - available system updates (Arch Linux only)
# - load average (1/5/15) with CPU count in parenthesis
# - memory usage (used/total)
# - time
0 :: 1.78 1.53 1.52 (8) :: 1586 MB/15903 MB :: 11:47 AM
```

## Implementation Goals

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
