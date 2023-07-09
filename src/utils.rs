use std::str::FromStr;

use crate::cli::Args;

// stolen from https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html
#[derive(Debug, PartialEq)]
struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let r: u8 = u8::from_str_radix(&hex_code[0..2], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[2..4], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[4..6], 16)?;

        Ok(RGB { r, g, b })
    }
}

pub fn args_to_css(args: &Args) -> String {
    let window_bg = RGB::from_str(&args.window_background_color)
        .expect("invalid color for window_background_color");
    let label_bg = RGB::from_str(&args.label_background_color)
        .expect("invalid color for label_background_color");

    format!(
        r#"
        window {{
            background: rgba({}, {}, {}, {});
        }}

        window label {{
            background: rgba({}, {}, {}, {});
            font-family: {};
            font-weight: {};
            padding: {}px {}px;
        }}
        "#,
        window_bg.r,
        window_bg.g,
        window_bg.b,
        args.window_background_opacity,
        label_bg.r,
        label_bg.g,
        label_bg.b,
        args.label_background_opacity,
        args.font_family,
        args.font_weight,
        args.label_padding_y,
        args.label_padding_x,
    )
}