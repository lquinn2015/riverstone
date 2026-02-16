use std::error::Error;

use image::{DynamicImage, ImageBuffer, ImageReader, RgbImage, RgbaImage};

fn main() {
    let img = ImageReader::open("test.jpg").unwrap().decode().unwrap();

    let floyd_steinberg: Vec<u8> = vec![
        0b11100000, // 7/16
        0b01010000, // 3/16
        0b00110000, // 5/16
        0b00010000, // 1/16
    ];

    let akinson: Vec<u8> = vec![
        0b0010000, // 1/8
        0b0010000, // 1/8
        0b0010000, 0b0010000, 0b0010000, 0b0010000,
    ];

    // GBA
    //let palettle: Vec<u32> = vec![0x000000, 0x005500, 0x55aa55, 0xaaffaa];

    // cultist dither
    //let palettle: Vec<u32> = vec![
    //    0x001c40, 0x1e2a5f, 0x235662, 0x5464cf, 0xcb8bf0, 0x75d7da, 0x9effb8,
    //];

    // 1 bit dither - akinson
    let palettle: Vec<u32> = vec![0xffffff, 0x000000];

    ////let palettle = vec![
    //    0x2f3b3d, 0x464b4f, 0x5c6163, 0x7b7d77, 0x999991, 0xb5b2ac, 0xd4d0cd, 0xebf0ee, 0x57483b,
    //    0x6e5f4d, 0x8a7b63, 0xa3987a, 0xbdb395, 0xd6d0b0, 0x614257, 0x7a586a, 0x997482, 0xb39196,
    //    0xc9adab, 0xdecbbf, 0x444a66, 0x566178, 0x6c8091, 0x839ea6, 0x99bab5, 0xbed4c8, 0x5e4452,
    //    0x80575b, 0x9e7565, 0xba9273, 0xd1ae8a, 0x5c4644, 0x785a55, 0x9c756a, 0xb89184, 0xccad9b,
    //    0x8f3648, 0xb04a58, 0xcc6764, 0xe38674, 0xe8a68e, 0xebcbbc, 0x8a3c24, 0x9e5333, 0xbd6f42,
    //    0xd48d57, 0xe0ac6c, 0xe8cd97, 0x855c22, 0x9e7a36, 0xba9745, 0xccb45c, 0xe3d176, 0xe6dfa1,
    //    0x32571d, 0x4b6c2e, 0x6d8740, 0x8aa355, 0xaabd68, 0xcfd496, 0x255461, 0x346c70, 0x4d8a7e,
    //    0x68a88e, 0x8ac290, 0xb7d9a9, 0x255269, 0x336c7a, 0x438c91, 0x5ba9a4, 0x80c2ac, 0xabdbb8,
    //    0x364996, 0x4761ad, 0x5782ba, 0x709fcf, 0x8cbade, 0xadd6e0, 0x46449c, 0x5d59b3, 0x7c75c9,
    //    0xa08fdb, 0xc0aae3, 0xd6caeb, 0x683b8a, 0x864ea6, 0xa46abd, 0xc385d6, 0xd8a3e3, 0xe8c5e6,
    //    0x85347a, 0xa8487f, 0xc4668c, 0xdb84a1, 0xe6a3af, 0xebc7ca,
    //];

    let algo = false;
    let is_rgba = false;
    let dmatrix = if algo { akinson } else { floyd_steinberg };

    //let nimg = dither_img(img, floyd_steinberg, palettle).unwrap();
    let nimg = dither_img(img, dmatrix, palettle, algo, is_rgba).unwrap();

    let file = std::fs::File::create("tmp.png").unwrap();
    nimg.write_to(file, image::ImageFormat::Png).unwrap();
}

fn in_bounds(x: isize, y: isize, img: &DynamicImage) -> bool {
    x >= 0 && y >= 0 && x < img.width() as isize && y < img.height() as isize
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl From<&[u8]> for Color {
    fn from(v: &[u8]) -> Self {
        Color {
            r: v[0],
            g: v[1],
            b: v[2],
        }
    }
}

impl From<u32> for Color {
    fn from(v: u32) -> Self {
        Color {
            r: ((v >> 16) & 0xFF) as u8,
            g: ((v >> 8) & 0xFF) as u8,
            b: (v & 0xFF) as u8,
        }
    }
}

fn dist_u8(v0: u32, v1: u32) -> u32 {
    let c0 = Color::from(v0);
    let c1 = Color::from(v1);

    i32::pow(c0.r as i32 - c1.r as i32, 2) as u32
        + (i32::pow(c0.b as i32 - c1.g as i32, 2) as u32)
        + (i32::pow(c0.g as i32 - c1.b as i32, 2)) as u32
}

fn dither_img(
    img: DynamicImage,
    dmatrix: Vec<u8>,
    palettle: Vec<u32>,
    algo: bool,
    is_rgba: bool,
) -> Result<DynamicImage, Box<dyn Error>> {
    let num_chan: isize = if is_rgba { 4 } else { 3 };

    let img_buf = if is_rgba {
        img.as_rgba8().unwrap().as_raw()
    } else {
        img.as_rgb8().unwrap().as_raw()
    };
    let mut dither = vec![0xffu8; (img.width() * img.height() * num_chan as u32) as usize];
    dither.clone_from_slice(&img_buf);

    for cy in 0..img.height() {
        for cx in 0..img.width() {
            let cx = cx as isize;
            let cy = cy as isize;
            let i = ((cx + cy * img.width() as isize) * num_chan) as usize;
            let r = dither[i];
            let g = dither[i + 1];
            let b = dither[i + 2];
            let rgb2 = Color::from(&[r, g, b] as &[u8]);
            let rgb = Color::from(&[r, g, b] as &[u8]).to_u32();

            // find max palettle
            let (sel_col, _) = palettle.iter().skip(1).fold(
                (palettle[0], dist_u8(rgb, palettle[0])),
                |acc, nc| {
                    let dist = dist_u8(rgb, *nc);
                    if dist < acc.1 {
                        (*nc, dist)
                    } else {
                        acc
                    }
                },
            );

            // dither error to components
            let col = Color::from(sel_col);
            dither[i] = col.r;
            dither[i + 1] = col.g;
            dither[i + 2] = col.b;
            let dr: i16 = (col.r as i16 - rgb2.r as i16) >> 1;
            let dg: i16 = (col.g as i16 - rgb2.g as i16) >> 1;
            let db: i16 = (col.b as i16 - rgb2.b as i16) >> 1;

            let floyd = [(1, 0), (-1, 1), (0, 1), (1isize, 1isize)];
            let akinson = [(1, 0), (2, 0), (-1, 1), (0, 1), (1, 1), (0isize, 2isize)];
            let pat = if algo {
                &akinson as &[(isize, isize)]
            } else {
                &floyd as &[(isize, isize)]
            };

            //for (mat_i, (dx, dy)) in floyd.iter().enumerate() {
            for (mat_i, (dx, dy)) in pat.iter().enumerate() {
                let px: isize = cx + dx;
                let py: isize = cy + dy;
                if in_bounds(px, py, &img) {
                    let p = ((px + py * img.width() as isize) * num_chan) as usize;
                    dither[p] =
                        (dither[p] as i32 - (((dr as i32) * dmatrix[mat_i] as i32 + 1) >> 8)) as u8;
                    dither[p + 1] = (dither[p + 1] as i32
                        - (((dg as i32) * dmatrix[mat_i] as i32 + 1) >> 8))
                        as u8;
                    dither[p + 2] = (dither[p + 2] as i32
                        - (((db as i32) * dmatrix[mat_i] as i32 + 1) >> 8))
                        as u8;
                }
            }
        }
    }
    if is_rgba {
        let mut rimg = RgbaImage::new(img.width(), img.height());
        rimg.copy_from_slice(&dither);
        let rimg = DynamicImage::ImageRgba8(rimg);
        Ok(rimg)
    } else {
        let mut rimg = RgbImage::new(img.width(), img.height());
        rimg.copy_from_slice(&dither);
        let rimg = DynamicImage::ImageRgb8(rimg);
        Ok(rimg)
    }
}
