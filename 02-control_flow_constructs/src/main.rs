fn main() {
    // if / else
    // se permite else if {} o usar && y ||

    let my_num = 5;
    if my_num < 10 {
        println!("El numero es menor de 10")
    } else {
        println!("El numero es mayor de 10")
    }



    // if let
    // datos del mismo tipo
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("Numero de if / let -> {}", number);



    // match
    // uso como switch
    let my_num2 = 2;

    match my_num2 {
        1 => println!("Es uno"),
        2 => println!("Es dos"),
        _ => println!("Es otro número"), // obligatorio para cubrir el resto
    }

    // asignar resultado a variables
    let operacion = "+";

    let simbolo_nombre = match operacion {
        "+" => "Suma",
        "-" => "Resta",
        _ => "No se"
    };

    println!("{}", simbolo_nombre);

    // rangos y comparadores, se pueden poner tambien if dentro de los casos
    let nota = 5;

    match nota {
        0..4 => println!("Suspenso"),
        5 | 6 => println!("Aprobado"),
        7..10 => println!("Notable/Sobresaliente"),
        _ => println!("Nota invalida")
    }

    // manejo de errores y vacios
    let texto = "5";

    // parse devuelve Ok o Err
    let my_numero3: i32 = match texto.parse() {
        Ok(num) => num, // devuelve el numero parseado
        Err(_) => {
            println!("¡Eso no es un número válido!");
            0 // devuelve 0 en caso de error
        }
    };

    println!("{}", my_numero3)



    // for



    // while



    // loop



    // break
}