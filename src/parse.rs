use std::fs::File;
use std::io::{Read, self, Error, ErrorKind};
use std::path::Path;

pub struct BmpImage {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>, // All binary
}

pub fn parse_bmp_file(path_str: String) -> io::Result<BmpImage> {
    let mut file = File::open(Path::new(&path_str))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Min 54 Byte
    if buffer.len() < 54 {
        return Err(Error::new(ErrorKind::InvalidData, "Too small file!"));
    }

    // First two bytes its B and M
    if buffer[0] != b'B' || buffer[1] != b'M' {
        return Err(Error::new(ErrorKind::InvalidData, "This is not a BM file!"));
    }

    let width = i32::from_le_bytes([buffer[18], buffer[19], buffer[20], buffer[21]]);
    let height = i32::from_le_bytes([buffer[22], buffer[23], buffer[24], buffer[25]]);

    Ok(BmpImage {
        width,
        height,
        data: buffer,
    })
}