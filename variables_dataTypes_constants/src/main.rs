fn main() {
    // tipos de dato
    /*
     * Enteros
     * Flotante
     * Boolean
     * Char
     * String
     * Tuplas
     * Array
     */

    // variables
    let my_string = "Cadena de texto";
    println!("{}", my_string);


    // variables mutables
    let mut my_muable_string = "Cadena de texto mutable";
    println!("{}", my_muable_string);

    my_muable_string = "Valor de la cadena cambiado";
    println!("{}", my_muable_string);


    // formas de declarar Strings
    let _my_string_example1 = "Este string ocupa el espacio del tipo";
    let _my_string_example2 = String::from("Este string ocupa el espacio justo");

    // tipos de dato simples
    let my_int = 5;
    let my_float = 4.5;
    let my_bool = true;
    let my_char = 'A';

    // tuplas
    // las tuplas sirven para almacenar conjuntos de tipos distintos
    let tupla = (my_int, my_float, my_bool, my_char);

    println!("Tupla -> {:?}", tupla);

    let dato1 = tupla.0;
    let dato2 = tupla.1;
    let dato3 = tupla.2;
    let dato4 = tupla.3;  

    println!("Datos por Separado -> {dato1} / {dato2} / {dato3} / {dato4}");


    // arrays
    // los arrays almacenan conjuntos de datos del mismo tipo
    let array = [1,2];

    let primero = array[0];
    let segundo = array[1];

    println!("Extraer del array -> {primero} / {segundo}");


    // constantes
    const MY_CONST: &str = "Constante";

    println!("Variable constante -> {}", MY_CONST);
}
