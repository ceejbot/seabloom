# seabloom

Yet Another Bloom Filter, this one using [seahash](https://github.com/ticki/tfs/tree/master/seahash).

[![Build Status](https://travis-ci.org/ceejbot/seabloom.svg?branch=master)](https://travis-ci.org/ceejbot/seabloom) [![cargo](https://crates.io/crates/seabloom)(https://img.shields.io/crates/v/seabloom.svg)

## Usage

`Seabloom::create(item_count: u32) -> Seabloom`

Create a Bloom filter sized for the given item count with an error rate of 0.5% (0.005). Seeds for the hashing functions will be generated randomly for you.

`Seabloom::create_optimal(item_count: u32, error_rate: f32) -> Seabloom`

Create a Bloom filter sized for the given item count with the specified error rate.  Seeds for the hashing functions will be generated randomly for you.

`Seabloom::create_random_seeds(bitcount: u64, hashcount: u32) -> Seabloom`

Create a Bloom filter with the given number of bits for storage and the given number of hashing functions. Seeds for the hashing functions will be generated randomly for you. You probably don't want to use this function; use `create` instead.

`Seabloom::new(bitcount: u64, seeds: Vec<u64>) -> Seabloom`

Create a Bloom filter with the given number of bits for storage and hashing functions using the seeds you provide.

`filter.clear()`

Clear the filter.

`filter.add(bytes: &[u8])`

Add an item to the filter.

`filter.add_str(item: &str)`

Add a string to the filter.

`filter.add_list(items: Vec<&str>)`

Add a list of strings to the filter.

`filter.has(bytes: &[u8]) -> bool`

Check to see if the given pile-o-bytes is in the filter. Provides a definitive no or a maybe-yes.

`filter.has_str(item: &str) -> bool`

Check to see if the given string is in the filter. Provides a definitive no or a maybe-yes.

## License

ISC.
