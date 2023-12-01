use std::env;

fn ma_fonction(string_a_couper: &str, string_separateur: &str) -> Vec<String> {
    string_a_couper
        .split(string_separateur)
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Vérification des arguments
    if args.len() != 3 {
        eprintln!("error");
        std::process::exit(1);
    }

    let string_a_couper = &args[1];
    let string_separateur = &args[2];

    let resultat = ma_fonction(string_a_couper, string_separateur);

    for element in resultat {
        println!("{}", element);
    }
}
