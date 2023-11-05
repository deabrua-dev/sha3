pub enum SHA3 {
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    KECCAK_224,
    KECCAK_256,
    KECCAK_384,
    KECCAK_512
}

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

const RATE_ARRAY: [u64; 4] = [ 1152, 1088, 832, 576 ];
const CAPACITY_ARRAY: [u64; 4] = [ 448, 512, 768, 1024 ];

const ROUNDS: usize = 24;

fn round_b(A: &mut [[u64; 5]; 5]) {
    for i in 0..ROUNDS {
        let mut C = [0; 5];
        let mut D = [0; 5];
        let mut B = [[0; 5]; 5];

        // Theta Step
        for x in 0..5 {
            C[x] = A[x][0] ^ A[x][1] ^ A[x][2] ^ A[x][3] ^ A[x][4];
        }
        for x in 0..5 {
            D[x] = C[(x + 4) % 5] ^ C[(x + 1) % 5].rotate_left(1);
        }
        for x in 0..5 {
            for y in 0..5 {
                A[x][y] ^= D[x];
            }
        }
        // Rho and Pi Steps
        for x in 0..5 {
            for y in 0..5 {
                B[y][(2 * x + 3 * y) % 5] = A[x][y].rotate_left(ROTATION_OFFSET[x][y]);
            }
        }
        // Chi Step
        for x in 0..5 {
            for y in 0..5 {
                A[x][y] = B[x][y] ^ ((!B[(x + 1) % 5][y]) & B[(x + 2) % 5][y])
            }
        }
        // Iota step
        A[0][0] ^= RND_C[i];
    }
}

fn keccak_f1600(state: &mut [u8; 200]) {
    let mut lanes: [[u64; 5]; 5]  = [[0; 5]; 5];
    for x in 0..5 {
        let mut temp: Vec<u64> = vec![];
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

pub fn keccak(input_bytes: &[u8], length: SHA3) -> Result<Vec<u8>, String> {
    let input_bytes_len = input_bytes.len();

    let rcd = match length {
        SHA3::SHA3_224 => (RATE_ARRAY[0], CAPACITY_ARRAY[0], 0x06),
        SHA3::SHA3_256 => (RATE_ARRAY[1], CAPACITY_ARRAY[1], 0x06),
        SHA3::SHA3_384 => (RATE_ARRAY[2], CAPACITY_ARRAY[2], 0x06),
        SHA3::SHA3_512 => (RATE_ARRAY[3], CAPACITY_ARRAY[3], 0x06),
        SHA3::KECCAK_224 => (RATE_ARRAY[0], CAPACITY_ARRAY[0], 0x01),
        SHA3::KECCAK_256 => (RATE_ARRAY[1], CAPACITY_ARRAY[1], 0x01),
        SHA3::KECCAK_384 => (RATE_ARRAY[2], CAPACITY_ARRAY[2], 0x01),
        SHA3::KECCAK_512 => (RATE_ARRAY[3], CAPACITY_ARRAY[3], 0x01),
    };
    let rate = rcd.0;
    let capacity = rcd.1;
    let delimiter = rcd.2;

    if rate + capacity != 1600 || rate % 8 != 0 {
        return Err(String::from("Error"));
    }
    let rates_in_bytes = (rate / 8) as usize;

    let mut output_bytes_len = (capacity / 16) as usize;

    let mut output_bytes = vec![];
    let mut state = [0; 200];
    
    let mut input_offset = 0;
    let mut block_size = 0;

    while input_offset < input_bytes_len {
        block_size = std::cmp::min(input_bytes_len - input_offset, rates_in_bytes);
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
        block_size = std::cmp::min(output_bytes_len, rates_in_bytes as usize);
        output_bytes.append(&mut state[0..block_size].to_vec());
        output_bytes_len -= block_size;
        if output_bytes_len > 0 {
            keccak_f1600(&mut state);
        }
    }
    Ok(output_bytes)
}