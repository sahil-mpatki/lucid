mod mutations;

use crate::mutations::mutations::{bitflip, gynvaels_magic_bytes};
use rand::Rng;
use std::fs;
use std::os::unix::process::ExitStatusExt;
use std::process::Command;

fn write_to_disk(data: Vec<u8>) {
    let mut rng = rand::rng();
    let _ = fs::write(
        format!(
            "/Users/sahilpatki/woot/lucid/out/canon_40D_mutated_{}.jpg",
            rng.random_range(0..10000)
        ),
        data,
    );
}

fn mutate() {
    let data = fs::read("/Users/sahilpatki/woot/lucid/data/canon_40D.jpg").unwrap();
    let mut mutation_data = data.clone();
    // let mut mutation_data_2 = data.clone();

    // mutate the data
    bitflip(&mut mutation_data);
    gynvaels_magic_bytes(&mut mutation_data);

    write_to_disk(mutation_data);
    // write_to_disk(mutation_data_2);
}

fn run() {
    let files = fs::read_dir("/Users/sahilpatki/woot/lucid/out/").unwrap();
    for mut file in files {
        println!(
            "{}",
            file.as_mut().unwrap().file_name().into_string().unwrap()
        );
        let output = Command::new("/Users/sahilpatki/woot/lucid/test_target/exif")
            .arg(
                "/Users/sahilpatki/woot/lucid/out/".to_string()
                    + file
                        .as_mut()
                        .unwrap()
                        .file_name()
                        .into_string()
                        .unwrap()
                        .as_str(),
            )
            .output()
            .expect("Failed to execute program!");

        let status = output.status;
        println!("{:?}", String::from_utf8_lossy(&output.stdout));

        if let Some(signal) = status.signal() {
            if signal == 11 {
                println!(
                    "CRASH FOUND: {}",
                    file.as_mut()
                        .unwrap()
                        .file_name()
                        .into_string()
                        .unwrap()
                        .as_str()
                );
            }
        }
    }
}

fn fuzz() {
    // mutate
    mutate();

    // feed it into the target program
    run();
}

fn main() {
    /*
     * Need to do some things like provide path for input corups
     * and dirs for out for crashes. Right now keeping it simple and directly
     * calling fuzz
     */

    fuzz();
}
