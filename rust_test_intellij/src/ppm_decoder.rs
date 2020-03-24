// Decodes from P6 format
pub mod ppm_decoder {
    use std::path::Path;
    use std::io::{BufReader, Read, BufRead};
    use std::fs::File;
    use std::string::String;
    use crate::ppm_encoder::ppm_encoder::PPM;
    const HEADER_SIZE: u8 = 4;

    pub fn read_image(filename: &str) -> (Option<PPM>, String) {
        let path = Path::new(filename);
        let f = match File::open(path)  {
            Err(_) => return (None, String::from("Error while opening a file")),
            Ok(f) => f
        };
        let mut br = BufReader::new(f);

        // Read Header
        let mut hdr_str= String::new();
        br.read_line(&mut hdr_str)
            .expect("Error while reading a file");


        let hdr_str_splitted = hdr_str.split_whitespace().collect::<Vec<&str>>();

        if hdr_str_splitted[0] != "P6" {
            return (None, String::from("Is not P6 formatted"))
        }
        if hdr_str_splitted.len() < HEADER_SIZE as usize {
            return (None, String::from("Invalid Header, missing values"))
        }
        let width: u32 = match hdr_str_splitted[1].trim().parse()  {
            Err(_) => return (None, String::from("Invalid header, width value")),
            Ok(i) => i
        };
        let height: u32 = match hdr_str_splitted[2].trim().parse()  {
            Err(_) => return (None, String::from("Invalid header, height value")),
            Ok(i) => i
        };;
        let bits_pp: u8 = match hdr_str_splitted[3].trim().parse() {
            Err(_) => return (None, String::from("Invalid header, bits_pp value")),
            Ok(i) => i
        };
        let mut d: Vec<u8> = Vec::new();

        br.read_to_end(&mut d).expect("error");
        //println!("{}", d[0]);

        (Some(PPM{width: width, height: height, bits_pp: bits_pp, data: d}), String::from("Ok"))
    }
}