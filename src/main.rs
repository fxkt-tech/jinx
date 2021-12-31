use image::{self, ImageBuffer, RgbImage};

fn main() {
    let r: u32 = 720;
    let d: u32 = r * 2;

    let mut imgbuf: RgbImage = ImageBuffer::new(r * 2, r * 2);

    for x in 0..d {
        for y in 0..d {
            let dx: u32 = if x > r { x - r } else { r - x };
            let dy: u32 = if y > r { y - r } else { r - y };
            // 圈外
            if dx.pow(2) + dy.pow(2) > r.pow(2) {
                if x < r {
                    // remu
                    *imgbuf.get_pixel_mut(x, y) = image::Rgb([145, 190, 240]);
                } else {
                    // lamu
                    *imgbuf.get_pixel_mut(x, y) = image::Rgb([235, 150, 171]);
                }
            } else {
                if r > x && r > y && x < y
                    || r < x && r > y && d - x > y
                    || r > x && r < y && x > d - y
                    || r < x && r < y && x - r > y - r
                {
                    // remu
                    *imgbuf.get_pixel_mut(x, y) = image::Rgb([145, 190, 240]);
                } else {
                    // lamu
                    *imgbuf.get_pixel_mut(x, y) = image::Rgb([235, 150, 171]);
                }
            }
        }
    }
    imgbuf.save("fxkt-tech.png").unwrap();
}
