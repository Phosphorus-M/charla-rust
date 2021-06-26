
fn main() {
    // let mi_numero = 2;
    let mut mi_numero = 2;
    mi_numero *= 4;

    if mi_numero > 5 {
        println!("El numero es mayor a 5");
    }else{
        println!("El numero no es mayor a 5");
    }

    // Puedes usar un pseudo ternary operator para asignar valores a variables
    // let _terniario = if true{ "pepe" } else { "sad" };
    // Rust nos pide usar un _ como prefijo de las variables si es que consideramos que son necesarias pero están sin uso actualmente.

    // let un_numero = 13;

    //Switch? Match? lo que usen
    // println!("Que es {} ?", un_numero);
    // match un_numero {
    //     1 => println!("Uno!"),
    //     2 | 3 | 5 | 7 | 11 => println!("Es un numero primo"),
    //     _ => println!("Otro, uno cualquiera, no se"),
    // }

    // El match también puede servir para asignar valores

}