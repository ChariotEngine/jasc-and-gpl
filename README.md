# `jasc_and_gpl`

 A binary tool used to convert between JASC ("pal") and GIMP ("gpl") palette formats.

## License

MIT

## Usage

`jasc_and_gpl` operates in one of two modes:

* `jasc-to-gpl` which converts PAL to GPL
* `gpl-to-jasc` which converts GPL to PAL

```sh
$ cargo run -- jasc-to-gpl --jasc-path /path/to/input.pal --gpl-path /path/to/output.gpl

$ cargo run -- gpl-to-jasc --gpl-path /path/to/input.gpl --jasc-path /path/to/output.pal
```