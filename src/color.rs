use rand;
use rand::Rng;
use sdl2::pixels::Color;

pub fn random_color_hsv() -> Color {
    let mut rng = rand::thread_rng();
    let h = rng.gen_range(0, 360);
    let s = rng.gen_range(0, 101);
    let v = rng.gen_range(50, 101);

    hsv_to_rgb(h, s, v)
}

fn map_to_range(p: i32) -> u8 {
    const M: f32 = 255.0f32 / 100.0f32;
    (p as f32 * M) as u8
}

// h: [0, 360)
// s: [0, 100]
// v: [0, 100]
fn hsv_to_rgb(h: i32, s: i32, v: i32) -> Color {
    let hi = (h / 60) % 6;
    let vmin = ((100 - s) * v) / 100;
    let a = (v - vmin) * (h % 60) / 60;
    let vinc = vmin + a;
    let vdec = v - a;

    let (r, g, b) = match hi {
        0 => (v, vinc, vmin),
        1 => (vdec, v, vmin),
        2 => (vmin, v, vinc),
        3 => (vmin, vdec, v),
        4 => (vinc, vmin, v),
        5 => (v, vmin, vdec),
        _ => panic!("Unexpected Hi value"),
    };

    Color::RGB(map_to_range(r), map_to_range(b), map_to_range(g))
}
