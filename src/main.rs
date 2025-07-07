mod mutations;

use mutations::*;
use std::fs;
use std::io;

use crate::mutations::mutations::bitflip;

fn fuzz() {
    let mut data = fs::read("/home/sahilpatki/woot/lucid/data/canon_40D.jpg").unwrap();

    // mutate the data
    bitflip(&mut data);

    // give it to the target program
}

fn main() {
    /*
     * Need to do some things like provide path for input corups
     * and dirs for out for crashes. Right now keeping it simple and directly
     * calling fuzz
     */

    fuzz();
}
