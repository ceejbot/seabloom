extern crate rand;
extern crate seahash;

use rand::Rng;

const LN2: f32 = std::f32::consts::LN_2;
const LN2_SQUARED: f32 = LN2 * LN2;

mod test;

pub struct Seabloom
{
    seeds: Vec<u64>,
    bitcount: u64,
    bitfield: Vec<u32>,
}

impl Seabloom
{
    pub fn new(bitcount: u64, seeds: Vec<u64>) -> Seabloom
    {
        let buf: Vec<u32> = vec![0; (bitcount as f32 / 32.0_f32).ceil() as usize];

        Seabloom
        {
            seeds: seeds,
            bitcount: bitcount,
            bitfield: buf,
        }
    }

    pub fn create_random_seeds(bitcount: u64, hashcount: u32) -> Seabloom
    {
        let mut seeds: Vec<u64> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..hashcount * 4
        {
            seeds.push(rng.gen::<u64>());
        }

        Seabloom::new(bitcount as u64, seeds)
    }

    pub fn create_optimal(item_count: u32, error_rate: f32) -> Seabloom
    {
        let bitcount = (-1.0_f32 * (item_count as f32) * error_rate.ln() / LN2_SQUARED)
            .round();
        let hashcount = (bitcount / (item_count as f32) * LN2)
            .round() as u32;

        Seabloom::create_random_seeds(bitcount as u64, hashcount)
    }

    pub fn create(item_count: u32) -> Seabloom
    {
        Seabloom::create_optimal(item_count, 0.005_f32)
    }

    pub fn clear(&mut self)
    {
        let buf: Vec<u32> = vec![0; (self.bitcount as f32 / 8.0_f32).ceil() as usize];
        self.bitfield = buf;
    }

    fn setbit(&mut self, bit: u64)
    {
        let mut pos: usize = 0;
        let mut shift = bit;
    	while shift > 31
    	{
    		pos += 1;
    		shift -= 32;
    	}

        let mut chunk = self.bitfield[pos];
        chunk |= 0x1 << shift;
        self.bitfield[pos] = chunk;
    }

    fn getbit(&self, bit: u64) -> bool
    {
        let mut pos: usize = 0;
        let mut shift = bit;
    	while shift > 31
    	{
            pos += 1;
    		shift -= 32;
    	}

        let chunk = self.bitfield[pos];
    	(chunk & (0x1 << shift)) != 0
    }

    pub fn add_bytes(&mut self, bytes: &[u8])
    {
        let mut i: usize = 0;
        loop
        {
            let hash = seahash::hash_seeded(bytes, self.seeds[i], self.seeds[i+1], self.seeds[i+2], self.seeds[i+3]);
            let bit = hash % self.bitcount;
            self.setbit(bit);
            i += 4;
            if i >= self.seeds.len() { break; }
        }
    }

    pub fn add(&mut self, item: &str)
    {
        self.add_bytes(item.as_bytes());
    }

    pub fn add_list(&mut self, items: Vec<&str>)
    {
        for item in items.iter() { self.add(item); }
    }

    pub fn has_bytes(&self, bytes: &[u8]) -> bool
    {
        let mut i: usize = 0;
        loop
        {
            let hash = seahash::hash_seeded(bytes, self.seeds[i], self.seeds[i+1], self.seeds[i+2], self.seeds[i+3]);
            let bit = hash % self.bitcount;
            if !self.getbit(bit) { return false; }

            i += 4;
            if i >= self.seeds.len() { break; }
        }

        true
    }

    pub fn has(&self, item: &str) -> bool
    {
        self.has_bytes(item.as_bytes())
    }

}
