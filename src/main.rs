use clap::{arg, Command};
use std::time::Instant;
use zcif::{fse_compress, fse_decompress, huff_compress, huff_decompress};

fn main() {
    let clp = Command::new("Zcif bin")
        .version("0.0")
        .about("Does awesome things")
        .arg(
            arg!(-f --fse <file> "Get FSE encoding and decoding speeds for <file> ")
                .required(false),
        )
        .arg(
            arg!(-z --huff <file> "Get Huffman encoding and decoding speeds for <file> ")
                .required(false),
        )
        .get_matches();
    if clp.value_of("fse").is_some() {
        println!("Opening file: {:?}", clp.value_of("fse").unwrap());
        _fse(clp.value_of("fse").unwrap());
    }
    else if clp.value_of("huff").is_some(){
        println!("Opening file: {:?}", clp.value_of("huff").unwrap());
        _fse(clp.value_of("huff").unwrap());
    }
    // _fse();

    // _huff();
}

fn _huff(file: &str) {
    const RUNS: usize = 20;
    let writer = Vec::with_capacity(2);
    let mut writer = std::io::Cursor::new(writer);
    writer.set_position(0);

    let mut reader = std::io::Cursor::new(std::fs::read(file).unwrap());

    let start = Instant::now();
    for _ in 0..RUNS {
        writer.set_position(0);
        reader.set_position(0);
        huff_compress(&reader.get_ref(), &mut writer);
    }
    let stop = Instant::now();

    println!("=====================================================");
    println!("In memory buffer compression Huffman(20 runs)");
    println!("Time elapsed : {} ms", (stop - start).as_millis());
    println!(
        "{} bytes -> {} bytes ({:.2}%)  ({:.4} to 1)",
        reader.get_ref().len(),
        writer.get_ref().len(),
        (writer.get_ref().len() as f64 / (reader.get_ref().len() as f64) * 100.0),
        reader.get_ref().len() as f64 / (writer.get_ref().len() as f64)
    );
    println!("-----------------------------------------------------");
    println!(
        "Speed :{} MB/s",
        reader.get_ref().len() as f64 * RUNS as f64
            / ((1 << 20) as f64 * (stop - start).as_secs_f64())
    );
    println!("-----------------------------------------------------");

    let new_timer = Instant::now();
    let mut new_writer = Vec::<u8>::with_capacity(85 * (1 << 20));
    for _ in 0..RUNS {
        unsafe {
            new_writer.set_len(0);
        }
        writer.set_position(0);
        huff_decompress(&mut writer, &mut new_writer);
    }
    let new_stop = Instant::now();
    println!("=====================================================");
    println!("In memory buffer decompression Huffman");
    println!("Time elapsed : {} ms", (new_stop - new_timer).as_millis());
    println!("-----------------------------------------------------");

    println!(
        "Speed :{} MB/s",
        new_writer.len() as f64 * RUNS as f64
            / ((1 << 20) as f64 * (new_stop - new_timer).as_secs_f64())
    );
    println!("-----------------------------------------------------");

    // assert that reader matches writer
    assert!(reader.get_ref().eq(&new_writer), "Bad decoder");
}

fn _fse(file: &str) {
    const RUNS: usize = 20;
    let writer = Vec::with_capacity(2);
    let mut writer = std::io::Cursor::new(writer);
    writer.set_position(0);

    let mut reader = std::io::Cursor::new(std::fs::read(file).unwrap());

    let start = Instant::now();
    for _ in 0..RUNS {
        writer.set_position(0);
        reader.set_position(0);
        fse_compress(&reader.get_ref(), &mut writer);
    }
    let stop = Instant::now();

    println!("=====================================================");
    println!("In memory buffer compression FSE(20 runs)");
    println!("Time elapsed : {} ms", (stop - start).as_millis());
    println!(
        "{} bytes -> {} bytes ({:.2}%) ({:.4} to 1)",
        reader.get_ref().len(),
        writer.get_ref().len(),
        (writer.get_ref().len() as f64 / (reader.get_ref().len() as f64) * 100.0),
        (reader.get_ref().len() as f64 / (writer.get_ref().len() as f64))
    );
    println!("-----------------------------------------------------");
    println!(
        "Speed :{} MB/s",
        reader.get_ref().len() as f64 * RUNS as f64
            / ((1 << 20) as f64 * (stop - start).as_secs_f64())
    );
    println!("-----------------------------------------------------");

    let new_timer = Instant::now();
    let mut new_writer = Vec::<u8>::with_capacity(85 * (1 << 20));
    for _ in 0..RUNS {
        unsafe {
            new_writer.set_len(0);
        }
        writer.set_position(0);
        fse_decompress(&mut writer, &mut new_writer);
    }
    let new_stop = Instant::now();
    println!("=====================================================");
    println!("In memory buffer decompression FSE");
    println!("Time elapsed : {} ms", (new_stop - new_timer).as_millis());
    println!("-----------------------------------------------------");

    println!(
        "Speed :{} MB/s",
        new_writer.len() as f64 * RUNS as f64
            / ((1 << 20) as f64 * (new_stop - new_timer).as_secs_f64())
    );
    println!("-----------------------------------------------------");

    // println!("{:?}\n{:?}",reader.get_ref(),&new_writer);
    assert!(reader.get_ref().eq(&new_writer), "Bad decoder");
}
