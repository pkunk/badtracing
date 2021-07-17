use badtracing::{write_color, Color};

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let color = Color::new(
                f64::from(i) / f64::from(IMAGE_WIDTH - 1),
                f64::from(j) / f64::from(IMAGE_HEIGHT - 1),
                0.25,
            );
            write_color(color);
        }
    }

    eprintln!("\nDone.");
}
