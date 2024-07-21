use clap::{value_parser, Arg, Command};
use nannou::{
    image::{imageops, open, GenericImageView, GrayImage, RgbImage},
    prelude::*,
};

enum ImageType {
    Gray(GrayImage),
    Rgb(RgbImage),
}

struct Model {
    fp: String,
    img: ImageType,
    steps: usize,
    chars: Option<Vec<char>>,
}

const TITLE: &'static str = "ASCII Art Creator";

fn main() {
    nannou::app(model).view(view).run();
}

fn model(app: &App) -> Model {
    let args = Command::new(TITLE)
    .version("1.0")
    .about("A program that creates ASCII art in Rust using Nannou. You can even watch it be made in real time!")
    .after_help("The width/height arguments are mutually exclusive. This is because the aspect ratio of the image is preserved, so the height is dependent on the width and vice versa.")
    .author("shroom00")
    .args([
        Arg::new("input").short('i').value_name("FILE").help("The path to the input image.").required(true),
        Arg::new("output").short('o').value_name("FILE").help("The path where the ASCII art will be saved.").default_value("ascii_art.png"),
        Arg::new("steps").short('s').value_name("NUMBER").help("How many letters to place.").value_parser(value_parser!(usize)).default_value("500"),
        Arg::new("width").long("width").value_name("NUMBER").help("The width of the output image.").value_parser(value_parser!(u32)).conflicts_with("height"),
        Arg::new("height").long("height").value_name("NUMBER").help("The height of the output image.").value_parser(value_parser!(u32)).default_value("1000"),
        Arg::new("characters").long("chars").value_name("CHARACTERS").help("The characters you want to be used in output image. (Defaults to visible ASCII characters)"),
        Arg::new("colour").long("colour").help("Uses accurate colours when specified, instead of grayscale.").action(clap::ArgAction::SetTrue)
        ]
    ).get_matches();

    let input: &str = args.get_one::<String>("input").unwrap();
    let fp: String = args.get_one::<String>("output").unwrap().to_string();
    let steps: usize = *args.get_one("steps").unwrap();
    let chars: Option<Vec<char>> = match args.get_one::<String>("characters") {
        Some(_string) => Some(_string.chars().collect()),
        None => None,
    };

    app.set_loop_mode(LoopMode::loop_ntimes(steps + 2));

    let (width, height): (u32, u32) = match args.get_one::<u32>("width") {
        Some(width) => (*width, u32::MAX),
        None => (u32::MAX, *args.get_one::<u32>("height").unwrap()),
    };

    let (width, height, img) = {
        let img = open(input)
            .unwrap()
            .resize(width, height, imageops::FilterType::CatmullRom);
        let (width, height) = img.dimensions();
        let img = match args.get_flag("colour") {
            true => ImageType::Rgb(img.to_rgb8()),
            false => ImageType::Gray(img.to_luma8()),
        };
        (width, height, img)
    };

    app.new_window()
        .size_pixels(width, height)
        .view(view)
        .resizable(false)
        .title(TITLE)
        .build()
        .unwrap();

    Model {
        fp,
        img,
        steps,
        chars,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if frame.nth() < model.steps as u64 {
        let (w, h) = app.main_window().inner_size_pixels();
        let w = w as i64;
        let h = h as i64;
        let (x, y) = (random_range(-w / 2, w / 2), random_range(-h / 2, h / 2));
        let draw = app.draw();

        let colour = {
            let (x, y) = ((x + (w / 2)) as u32, (y + (h / 2)) as u32);
            match &model.img {
                ImageType::Gray(img) => {
                    let lightness = img.get_pixel(x, y).0[0];
                    gray(lightness)
                }
                ImageType::Rgb(img) => {
                    let p = img.get_pixel(x, y).0;
                    srgb(p[0], p[1], p[2])
                },
            }
        };

        let random_char = match &model.chars {
            Some(chars) => chars[random_range(0, chars.len())],
            None => random_ascii(),
        };

        draw.text(&random_char.to_string())
            .x_y(x as f32, -y as f32)
            .color(colour)
            .font_size(random_range(10, 20))
            .center_justify();
        draw.to_frame(app, &frame).unwrap();
    } else if frame.nth() == model.steps as u64 {
        app.main_window().capture_frame(&model.fp);
    } else if frame.nth() == model.steps as u64 + 1 {
        println!("The design is complete and has been saved to {}, you may now close the window. (or exit from the terminal with Ctrl-C)", model.fp);
        app.main_window().set_title(&format!("{TITLE} - Finished!"));
    }
}
