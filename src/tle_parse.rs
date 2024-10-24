use std::fs::File;
use sgp4::Elements;
use std::io::{BufRead, BufReader};

pub fn tle_parse(filename: &str) -> Result<Vec<Elements>, Box<dyn std::error::Error>> {
    let file = File::open(filename).unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut elements_vec = Vec::new();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let line1 = line1?;
        let line2 = line2?;
        let line3 = line3?;

        let elements = Elements::from_tle(Some(line1), line2.as_bytes(), line3.as_bytes())?;
        elements_vec.push(elements);
    }

    Ok(elements_vec)
}
