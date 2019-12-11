use std::fmt::Write;

fn solve(input: &str) -> Image {
    let width = 25;
    let height = 6;

    let image = input.trim().as_bytes();
    let layer_size = width * height;
    let num_layers = image.len() / layer_size;

    let pixels: Vec<_> = (0..layer_size)
        .map(|pixel| {
            (0..num_layers)
                .map(|layer| image[layer * layer_size + pixel])
                .find_map(|p| match p {
                    b'0' => Some(Pixel::Black),
                    b'1' => Some(Pixel::White),
                    b'2' => None,
                    _ => unreachable!(),
                })
                .expect("no non-transparent pixels found")
        })
        .collect();

    Image { width, pixels }
}

struct Image {
    width: usize,
    pixels: Vec<Pixel>,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pixels.chunks(self.width).try_for_each(|row| {
            row.iter().try_for_each(|p| write!(f, "{}", p))?;
            f.write_char('\n')
        })
    }
}

enum Pixel {
    Black,
    White,
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::Black => f.write_char('■'),
            Pixel::White => f.write_char('□'),
        }
    }
}

common::read_main!();
