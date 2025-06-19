fn main() {
    println!("Hello world!");

    let age: i32 = 23;

    if age >= 18 {
        println!("Je suis majeur!");
    } else {
        println!("Je suis mineur!");
    }

    if age >= 18 && age <= 35 {
        println!("Donc je suis adulte!");
    }

    // Pattern Matching :
    let my_string = "lingala";
    match my_string {
        "bonjour" => println!("Français !"),
        "ciao" => println!("Italien !"),
        "hello" => println!("Anglais !"),
        "hola" => println!("Espagnol !"),
        _ => println!("Je ne connais pas cette langue !"),
    }

    // Booléen :
    let b = true;
    match b {
        true => println!("true"),
        false => println!("false "),
    }

    // Boucle for
    let mut x = 0;
    for _ in 0..5 {
        x += 1;
    }
    println!("Valeur finale de x = {}", x);

    // Appel de la fonction addition
    println!("36 + 12 = {}", addition(36, 12));

    // Tuple (entier, float)
    let tuple = entier_et_float();
    println!("entier : {}, float : {}", tuple.0, tuple.1);

    // Appel de la fonction get_bigger
    let plus_grand = get_bigger(7, 12);
    println!("Le plus grand nombre est : {}", plus_grand);
}


// Déclaration des fonctions EN DEHORS de main()

fn addition(nb1: i32, nb2: i32) -> i32 {
    nb1 + nb2
}

fn entier_et_float() -> (usize, f32) {
    (12, 0.1)
}

fn get_bigger(nb1: i32, nb2: i32) -> i32 {
    if nb1 > nb2 {
        nb1
    } else {
        nb2
    }
}
