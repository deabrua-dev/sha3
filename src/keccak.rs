use std::cmp::min;

const ROTATION_OFFSET: [[u32; 5]; 5] = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
];

const RND_C: [u64; 24] = [
    0x0000000000000001, 
    0x0000000000008082, 
    0x800000000000808A,
    0x8000000080008000, 
    0x000000000000808B, 
    0x0000000080000001,
    0x8000000080008081, 
    0x8000000000008009, 
    0x000000000000008A,
    0x0000000000000088, 
    0x0000000080008009, 
    0x000000008000000A,
    0x000000008000808B, 
    0x800000000000008B, 
    0x8000000000008089,
    0x8000000000008003, 
    0x8000000000008002, 
    0x8000000000000080,
    0x000000000000800A, 
    0x800000008000000A, 
    0x8000000080008081,
    0x8000000000008080, 
    0x0000000080000001, 
    0x8000000080008008
];

const ROUNDS: usize = 24;

fn round_b(lanes: &mut [[u64; 5]; 5]) {
    for i in 0..ROUNDS {
        let mut c = [0; 5];
        let mut d = [0; 5];
        let mut b = [[0; 5]; 5];

        // Theta Step
        for x in 0..5 {
            for y in 0..5 {
                c[x] ^= lanes[x][y];
            }
        }
        for x in 0..5 {
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        }
        for x in 0..5 {
            for y in 0..5 {
                lanes[x][y] ^= d[x];
            }
        }
        // Rho and Pi Steps
        for x in 0..5 {
            for y in 0..5 {
                b[y][(2 * x + 3 * y) % 5] = lanes[x][y].rotate_left(ROTATION_OFFSET[x][y]);
            }
        }
        // Chi Step
        for x in 0..5 {
            for y in 0..5 {
                lanes[x][y] = b[x][y] ^ ((!b[(x + 1) % 5][y]) & b[(x + 2) % 5][y])
            }
        }
        // Iota step
        lanes[0][0] ^= RND_C[i];
    }
}

fn keccak_f1600(state: &mut [u8; 200]) {
    let mut lanes: [[u64; 5]; 5]  = [[0; 5]; 5];
    for x in 0..5 {
        for y in 0..5 {
            lanes[x][y] = u64::from_le_bytes(state[(8 * (x + 5 * y))..(8 * (x + 5 * y) + 8)].try_into().unwrap())
        }
    }
    round_b(&mut lanes);
    for x in 0..5 {
        for y in 0..5 {
            let temp = lanes[x][y].to_le_bytes();
            for (i, &t) in temp.iter().enumerate() {
                state[8 * (x + 5 * y) + i] = t;
            }
        }
    }
}

pub fn keccak(input_bytes: &[u8], rate: u64, capacity: u64, delimiter: u8, output: &mut [u8]) {
    let input_bytes_len = input_bytes.len();

    if rate + capacity != 1600 || rate % 8 != 0 {
        return;
    }
    let rates_in_bytes = (rate / 8) as usize;

    let mut state = [0; 200];
    
    let mut input_offset = 0;
    let mut block_size = 0;

    let mut output_bytes_len = output.len();
    let mut start_index = 0;

    while input_offset < input_bytes_len {
        block_size = min(input_bytes_len - input_offset, rates_in_bytes);
        for i in 0..block_size {
            state[i] ^= input_bytes[i + input_offset];
        }
        input_offset += block_size;
        if block_size == rates_in_bytes {
            keccak_f1600(&mut state);
        }
    }
    state[block_size] ^= delimiter;

    if (delimiter & 0x80) != 0 && (block_size == rates_in_bytes - 1) {
        keccak_f1600(&mut state);
    }
    state[rates_in_bytes - 1] ^= 0x80;
    keccak_f1600(&mut state);

    while output_bytes_len > 0 {
        block_size = min(output_bytes_len, rates_in_bytes as usize);
        output[start_index..(start_index + block_size)].copy_from_slice(state[0..block_size].as_mut());
        output_bytes_len -= block_size;
        start_index += block_size;
        if output_bytes_len > 0 {
            keccak_f1600(&mut state);
        }
    }
}