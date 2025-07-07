mod mutations;

use std::fs;

use crate::mutations::mutations::bitflip;

fn fuzz() {
    let data = fs::read("/Users/sahilpatki/woot/lucid/data/canon_40D.jpg").unwrap();
    let mut mutation_data = data.clone();

    // mutate the data
    bitflip(&mut mutation_data);

    // give it to the target program (for now just printing the muts)
    for i in 0..data.len() {
        if data[i] != mutation_data[i] {
            println!(
                "Difference found at: {}: {} {} ",
                i, data[i], mutation_data[i]
            );
        }
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
