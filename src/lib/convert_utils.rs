const FACTOR: f64 = 1.8;

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * FACTOR + 32.0
}

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / FACTOR
}
