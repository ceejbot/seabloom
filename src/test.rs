#![cfg(test)]

pub use super::Seabloom;

#[test]
fn new_size()
{
	// constructs a filter of the requested size
	let mut seeds: Vec<u64> = Vec::new();
	seeds.push(1);
	seeds.push(2);
	seeds.push(3);
	seeds.push(4);

	let bloom = Seabloom::new(32, seeds);
	assert_eq!(bloom.bitcount, 32);
	assert_eq!(bloom.seeds.len(), 4);
	assert_eq!(bloom.bitfield.len(), 1);
}

#[test]
fn test_create_with_hash_count()
{
	// constructs a filter with 4x the requested hash seed count
	let bloom = Seabloom::create_random_seeds(32, 3);
	assert_eq!(bloom.seeds.len(), 12);
}

#[test]
fn new_zeros()
{
	// zeros out its storage buffer
	let seeds: Vec<u64> = Vec::new();
	let bloom = Seabloom::new(128, seeds);

	for i in 0..bloom.bitfield.len()
	{
		assert_eq!(bloom.bitfield[i], 0);
	}
}

#[test]
fn create_defaults()
{
	let filter = Seabloom::create(95);
	assert_eq!(filter.bitcount, 1048);
	assert_eq!(filter.seeds.len(), 32);

	let f1 = Seabloom::create(148);
	assert_eq!(f1.bitcount, 1632);
	assert_eq!(f1.seeds.len(), 32);

	let f2 = Seabloom::create(10);
	assert_eq!(f2.bitcount, 110);
	assert_eq!(f2.seeds.len(), 32);
}

#[test]
fn create_optimal_error_rate()
{
	let filter = Seabloom::create(20000);
	assert_eq!(filter.bitcount, 220555);

	let f2 = Seabloom::create_optimal(20000, 0.2);
	assert_eq!(f2.bitcount < filter.bitcount, true);
}

#[test]
fn setbit_getbit()
{
	let mut filter = Seabloom::create_random_seeds(32, 4);

	filter.setbit(0);
	assert_eq!(filter.getbit(0), true);

	assert_eq!(filter.getbit(1), false);
	filter.setbit(1);
	assert_eq!(filter.getbit(1), true);

	assert_eq!(filter.getbit(2), false);

	assert_eq!(filter.getbit(17), false);
	filter.setbit(17);
	assert_eq!(filter.getbit(17), true);
}

#[test]
fn set_all_bits()
{
	let mut filter = Seabloom::create_random_seeds(64, 3);
	assert_eq!(filter.bitfield.len(), 2);

	for i in 0..64 { filter.setbit(i); }

	assert_eq!(filter.bitfield[0], u32::max_value());
	assert_eq!(filter.bitfield[1], u32::max_value());
}

#[test]
fn set_slice_shift()
{
	let mut filter = Seabloom::create_random_seeds(128, 3);

	filter.setbit(32);
	assert_eq!(filter.bitfield[1], 1);

	filter.setbit(65);
	assert_eq!(filter.bitfield[2], 2);
}

#[test]
fn add()
{
	let bytes = "cat".as_bytes();
	let mut filter = Seabloom::create_random_seeds(128, 3);
	assert_eq!(filter.has(bytes), false);
	filter.add(bytes);
	assert_eq!(filter.has(bytes), true);
}

#[test]
fn add_str()
{
	let mut filter = Seabloom::create_random_seeds(128, 3);
	assert_eq!(filter.has_str("cat"), false);
	filter.add_str("cat");
	assert_eq!(filter.has_str("cat"), true);
}

#[test]
fn add_list()
{
	let mut filter = Seabloom::create_random_seeds(128, 3);
	let vec = vec!["cat", "dog", "wallaby"];
	filter.add_list(vec);

	assert_eq!(filter.has_str("cat"), true);
	assert_eq!(filter.has_str("dog"), true);
	assert_eq!(filter.has_str("wallaby"), true);
	assert_eq!(filter.has_str("orange"), false);
}

#[test]
fn clear()
{
	let mut filter = Seabloom::create_random_seeds(128, 3);
	let vec = vec!["cat", "dog", "wallaby"];
	filter.add_list(vec);
	assert_eq!(filter.has_str("cat"), true);
	filter.clear();
	assert_eq!(filter.has_str("cat"), false);
}
