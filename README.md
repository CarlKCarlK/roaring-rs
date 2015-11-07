# RoaringBitmap [![travis-badge][]][travis] [![release-badge][]][cargo] [![docs-badge][]][docs] [![license-badge][]][license]

> This is not yet production ready. The API should be mostly complete now.

This is a [Rust][] port of the [Roaring bitmap][] data structure, initially
defined as a [Java library][roaring-java] and described in [_Better bitmap
performance with Roaring bitmaps_][roaring-paper].

## Developing

Take note of the [Collections reform RFC][collections-rfc] for the API.  Mostly aiming to
duplicate the [BitSet][] API.

[travis-badge]: https://img.shields.io/travis/Nemo157/roaring-rs/master.svg?style=flat-square
[travis]: https://travis-ci.org/Nemo157/roaring-rs
[release-badge]: https://img.shields.io/github/release/Nemo157/roaring-rs.svg?style=flat-square
[cargo]: https://crates.io/crates/roaring
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: https://nemo157.com/roaring-rs/
[license-badge]: https://img.shields.io/badge/license-MIT-lightgray.svg?style=flat-square
[license]: https://github.com/Nemo157/roaring-rs/blob/master/LICENSE
[Rust]: https://rust-lang.org
[Roaring bitmap]: http://roaringbitmap.org
[roaring-java]: https://github.com/lemire/RoaringBitmap
[roaring-paper]: http://arxiv.org/pdf/1402.6407v4
[collections-rfc]: https://github.com/rust-lang/rfcs/pull/235
[BitSet]: http://doc.rust-lang.org/std/collections/struct.BitSet.html

## Benchmarking

The benchmarking for this project is integrated into the Java library's
benchmarking using JNA. First you must build the bindings lib to allow JNA to
access the rust code, then ensure you have the RoaringBitmap submodule and
simply run the benchmarks in `RoaringBitmap/jmh`.

For more info on the benchmarks see the documentation in
`RoaringBitmap/jmh/README.md`.

```sh
(cd bindings; cargo build --release)
git submodule update --init
./RoaringBitmap/jmh/run.sh
```
