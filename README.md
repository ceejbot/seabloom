# seabloom

Yet Another Bloom Filter, this one using [seahash](https://github.com/ticki/tfs/tree/master/seahash).

[![Build Status](https://travis-ci.org/ceejbot/seabloom.svg?branch=master)](https://travis-ci.org/ceejbot/seabloom) [![cargo](https://crates.io/crates/seabloom)](https://img.shields.io/crates/v/seabloom.svg)

## Usage

```rust
extern crate seabloom;

let mut filter = Seabloom::create(2000);

assert_eq!(filter.has("cat"), false);
filter.add("cat");
assert_eq!(filter.has("cat"), true);

filter.add_list(vec!["cat", "jaguar", "lion", "tiger", "leopard"]);
assert_eq!(filter.has("caracal"), false);
assert_eq!(filter.has("jaguar"), true);

filter.clear();
assert_eq!(filter.has("cat"), false);
```

## API

`Seabloom::create(item_count: u32) -> Seabloom`

Create a Bloom filter sized for the given item count with an error rate of 0.5% (0.005). Seeds for the hashing functions will be generated randomly for you.

`Seabloom::create_optimal(item_count: u32, error_rate: f32) -> Seabloom`

Create a Bloom filter sized for the given item count with the specified error rate.  Seeds for the hashing functions will be generated randomly for you.

`Seabloom::create_random_seeds(bitcount: u64, hashcount: u32) -> Seabloom`

Create a Bloom filter with the given number of bits for storage and the given number of hashing functions. Seeds for the hashing functions will be generated randomly for you. You probably don't want to use this function; use `create` instead.

`Seabloom::new(bitcount: u64, seeds: Vec<u64>) -> Seabloom`

Create a Bloom filter with the given number of bits for storage and hashing functions using the seeds you provide. You need 4x the number of seeds as hashing functions for seahash's current API.

`filter.clear()`

Clear the filter.

`filter.add(item: &str)`

Add a string to the filter.

`filter.add_bytes(bytes: &[u8])`

Add an item represented by the given bytes to the filter.

`filter.add_list(items: Vec<&str>)`

Add a list of strings to the filter.

`filter.has(item: &str) -> bool`

Check to see if the given string is in the filter. Provides a definitive no or a maybe-yes.

`filter.has_bytes(bytes: &[u8]) -> bool`

Check to see if the given pile-o-bytes is in the filter. Provides a definitive no or a maybe-yes.

## License

ISC.
