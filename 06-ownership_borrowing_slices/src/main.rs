/* Normas universales
 * 1. Por defecto las variables son inmutables
 * 2. Cada valor tiene un dueño
 * 3. Solo se puede tener un dueño a la vez
 * 4. Si el dueño sale del foco, el valor se droppea
 * 5. Se pueden tener x cantidad de referencias
 * 6. Solo se puede tener una referencia mutable activa
 * 7. No se puede usar una referencia mutable si hay una inmutable activa
 */

fn main() {
    // OWNERSHIP
    // basicos
    let mut s = String::from("Hola"); // String

    s.push_str(", Mundo"); // append literal al final

    println!("{}", s); // Hola, Mundo

    // funciones
    let cadena = String::from("cadena");

    // s deja de existir aqui y pasa a existir solo a la funcion
    takes_ownership(cadena);

    let x = 5;

    // x sigue existiendo por las reglas de Copy
    makes_copy(x);

    // BORROWING (inmutable)
    let my_string = String::from("value");

    // pasamos una referencia al valor (obtiene el valor sin volverse owner)
    let len = calculate_lenght(&my_string);

    println!("{}", len);

    // BORROWING (mutable)
    let mut my_mutable_string = String::from("value");

    let new_mutable_string = change(&mut my_mutable_string);

    println!("{:?}", new_mutable_string);

    // Casos de error
    // una referencia mutable activa a la vez que una inmutable
    let mut other_string = String::from("hello");

    let r1 = &other_string; // no problem
    let r2 = &other_string; // no problem
    let r3 = &mut other_string; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");

    // SLICE
    let slice = String::from("hello world");

    let hello = &slice[0..5];
    let world = &slice[6..11];

    let slice_first = &slice[..5];
    let slice_last = &slice[6..];
    let slice_all = &slice[..];

    let definir_slice: &str = "My slice"; // no hace falta especificar &str

    // slices se otros tipos de dato
    let array = [1, 2, 3, 4];

    let array_slice = &array[1..3]; // 2,3,4
}

// OWNERSHIP
fn _comprobar_scope() {
    // aqui s no existia
    let _s = "hola";

    // operar con s
} // aqui s deja de existir (end of scope)

fn basic_ownership() {
    let s1 = String::from("hola");
    let s2 = s1; // reasigna el valor de s1

    println!("{}", s1); // s1 dejo de existir, ownership en s2
}

fn drop_ownership() {
    let mut s = String::from("hola");
    s = String::from("aloh"); // drop al valor anterior y reasigna

    println!("{}", s); // aloh
}

fn clone() {
    let s1 = String::from("hola");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}"); // hola - hola
}

fn valid_ownership() {
    // int, bool, float, char y tuplas(con solo estos tipos)
    // tipos que ya tienen un tamaño concreto asignado
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

fn takes_ownership(cadena: String) {
    println!("{cadena}");
} // cadena deja de existir

fn makes_copy(entero: i32) {
    println!("{entero}");
} // entero sigue existiendo en main

// BORROWING
// recibe una referencia INMUTABLE a String
fn calculate_lenght(s: &String) -> usize {
    s.len() // devuelve usize
}

// recibe una referencia MUTABLE a String
fn change(s: &mut String) {
    s.push_str(", append");
}

// SLICE
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // convierte el string a bytes

    // mas detalles a futuro
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // al detectar un espacio
            return &s[0..i]; // añade desde el inicio hasta ahi
        }
    }
    &s[..] // devuelve lo encontrado
}
