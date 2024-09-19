const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main() {
    // Initial temperature in Fahrenheit
    let mut temp_f: f64 = 32.0;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{}°F is {:.2}°C", temp_f, temp_c);
    
    // Convert next 5 temperatures
    for _i in 0..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{}°F is {:.2}°C", temp_f, temp_c);
    }
}