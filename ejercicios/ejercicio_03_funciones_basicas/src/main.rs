fn main() {
    println!("FUNCIONES BASICAS");

    println!("12 es par? -> {}", is_even(12));
    println!("13 es par? -> {}", is_even(13));

    println!("Precio final -> {}", calculate_final_price(100.0));

    println!("Nombre formateado -> {}", format_username(" David Adrian "));
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

// impuesto por defecto del 21%
fn calculate_final_price(precio: f64) -> f64 {
    precio + (precio * 0.21)
}

fn format_username(name: &str) -> String {
    name.trim().to_uppercase()
}
