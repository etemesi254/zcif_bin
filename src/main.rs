use clap::{arg, Command};
use std::{time::Instant};
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
        .arg(arg!(-r --runs <run> "Run the encoder/decoder <run> number of times").required(false
        ).default_value("20"))
        .get_matches();
    let runs = clp.value_of("runs").unwrap().parse::<usize>().unwrap_or(20);

    if clp.value_of("fse").is_some() {
        println!("Opening file: {:?}", clp.value_of("fse").unwrap());
        _fse(clp.value_of("fse").unwrap(),runs);
    } else if clp.value_of("huff").is_some() {
        println!("Opening file: {:?}", clp.value_of("huff").unwrap());
        _huff(clp.value_of("huff").unwrap(),runs);
    }
    // _fse();

    // _huff();
}

fn _huff(file: &str,runs:usize) {
    let writer = Vec::with_capacity(2);
    let mut writer = std::io::Cursor::new(writer);
    
    writer.set_position(0);

    let mut reader = std::io::Cursor::new(std::fs::read(file).unwrap());

    
    let mut new_writer = Vec::<u8>::with_capacity(85 * (1 << 20));


    println!("In memory buffer compression Huffman, {} runs", runs);
    for _ in 0..runs {
        writer.set_position(0);

        new_writer.clear();

        reader.set_position(0);

        let start_compress = Instant::now();
        huff_compress(&reader.get_ref(), &mut writer);
        let stop_compress = Instant::now();

        writer.set_position(0);

        let start_decomp = Instant::now();
        huff_decompress(&mut writer, &mut new_writer);
        let stop_decomp = Instant::now();


        print!(
            "\r {} bytes -> {} bytes ({:.2}%)  [{:.4} to 1] ",
            reader.get_ref().len(),
            writer.get_ref().len(),
            (writer.get_ref().len() as f64 / (reader.get_ref().len() as f64) * 100.0),
            reader.get_ref().len() as f64 / (writer.get_ref().len() as f64)
        );
        print!(
            " {:.2} MB/s",
            reader.get_ref().len() as f64
                / ((1 << 20) as f64 * (stop_compress - start_compress).as_secs_f64())
        );

        print!(
            " {:.2} MB/s ",
            new_writer.len() as f64 
                / ((1 << 20) as f64 * (stop_decomp - start_decomp).as_secs_f64())
        );
        
    }
    println!();

    // assert that reader matches writer
    assert!(reader.get_ref().eq(&new_writer), "Bad decoder");
}

fn _fse(file: &str,runs:usize) {
    let writer = Vec::with_capacity(2);
    let mut writer = std::io::Cursor::new(writer);
    writer.set_position(0);

    let mut reader = std::io::Cursor::new(std::fs::read(file).unwrap());

    let mut new_writer = Vec::<u8>::with_capacity(85 * (1 << 20));

    
    println!("In memory buffer compression FSE, {} runs", runs);
    for _ in 0..runs {
        writer.set_position(0);

        new_writer.clear();

        reader.set_position(0);

        let start_compress = Instant::now();
        fse_compress(&reader.get_ref(), &mut writer);
        let stop_compress = Instant::now();

        writer.set_position(0);

        let start_decomp = Instant::now();
        fse_decompress(&mut writer, &mut new_writer);
        let stop_decomp = Instant::now();


        print!(
            "\r {} bytes -> {} bytes ({:.2}%)  [{:.4} to 1] ",
            reader.get_ref().len(),
            writer.get_ref().len(),
            (writer.get_ref().len() as f64 / (reader.get_ref().len() as f64) * 100.0),
            reader.get_ref().len() as f64 / (writer.get_ref().len() as f64)
        );
        print!(
            "  {:.2} MB/s",
            reader.get_ref().len() as f64
                / ((1 << 20) as f64 * (stop_compress - start_compress).as_secs_f64())
        );

        print!(
            " {:.2} MB/s ",
            new_writer.len() as f64 
                / ((1 << 20) as f64 * (stop_decomp - start_decomp).as_secs_f64())
        );
        
    }
    println!();



    assert!(reader.get_ref().eq(&new_writer), "Bad decoder");
}
