use std::{fs::File, io::{self, BufReader}};
use std::io::{BufRead, Seek, SeekFrom};


pub fn watch_file(file: &str) -> Result<(), io::Error> {
    let f = match File::open(file) {
        Ok(x) => x,
        Err(err) => return Err(err),
    };

    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut pos: u64;

    // Go to the end of the file
    pos = reader.seek(SeekFrom::End(0)).unwrap();

    // Start watching for new lines
    loop {
        let resp = reader.read_line(&mut line);

        match resp {
            Ok(len) => {
                if len > 0 {
                    pos += len as u64;
                    reader.seek(SeekFrom::Start(pos)).unwrap();
                    print!("{}", line);
                    line.clear();
                }
            },
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}