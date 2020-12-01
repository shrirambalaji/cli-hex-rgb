use cli_hex_rgb::*;
use owo_colors::OwoColorize;
use std::process;
use structopt::StructOpt;
const SQUARE: &str = "■";

#[derive(StructOpt, Debug)]
#[structopt(name = "cli-hex-rgb", about = "A cli to convert hex to rgb")]
struct Opt {
    #[structopt(short, long)]
    /// Activate debug mode (-d, --debug)
    debug: bool,

    hex_code: String,
}

pub fn init() {
    let opt = Opt::from_args();
    match convert_hexcode_to_rgb(opt.hex_code) {
        Ok(color) => {
            // TODO: Highlight result with specified color, or show a box with filled with color
            // TODO: Add option -c, --copy to copy rgb value to clipboard
            // TODO: Add option -i, --inverse to convert from rgb to hex
            let rgb = format!("rgb({}, {}, {})", color.red, color.green, color.blue);
            let output = format!(
                "{} {}",
                SQUARE.truecolor(color.red, color.green, color.blue),
                rgb.truecolor(color.red, color.green, color.blue)
                    .underline()
            );

            println!("{}", output);

        Err(e) => {
            eprintln!("{} {}", " ERROR ".bold().on_bright_red().bright_white(), e.bright_red());
            process::exit(1);
        }
    }
}
