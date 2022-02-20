use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file(path: &str) -> Vec<u8> {
    let mut file = match File::open(Path::new(path)) {
        Err(error) => panic!("Could not open {}: {}", path, error),
        Ok(file) => file,
    };
    let mut content = Vec::new();
    return match file.read_to_end(&mut content) {
        Err(error) => panic!("Could not read {}: {}", path, error),
        Ok(_) => content,
    };
}

fn write_file(content: &[u8], path: &str) {
    let mut file = match File::create(Path::new(path)) {
        Err(error) => panic!("Could not open {}: {}", path, error),
        Ok(file) => file,
    };
    match file.write_all(content) {
        Err(error) => panic!("Could not write {}: {}", path, error),
        Ok(ok) => ok,
    };
}

fn find_longest_match(data: &[u8], pos: usize) -> (u8, u8) {
    let mut best_offset = 0u8;
    let mut best_len = 0u8;
    let start = if pos > 255 { pos - 255 } else { 0 };

    for offset in start..pos {
        let len = matcher(data, offset, pos);
        if len > best_len {
            best_offset = (pos - (offset as usize)) as u8;
            best_len = len;
        }
    }
    return (best_offset, best_len);
}

fn matcher(data: &[u8], offset: usize, end: usize) -> u8 {
    let mut offset = offset;
    let mut pos = end;
    let mut len = 0u8;

    while offset < pos && pos < data.len() && data[offset] == data[pos] && len < 255 {
        offset += 1;
        pos += 1;
        len += 1;
    }
    return len;
}

pub fn compress(path: &str, dir: &str, is_directory: bool) {
    let file_name = Path::new(path)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    let data = read_file(&path);
    let mut output = Vec::new();
    let mut outfile = dir.to_owned();
    if is_directory {
        outfile.push_str("\\");
        outfile.push_str(&file_name)
    }
    outfile.push_str(".compressed");
    let mut pos = 0;
    while pos < data.len() {
        let (offset, len) = find_longest_match(&data, pos);
        output.push(offset);
        if offset == 0 {
            output.push(data[pos]);
            pos += 1;
        } else {
            output.push(len);
            pos = pos + (len as usize);
        }
    }

    write_file(&output, &outfile);

    println!("{} successfully compressed in {}", path.to_owned(), outfile);
}

pub fn decompress(path: &str, dir: &str, is_directory: bool) {
    let file_name = Path::new(path)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    let data = read_file(&path);
    let mut output = Vec::new();
    let mut outfile = dir.to_owned();
    if is_directory {
        outfile.push_str("\\");
        outfile.push_str(&file_name[..file_name.len() - 11]);
    } else {
        outfile = (&outfile[..outfile.len() - 11]).to_owned();
    }
    let mut pos = 0;
    while pos + 1 < data.len() {
        let header = data[pos];
        let item = data[pos + 1];
        pos += 2;
        if header == 0 {
            output.push(item);
        } else {
            let offset = header as usize;
            let len = item as usize;
            let start = output.len() - offset;
            for i in start..(start + len) {
                let c = output[i];
                output.push(c);
            }
        }
    }
    write_file(&output, &outfile);

    println!(
        "{} successfully decompressed in {}",
        path.to_owned(),
        outfile
    );
}
