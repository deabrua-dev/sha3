mod keccak;
use keccak::*;

pub struct SHA3_224 {
    rate: u64,
    capacity: u64,
    delimiter: u8,
}

impl SHA3_224 {
    pub fn new() -> Self {
        SHA3_224 {
            rate: 1152,
            capacity: 448,
            delimiter: 0x06,
        }
    }

    pub fn hash(&self, input: &[u8], output: &mut [u8]) {
        keccak(input, self.rate, self.capacity, self.delimiter, output)
    }
}

pub struct SHA3_256 {
    rate: u64,
    capacity: u64,
    delimiter: u8,
}

impl SHA3_256 {
    pub fn new() -> Self {
        SHA3_256 {
            rate: 1088,
            capacity: 512,
            delimiter: 0x06,
        }
    }
    
    pub fn hash(&self, input: &[u8], output: &mut [u8]) {
        keccak(input, self.rate, self.capacity, self.delimiter, output)
    }
}

pub struct SHA3_384 {
    rate: u64,
    capacity: u64,
    delimiter: u8,
}

impl SHA3_384 {
    pub fn new() -> Self {
        SHA3_384 {
            rate: 832,
            capacity: 768,
            delimiter: 0x06,
        }
    }
    
    pub fn hash(&self, input: &[u8], output: &mut [u8]) {
        keccak(input, self.rate, self.capacity, self.delimiter, output)
    }
}

pub struct SHA3_512 {
    rate: u64,
    capacity: u64,
    delimiter: u8,
}

impl SHA3_512 {
    pub fn new() -> Self {
        SHA3_512 {
            rate: 576,
            capacity: 1024,
            delimiter: 0x06,
        }
    }
    
    pub fn hash(&self, input: &[u8], output: &mut [u8]) {
        keccak(input, self.rate, self.capacity, self.delimiter, output)
    }
}

pub struct Keccak224 {
    rate: u64,
    capacity: u64,
    delimiter: u8,
}

impl Keccak224 {
    pub fn new() -> Self {
        Keccak224 {
            rate: 1152,
            capacity: 448,
            delimiter: 0x01,
        }
    }
    
    pub fn hash(&self, input: &[u8], output: &mut [u8]) {
        keccak(input, self.rate, self.capacity, self.delimiter, output)
    }
}

pub struct Keccak256 {
    rate: u64,
    capacity: u64,
    delimiter: u8,
}

impl Keccak256 {
    pub fn new() -> Self {
        Keccak256 {
            rate: 1088,
            capacity: 512,
            delimiter: 0x01,
        }
    }
    
    pub fn hash(&self, input: &[u8], output: &mut [u8]) {
        keccak(input, self.rate, self.capacity, self.delimiter, output)
    }
}

pub struct Keccak384 {
    rate: u64,
    capacity: u64,
    delimiter: u8,
}

impl Keccak384 {
    pub fn new() -> Self {
        Keccak384 {
            rate: 832,
            capacity: 768,
            delimiter: 0x01,
        }
    }
    
    pub fn hash(&self, input: &[u8], output: &mut [u8]) {
        keccak(input, self.rate, self.capacity, self.delimiter, output)
    }
}

pub struct Keccak512 {
    rate: u64,
    capacity: u64,
    delimiter: u8,
}

impl Keccak512 {
    pub fn new() -> Self {
        Keccak512 {
            rate: 576,
            capacity: 1024,
            delimiter: 0x01,
        }
    }
    
    pub fn hash(&self, input: &[u8], output: &mut [u8]) {
        keccak(input, self.rate, self.capacity, self.delimiter, output)
    }
}