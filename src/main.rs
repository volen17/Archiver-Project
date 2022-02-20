mod lz77;
use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let com = &args[1];
    let file = &args[2];
    let flag_comp: bool;

    match com.as_str() {
        "compress" => flag_comp = true,
        "decompress" => flag_comp = false,
        _ => panic!("Wrong command!"),
    }

    let md = fs::metadata(file).unwrap();

    if md.is_file() {
        if flag_comp {
            println!("Compressing...\n");
            lz77::compress(file, file, false)
        } else {
            println!("Decompressing...\n");
            lz77::decompress(file, file, false)
        };
    } else {
        let paths = fs::read_dir(file).unwrap();
        if flag_comp {
            println!("Compressing your files...\n");
            for path in paths {
                let dir = &(file.to_owned() + "-compressed");
                fs::create_dir(Path::new(dir)).ok();
                lz77::compress(path.unwrap().path().to_str().unwrap(), dir, true)
            }
        } else {
            println!("Decompressing your files...\n");
            for path in paths {
                let dir = &(file.to_owned() + "-decompressed");
                fs::create_dir(Path::new(dir)).ok();
                lz77::decompress(path.unwrap().path().to_str().unwrap(), dir, true)
            }
        };
    }

    println!("\nYou are done!\n");
}
