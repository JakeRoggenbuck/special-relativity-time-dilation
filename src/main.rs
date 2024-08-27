const C: f64 = 299_792_458.0;

/// t: time in seconds
/// v: km/s
fn time_dilation_by_velocity(t: f64, v: f64) -> f64 {
    t / (1.0 - (v * v) / (C * C)).sqrt()
}

fn main() {
    let a: f64 = time_dilation_by_velocity(60.0 * 60.0, 300.0);
    println!("{}", a);
}
