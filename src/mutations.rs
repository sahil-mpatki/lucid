pub mod mutations {
    use rand::Rng;
    const BITFLIP_FACTOR: u32 = 10;
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
}
