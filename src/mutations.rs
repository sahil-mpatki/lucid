pub mod mutations {
    use rand::Rng;
    const BITFLIP_FACTOR: u32 = 10;
    const GYNVAEL_MAGIC_NUMBERS: [(u32, u32); 10] = [
        (1, 0xFF),
        (1, 0x7F),
        (1, 0x00),
        (2, 0xFFFF),
        (2, 0x0000),
        (4, 0xFFFFFFFF),
        (4, 0x00000000),
        (4, 0x80000000),
        (4, 0x40000000),
        (4, 0x7FFFFFFF),
    ];

    pub fn bitflip(data: &mut Vec<u8>) {
        let num_of_flips: u32 = (((data.len() - 4) as u32) * BITFLIP_FACTOR) / 100;
        let mut rng = rand::rng();

        for _i in 0..num_of_flips {
            // exclude the first and last bytes which denote the start and end marker
            let index = rng.random_range(1..data.len() - 1);
            let bitnum = rng.random_range(0..8);

            data[index] = data[index] ^ (1 << bitnum);
        }
    }

    pub fn gynvaels_magic_bytes(data: &mut Vec<u8>) {
        let mut rng = rand::rng();

        // for now just do 10 random bytes
        for _i in 0..10 {
            let magic_num_index = rng.random_range(0..GYNVAEL_MAGIC_NUMBERS.len());
            let magic_num_len = GYNVAEL_MAGIC_NUMBERS[magic_num_index].0;
            let magic_num = GYNVAEL_MAGIC_NUMBERS[magic_num_index].1;

            match magic_num_len {
                1 => {
                    let random_data_index = rng.random_range(1..data.len() - 1);
                    data[random_data_index] = magic_num as u8;
                }
                2 => {
                    let random_data_index = rng.random_range(1..data.len() - 1 - 2);
                    data[random_data_index] = ((magic_num >> 8) & 0xff) as u8;
                    data[random_data_index + 1] = ((magic_num) & 0xff) as u8;
                }
                4 => {
                    let random_data_index = rng.random_range(1..data.len() - 1 - 4);
                    data[random_data_index] = ((magic_num >> 24) & 0xff) as u8;
                    data[random_data_index + 1] = ((magic_num >> 16) & 0xff) as u8;
                    data[random_data_index + 2] = ((magic_num >> 8) & 0xff) as u8;
                    data[random_data_index + 3] = ((magic_num) & 0xff) as u8;
                }
                _ => {}
            }
        }
    }
}
