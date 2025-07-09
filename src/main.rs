mod mutations;

use crate::mutations::mutations::{bitflip, gynvaels_magic_bytes};
use rand::Rng;
use std::fs;
use std::os::unix::process::ExitStatusExt;
use std::process::Command;

fn write_to_disk(data: Vec<u8>) -> String {
    let mut rng = rand::rng();
    let file_name = format!(
        "/Users/sahilpatki/woot/lucid/out/canon_40D_mutated_{}.jpg",
        rng.random_range(0..10000)
    );
    let _ = fs::write(file_name.clone(), data);

    return file_name;
}

fn read_file(abs_path: &str) -> Vec<u8> {
    let data = fs::read(abs_path).unwrap();
    return data;
}

fn mutate(data: &mut Vec<u8>) {
    // mutate the data
    bitflip(data);
    gynvaels_magic_bytes(data);

    // write_to_disk(mutation_data);
    // write_to_disk(mutation_data_2);
}

fn run(file_name: String) {
    let output = Command::new("/Users/sahilpatki/woot/lucid/test_target/exif")
        .arg(file_name.clone())
        .output()
        .expect("Failed to execute program!");

    let status = output.status;

    if let Some(signal) = status.signal() {
        if signal == 11 {
            println!("CRASH FOUND: {}", file_name.clone());
            let data = fs::read(file_name.clone()).unwrap();
            let split_file_name: Vec<&str> = file_name.split('/').collect();
            let just_file_name = split_file_name[split_file_name.len() - 1];
            fs::write(
                format!("/Users/sahilpatki/woot/lucid/crashes/{}", just_file_name),
                data,
            )
            .unwrap();
        }
    }
}

fn fuzz() {
    loop {
        // read file
        let mut data = read_file("/Users/sahilpatki/woot/lucid/data/canon_40D.jpg");

        // mutate
        mutate(&mut data);

        // write to the a file
        let out_name = write_to_disk(data);

        // feed it into the target program
        run(out_name);
    }
}

fn main() {
    /*
     * Need to do some things like provide path for input corups
     * and dirs for out for crashes. Right now keeping it simple and directly
     * calling fuzz
     */

    fuzz();
}
