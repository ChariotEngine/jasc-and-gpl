use std::fs::File;
use std::io::BufReader;
use std::path::Path;

extern crate chariot_palette as jasc_palette;
extern crate gimp_palette;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("jasc-and-gpl")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Convert JASC palettes (\"pal\") to GIMP (\"gpl\") palettes")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("jasc-to-gpl")
            .arg(Arg::with_name("jasc-path")
                .long("jasc-path")
                .value_name("jasc-path")
                .help("The filepath to the jasc palette to convert to gpl")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("gpl-path")
                .long("gpl-path")
                .help("The filepath to write the output gpl palette to")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("gpl-to-jasc")
            .arg(Arg::with_name("jasc-path")
                .long("jasc-path")
                .value_name("jasc-path")
                .help("The filepath to write the output jasc palette to")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("gpl-path")
                .long("gpl-path")
                .help("The filepath to the gpl palette to convert to jasc")
                .required(true)
                .takes_value(true)))
        .get_matches();

        match matches.subcommand() {
            ("jasc-to-gpl", Some(args)) => jasc_to_gpl(args.value_of("jasc-path").unwrap(), args.value_of("gpl-path").unwrap()),
            ("gpl-to-jasc", Some(args)) => jasc_to_gpl(args.value_of("gpl-path").unwrap(), args.value_of("jasc-path").unwrap()),
            _ => unreachable!(),
        }
}

fn jasc_to_gpl<P: AsRef<Path> + std::fmt::Display>(jasc_path: P, gpl_path: P) {
    let jasc_colors = {
        let jasc_file = File::open(&jasc_path).expect(&format!("Failed to open {}", jasc_path));
        let mut jasc_reader = BufReader::new(jasc_file);
        jasc_palette::read_from(&mut jasc_reader).expect(&format!("Failed to load a JASC palette from {}", jasc_path))
    };

    let gpl_colors = jasc_colors.iter().map(|j| gimp_palette::Color { r: j.r, g: j.g, b: j.b }).collect::<Vec<_>>();
    let gpl_pal = match gimp_palette::Palette::new("Unnamed", gpl_colors) {
        Ok(pal) => pal,
        Err(_) => panic!("Failed to create a GIMP palette in-memory"),
    };

    gpl_pal.write_to_file(&gpl_path).expect(&format!("Failed to write GIMP palette to {}", gpl_path));
}

fn gpl_to_jasc<P: AsRef<Path> + std::fmt::Display>(gpl_path: P, jasc_path: P) {
    let gpl_pal = match gimp_palette::Palette::read_from_file(&gpl_path) {
        Ok(p) => p,
        Err(_) => panic!("Failed to read GIMP palette from {}", gpl_path)
    };

    println!("{} colors in the in-memory GIMP palette", gpl_pal.get_colors().len());
}