fn main() {
    println!("BUSCANDO A NEMO");

    // el segundo ejemplo no encontraria nada
    let my_string = String::from("I am finding Nemo !");
    // let my_string = String::from("I am finding nemo !");
    let my_split_string = my_string.split(' ');

    let mut cont = 1;
    let mut flag = false;

    for parte in my_split_string {
        if parte == "Nemo" {
            println!("\nNemo encontrado en posicion: {}", cont);
            flag = true;
            break;
        }
        cont += 1;
    }

    if !flag {
        println!("No encontre a Nemo :(");
    }
}
