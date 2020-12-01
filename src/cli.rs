use hex_rgb::*;
use copypasta::{ClipboardContext, ClipboardProvider};
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

    #[structopt(short, long)]
    /// Enable copy to clipboard
    copy: bool,
}

fn copy_to_clipboard(contents: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(contents.to_owned()).unwrap();
}

pub fn init() {
    let opt = Opt::from_args();
    match convert_hexcode_to_rgb(opt.hex_code) {
        Ok(color) => {
            // TODO: Add option -i, --inverse to convert from rgb to hex
            let rgb = format!("rgb({}, {}, {})", color.red, color.green, color.blue);
            let output = format!(
                "{} {}",
                SQUARE.truecolor(color.red, color.green, color.blue),
                rgb.truecolor(color.red, color.green, color.blue)
                    .underline()
            );

            println!("{}", output);

            if opt.copy {
                copy_to_clipboard(&rgb);
                println!("{}", "copied to your clipboard.".dimmed())
            }
        }
        Err(e) => {
            eprintln!("{} {}", " ERROR ".bold().on_bright_red().bright_white(), e.bright_red());
            process::exit(1);
        }
    }
}
