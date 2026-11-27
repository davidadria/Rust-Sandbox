use std::collections::{HashMap, HashSet};
use std::vec;

fn main() {
    vector();
    hashmap();
    hashset();
}

// lista de tamaño indefinido
fn vector() {
    // declarar vacio
    let _vector_vacio: Vec<i32> = Vec::new();

    // declarar con valores
    let _vector_valores = vec![1, 2, 3];

    // añadir valores
    let mut vector_updated = Vec::new();

    // hay que mostrar valores con :?
    println!("{:?}", vector_updated);

    vector_updated.push(1);
    vector_updated.push(2);
    vector_updated.push(3);

    println!("{:?}", vector_updated);

    // leer elementos
    let vector_buscar = vec![1, 2, 3];

    // de forma directa
    let pos_dos: &i32 = &vector_buscar[1];
    println!("Segunda pos del vector -> {}", pos_dos);

    // value check
    let pos_dos: Option<&i32> = vector_buscar.get(1);
    match pos_dos {
        Some(valor) => {
            println!("Segunda pos del vector -> {:?}", valor);
        }
        None => {
            println!("No encontrado");
        }
    }

    // iterar sobre array
    let vector_iterar = vec![1, 2, 3];
    for num in &vector_iterar {
        println!("{}", num);
    }

    let mut vector_iterar_mutable = vec![1, 2, 3];
    for num in &mut vector_iterar_mutable {
        *num += 50; // puntero hacia la direccion de memoria
        println!("{}", num);
    }
}

// parejas clave-valor
fn hashmap() {
    // declaracion
    // no hace falta especificar valores, pero es buena practica
    let mut map: HashMap<i32, &str> = HashMap::new();

    map.insert(1, "David");
    map.insert(2, "Adrian");

    println!("{:?}", map);

    // acceder a un valor
    let id_buscar = 1;
    let nombre_buscado = map.get(&id_buscar).copied().unwrap_or("ERROR");

    println!("{}", nombre_buscado);

    // recorrer con un bucle
    for (key, value) in &map {
        println!("{} -> {}", key, value);
    }

    // insertar tipos que incluyen .copy()
    let key = 3;
    let value = "Felipe";

    map.insert(key, value);
    println!("{:?}", map);

    // insertar valores comprobando clave
    let mut scores = HashMap::new();
    scores.insert(String::from("Azul"), 10);

    scores.entry(String::from("Rojo")).or_insert(20);
    scores.entry(String::from("Azul")).or_insert(50); // no se inserta

    println!("{:?}", scores);

    // uso como contador
    let texto = "hola caracola que tal hola que";

    let mut mapa_contador = HashMap::new();

    for palabra in texto.split_whitespace() {
        let cont = mapa_contador.entry(palabra).or_insert(0);
        *cont += 1; // puntero hacia la direccion de memoria
    }

    println!("{:?}", mapa_contador);
}

// parejas claveNOrepetidas-valor
fn hashset() {
    // declaracion simple
    let mut set = HashSet::new();

    // insertar
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(3); // no se guarda

    // comprobar si se contiene
    if !set.contains(&4) {
        println!("No se encontro el valor 4");
    }

    // borrar
    set.remove(&2);

    println!("{:?}", set);

    // recorrer
    for num in set {
        println!("{}", num);
    }

    // crear set con tamaño concreto
    let _set2: HashSet<i32> = HashSet::with_capacity(10);
}
