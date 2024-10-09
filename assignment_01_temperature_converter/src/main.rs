
const FREEZING_POINT_F: f64 = 32.0;

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64
 {
    (fahrenheit - FREEZING_POINT_F) * (5.0 / 9.0)
}

#[allow(dead_code)]
fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 
{
    (celsius * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main() 
{
    let initial_temp_f = 32.0;
    let initial_celsius = convert_fahrenheit_to_celsius(initial_temp_f);
    println!("{}°F is {:.2}°C", initial_temp_f, initial_celsius);

    for increment in 1..6 
    {
        let current_temp_f = initial_temp_f + increment as f64;
        let current_celsius = convert_fahrenheit_to_celsius(current_temp_f);
        println!("{}°F is {:.2}°C", current_temp_f, current_celsius);
    }
}

