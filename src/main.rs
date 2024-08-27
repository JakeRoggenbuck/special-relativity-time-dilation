const C: f64 = 299_792_458.0;

/// t: time in seconds
/// v: km/s
fn time_dilation_by_velocity(t: f64, v: f64) -> f64 {
    t / (1.0 - (v * v) / (C * C)).sqrt()
}

fn derivative_time_dilation_by_velocity(t: f64, v: f64) -> f64 {
    t * (-1.0 / 2.0 * (1.0 - ((v * v) * (C * C))).powf(-3.0 / 2.0) * -(2.0 * v) / (C * C))
}

fn main() {
    let a: f64 = time_dilation_by_velocity(60.0 * 60.0, 300.0);
    println!("{}", a);

    let b: f64 = derivative_time_dilation_by_velocity(60.0 * 60.0, 300.0);
    println!("{}", b);
}
