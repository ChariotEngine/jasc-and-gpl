use std::fs::File;
use std::io::BufReader;

extern crate chariot_palette as jasc_palette;
extern crate gimp_palette;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("jasc-to-gpl")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Convert JASC palettes (\"pal\") to GIMP (\"gpl\") palettes")
        .arg(Arg::with_name("jasc-path")
            .long("jasc-path")
            .value_name("jasc-path")
            .help("The filepath to the 'pal' to convert to 'gpl'")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("gpl-path")
            .long("gpl-path")
            .help("The filepath to write the output 'gpl' to")
            .required(true)
            .takes_value(true))
        .get_matches();

    let jasc_colors = {
        let jasc_path = matches.value_of("jasc-path").unwrap();
        let jasc_file = File::open(&jasc_path).expect(&format!("Could not open {}", jasc_path));
        let mut jasc_reader = BufReader::new(jasc_file);
        jasc_palette::read_from(&mut jasc_reader).expect(&format!("Failed to load a JASC palette from {}", jasc_path))
    };

    let gpl_path = matches.value_of("gpl-path").unwrap();
    let gpl_colors = jasc_colors.iter().map(|j| gimp_palette::Color { r: j.r, g: j.g, b: j.b }).collect::<Vec<_>>();
    let gpl_pal = gimp_palette::Palette::new("Unnamed", gpl_colors).expect("Failed to create a GIMP palette in-memory");
    gpl_pal.write_to_file(&gpl_path).expect(&format!("Failed to write GIMP palette to {}", gpl_path));
}
