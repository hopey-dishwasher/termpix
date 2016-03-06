extern crate docopt;
extern crate image;
extern crate rustc_serialize;
extern crate terminal_size;
extern crate termpix;

use std::io::Write;

use docopt::Docopt;
use image::GenericImage;
use terminal_size::{Width, Height, terminal_size};

use std::path::Path;
use std::cmp::min;

const USAGE: &'static str = "
    termpix : display image from <file> in an ANSI terminal

    Usage:
      termpix <file> [--width <width>] [--height <height>] [--max-width <max-width>] [--max-height <max-height>] [--true-color|--true-colour]

      By default it will use as much of the current terminal window as possible, while maintaining the aspect 
      ratio of the input image. This can be overridden as follows.

    Options:
      --width <width>    Output width in terminal columns.
      --height <height>  Output height in terminal rows.
      --max-width <max-width>  Maximum width to use when --width is excluded
      --max-height <max-height>  Maximum height to use when --height is excluded
      --true-colour             Use 24-bit RGB colour. Some terminals don't support this.
      --true-colour             Use 24-bit RGB color but you don't spell so good.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_width: Option<u32>,
    flag_height: Option<u32>,
    flag_max_width: Option<u32>,
    flag_max_height: Option<u32>,
    flag_true_colour: bool,
    flag_true_color: bool,
    arg_file: String,
}

fn main() {

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let img = image::open(&Path::new(&args.arg_file)).unwrap();
    let (orig_width, orig_height) = img.dimensions();
    let true_colour = args.flag_true_colour || args.flag_true_color;
    let (width, height) = determine_size(args, orig_width, orig_height);

    termpix::print_image(img, true_colour, width, height);

}

fn determine_size(args: Args,
                  orig_width: u32,
                  orig_height: u32) -> (u32, u32) {
    match (args.flag_width, args.flag_height) {
        (Some(w), Some(h)) => (w, h * 2),
        (Some(w), None) => (w, scale_dimension(w, orig_height, orig_width)),
        (None, Some(h)) => (scale_dimension(h * 2, orig_width, orig_height), h * 2),
        (None, None) => {
            let size = terminal_size();

            if let Some((Width(terminal_width), Height(terminal_height))) = size {                            
                fit_to_size(
                    orig_width, 
                    orig_height, 
                    terminal_width as u32, 
                    (terminal_height - 1) as u32,
                    args.flag_max_width,
                    args.flag_max_height)
            } else {
                writeln!(std::io::stderr(), "Neither --width or --height specified, and could not determine terminal size. Giving up.").unwrap();
                std::process::exit(1);
            }
        }
    }
}

pub fn fit_to_size(orig_width: u32, 
                   orig_height: u32, 
                   terminal_width: u32, 
                   terminal_height: u32,
                   max_width: Option<u32>,
                   max_height: Option<u32>) -> (u32, u32) {
    let target_width = match max_width {
        Some(max_width) => min(max_width, terminal_width),
        None => terminal_width
    };

    //2 pixels per terminal row
    let target_height = 2 * match max_height {
        Some(max_height) => min(max_height, terminal_height),
        None => terminal_height
    };

    let calculated_width = scale_dimension(target_height, orig_width, orig_height);
    if calculated_width <= target_width {
        (calculated_width, target_height)
    } else {
        (target_width, scale_dimension(target_width, orig_height, orig_width))
    }
}
