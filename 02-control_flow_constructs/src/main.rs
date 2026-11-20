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
        _ => "No se",
    };

    println!("{}", simbolo_nombre);

    // rangos y comparadores, se pueden poner tambien if dentro de los casos
    let nota = 5;

    #[allow(non_contiguous_range_endpoints)]
    match nota {
        0..4 => println!("Suspenso"),
        5 | 6 => println!("Aprobado"),
        7..10 => println!("Notable/Sobresaliente"),
        _ => println!("Nota invalida"),
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

    println!("{}", my_numero3);

    // for
    // recorre de 1 a 100
    println!("Imprimiendo 1 -> 100");
    for n in 1..101 {
        // se puede usar (1..101).rev para cuenta atras
        print!("{} ", n)
    }

    // uso del iterador
    let vector = vec!["David", "Adrian"];
    for cadena in vector.iter() {
        match cadena {
            &"David" => println!("David encontrado"),
            _ => println!("Hola {}", cadena),
        }
    }

    // while
    // necesita si o si una condicion
    let mut contador = 1;

    while contador < 10 {
        if contador % 2 == 0 {
            println!("{} divisible entre 2", contador);
        }
        contador += 1;
    }

    // loop
    // se considera infinito
    let mut contador2 = 0;

    loop {
        contador2 += 1;

        if contador2 == 3 {
            println!("Tres");
            continue;
        }

        println!("{}", contador2);

        if contador2 == 5 {
            println!("Fin");
            break;
        }
    }

    // se pueden crear etiquetas para romperlos
    #[warn(unreachable_code)]
    'exterior: loop {
        println!("Dentro del loop interior");

        '_inner: loop {
            println!("Dentro del loop interior");

            break 'exterior;
        }
        println!("Nunca se llegara aqui");
    }
    println!("Fin del loop con etiquetas");

    // break
    // salir de bucles
    let mut contador3 = 0;

    let result = loop {
        contador3 += 1;

        if contador3 == 3 {
            break contador * 2;
        }
    };

    println!("{}", result);
}
