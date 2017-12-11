extern crate serde_xml_rs;
extern crate sogram;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::fs::File;
use std::io::Read;

use sogram::RsaPersist;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Arguments {
    #[structopt(help = "Input .sogram file")]
    file: String,
}

fn main() {
    let arguments = Arguments::from_args();
    let mut file =
        File::open(arguments.file).expect("Failed to open input file");

    let mut sogram = String::new();
    file.read_to_string(&mut sogram).expect("Failed to read input file");

    let sogram: RsaPersist =
        serde_xml_rs::from_str(&sogram).expect("Invalid input file");
    let spectrogram = sogram.data_points();

    for spectrogram_line in spectrogram {
        for amplitude in spectrogram_line {
            print!("{}\t", amplitude);
        }

        println!();
    }
}
