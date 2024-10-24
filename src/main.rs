use orbital_calc::tle_parse;
use orbital_calc::render;
use std::env;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let elements = tle_parse::tle_parse(filename).unwrap();


    render::render(elements)?;

    Ok(())

}

