use num::complex::Complex;
use image::{ImageBuffer, RgbImage, Rgb};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use substring::Substring;

fn membership(x: f64, y:f64, n:f64, m:f64) -> f64{
    let max_iterations = 100;
    let config = Complex::<f64>::new(n, m);
    let mut z = Complex::<f64>::new(x, y);
    let mut count = 0;
    while z.norm() <= 2.0 && count < max_iterations {
       z = z * z + config;
       //println!("z: {}", z);
       count += 1;
    }
    //println!("count: {}", count);
    let infinity_prop: f64 = (count as f64 / max_iterations as f64) as f64;
    1.0 - infinity_prop
}


fn main() {

    let (width, height) = (100, 100);
    let iterations = 10000;
    let base_n = -1.0;
    let base_m = 0.0;

    for iter in 0..iterations {

        let mut image: RgbImage = ImageBuffer::new(width, height);

        let (sender, receiver) = channel();
        let pool = ThreadPool::new(8);

        for r in 0..height {
            let sender = sender.clone();
            pool.execute(move || {
                for c in 0..width {
                    let x: f64 = (c as f64/ (width as f64/ 4.0)) - 2.0;
                    let y: f64 = (r as f64/ (height as f64/ 4.0)) - 2.0;
                    let n: f64 = base_n + (iter as f64) * 0.001;
                    let m: f64 = base_m + (iter as f64) * 0.001;
                    let membership = membership(x, y, n, m);
                    let max_hex = 16777215.0; // ffffff in base 10
                    let new_hex = ((max_hex * membership) as u64).to_string();
                    let string_rgb = format!("{:0>8}", new_hex);                
                    let s_red = (string_rgb.substring(0, 2).parse::<f64>().unwrap() / 15.0 * 255.0) as u8;
                    let s_green = (string_rgb.substring(3, 4).parse::<f64>().unwrap() / 15.0 * 255.0) as u8;
                    let s_blue = (string_rgb.substring(5, 6).parse::<f64>().unwrap() / 15.0 * 255.0) as u8;
                    let rgb = Rgb([s_red, s_green, s_blue]);
                    sender.send((c, r, rgb)).unwrap();
                }
            });
        }
        for _ in 0..(width * height) {
            let (c, r, rgb) = receiver.recv().unwrap();
            image.put_pixel(c, r, rgb);
        }
        let iter_num_str = format!("{:0>8}", &iter.to_string());
        image.save("../../images/".to_string() + &iter_num_str +".png").unwrap();
    }

    println!("image made!");
}
