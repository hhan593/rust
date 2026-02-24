pub fn transform_temperatures() {
    // 摄氏度转华氏度
    let celsius = 100.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C = {}°F", celsius, fahrenheit);

    // 华氏度转摄氏度
    let fahrenheit = 32.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}°F = {}°C", fahrenheit, celsius);
}

// 摄氏转华氏
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// 华氏转摄氏
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
