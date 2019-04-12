use image::{DynamicImage, GenericImageView, Pixel, Rgba};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const UPPER_HALF_BLOCK: &str = "\u{2580}";
const LOWER_HALF_BLOCK: &str = "\u{2584}";

pub fn print(img: &DynamicImage) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let (width, _) = img.dimensions();

    let mut _curr_row_px = 0;
    let mut curr_col_px = 0;
    let mut buffer: Vec<ColorSpec> = Vec::with_capacity(width as usize);
    let mut mode: Status = Status::TopRow;
    for pixel in img.pixels() {
        if mode == Status::TopRow {
            let mut c = ColorSpec::new();
            let color = get_color(get_pixel_data(pixel));
            c.set_bg(Some(color));
            buffer.push(c);
            curr_col_px += 1;
        } else {
            let color = get_color(get_pixel_data(pixel));
            let spec_to_upg = &mut buffer[curr_col_px as usize];
            spec_to_upg.set_fg(Some(color));
            curr_col_px += 1;
        }
        //if the buffer is full start adding the second row of pixels
        if is_buffer_full(&buffer, width) {
            if mode == Status::TopRow {
                mode = Status::BottomRow;
                _curr_row_px += 1;
                curr_col_px = 0;
            }
            //only if the second row is completed flush the buffer and start again
            else if curr_col_px == width {
                curr_col_px = 0;
                _curr_row_px += 1;
                print_buffer(&mut buffer);
                mode = Status::TopRow;
            }
        }
    }

    //flush the buffer (will be invoked if the image has an odd height)
    if !buffer.is_empty() {
        flush_buffer(&mut buffer);
    }

    clear_printer(&mut stdout);
}

//TODO: print_buffer and flush_buffer are too identical
fn print_buffer(buff: &mut Vec<ColorSpec>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for c in buff.iter() {
        stdout
            .set_color(&c)
            .unwrap_or_else(|e| eprintln!("Error while changing terminal colors: {}", e));

        write!(&mut stdout, "{}", LOWER_HALF_BLOCK)
            .unwrap_or_else(|e| eprintln!("Error while displaying image: {}", e));
    }

    clear_printer(&mut stdout);
    write_newline(&mut stdout);
    buff.clear();
}
fn flush_buffer(buff: &mut Vec<ColorSpec>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for c in buff.iter() {
        let mut new_c = ColorSpec::new();
        let bg = c.bg().unwrap();
        new_c.set_fg(Some(*bg));

        stdout
            .set_color(&new_c)
            .unwrap_or_else(|e| eprintln!("Error while changing terminal colors: {}", e));

        write!(&mut stdout, "{}", UPPER_HALF_BLOCK)
            .unwrap_or_else(|e| eprintln!("Error while displaying image: {}", e));
    }

    clear_printer(&mut stdout);
    write_newline(&mut stdout);
    buff.clear();
}

fn write_newline(stdout: &mut StandardStream) {
    writeln!(stdout).unwrap_or_else(|e| eprintln!("Error while displaying image: {}", e));
}

fn get_color(p: Rgba<u8>) -> Color {
    Color::Rgb(p.data[0], p.data[1], p.data[2])
}

fn get_pixel_data<T: Pixel>(p: (u32, u32, T)) -> T {
    p.2
}

fn clear_printer(stdout: &mut StandardStream) {
    let c = ColorSpec::new();
    stdout
        .set_color(&c)
        .unwrap_or_else(|e| eprintln!("Error while changing terminal colors: {}", e));
}
#[derive(PartialEq)]
enum Status {
    TopRow,
    BottomRow,
}
fn is_buffer_full(buffer: &[ColorSpec], width: u32) -> bool {
    buffer.len() == width as usize
}

#[test]
fn test_buffer_full() {
    let buffer = vec![ColorSpec::new(), ColorSpec::new()];
    let width = 2;
    assert!(is_buffer_full(&buffer, &width));
}
#[test]
fn test_print_buffer() {
    let mut buffer = vec![ColorSpec::new(), ColorSpec::new()];
    print_buffer(&mut buffer);
    assert!(buffer.len() == 0);
}
#[test]
fn test_status_eq() {
    let s1 = Status::TopRow;
    let s2 = Status::BottomRow;
    assert!(s1 != s2);
}