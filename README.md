# watch-clipboard

[![Build Status][travis-image]][travis-url]
[![License: MIT][license-image]][license-url]

Monitor clipboard for changes and print them to stdout.

## Usage

```
$ watch-clipboard --help
watch-clipboard 0.1.0
Monitor clipboard for changes and print them to stdout

USAGE:
    watch-clipboard [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -v               Enable logging, use multiple `v`s to increase verbosity
    -V, --version    Prints version information

OPTIONS:
    -d, --delay <delay>    Delay in milliseconds before polling clipboard for new content [default: 250]
```

## License

MIT

[travis-image]:  https://travis-ci.org/menski/watch-clipboard-rs.svg?branch=master
[travis-url]: https://travis-ci.org/menski/watch-clipboard-rs
[license-image]: https://img.shields.io/badge/License-MIT-yellow.svg
[license-url]: https://opensource.org/licenses/MIT
