use std::collections::HashMap;

fn main() {
    println!("GESTOR INVENTARIO");

    let mut map: HashMap<String, i32> = std::collections::HashMap::new();

    // inserta sin problema
    map.insert(String::from("Manzana"), 10);
    map.insert(String::from("Pan"), 43);
    map.insert(String::from("Leche"), 5);

    println!("{:?}", map);

    map.entry(String::from("Queso")).or_insert(20); // bien
    map.entry(String::from("Pan")).or_insert(50); // no se inserta

    println!("{:?}", map);

    // tener la cantidad inicial
    let cantidad_incial = map.get("Pan");
    let mut cantidad_final = 0;
    // calcular la cantidad final
    match cantidad_incial {
        Some(value) => cantidad_final = value - 35,
        None => {}
    }

    // insertar nueva cantidad
    map.insert(String::from("Pan"), cantidad_final);

    // recorrerlo todo
    for (ingrediente, cantidad) in &map {
        println!("{} {}", ingrediente, cantidad);
    }
}

/* VERSION OPTIMIZADA
// si exste la referencia a Pan obtener referencia mutable
if let Some(cantidad) = map.get_mut("Pan") {
    *cantidad -= 35; // modificar valor directamente
}
*/
