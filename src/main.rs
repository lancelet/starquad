extern crate csv;
extern crate flate2;
extern crate serde;

use csv::{ReaderBuilder, Terminator, Trim};
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct GaiaRecord {
    solution_id: u64,
    designation: String,
    source_id: u64,
    random_index: u64,
}

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let test_file = File::open(
        "/Volumes/Gaia/GaiaSource/GaiaSource_99872562456678912_100222271578464384.csv.gz",
    )?;
    let gz_decoder = GzDecoder::new(&test_file);

    // debugging: dump out each byte from the file to check that gz decryption is working
    /*
    for byte in gz_decoder.bytes() {
        print!("{}", byte.unwrap() as char);
    }
    */

    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .flexible(false)
        .trim(Trim::All)
        .terminator(Terminator::CRLF)
        .quoting(false)
        .from_reader(gz_decoder);

    let records = csv_reader.deserialize::<GaiaRecord>();

    for record in records {
        println!("{:?}", record)
    }

    Ok(())
}
