#[allow(unused_variables)]
use std::fmt::Debug;
use std::io;

fn input(text: &str) -> String {
    let mut input = String::new();
    println!("{}", text);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input_sem_quebra_de_linha = input.replace('\n', "");
    input_sem_quebra_de_linha
}

fn print<T: Debug>(text: T) -> () {
    println!("{:?}", text);
}

fn main() {
    let nome = input("Qual seu nome? ");

    println!("---");
    println!("O seu nome é {}", nome);
    // Ou tmb...
    // println!("O seu nome {nome}", nome=nome);
    // println!("O seu nome {nome}", nome); /* JavaScript Style */
    println!("---");

    print("Por que não interar e imprimir cada caracter?");
    for i in nome.chars() {
        print(i);
    }

    print("Ou só os nove primeiros");
    // Vamos fazer um for por index ao inves de iterators
    // Mas calma lá, nao tao simples...
    for i in 0..9 {
        let char_in_index = nome.chars().nth(i);
        print(char_in_index);
    }

    print("Pera, imprimiu estranho... Vamos lidar com isso");
    // Vamos fazer um for por index ao inves de iterators
    for i in 0..9 {
        let char_in_index = nome.chars().nth(i);

        // Esse cha é do tipo "Option" => https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap.html
        // Ele pode ter um valor de Some(), quando retorna um sucesso na operação, ou None
        // Esse match é como um switch case, mas WAY TOO BETTER!
        match char_in_index {
            Some(i) => print(i), // 'i' é a variavel feita o 'unwrap()'
            None => print("--"),
        }
    }

    // Isso dispara um panic! em caracteres que no for passado retorna o None, visto que o unwrap
    // faz já o retorno da Option, dando panic se achar um None. Só use se souber que haverá erros
    // for i in 0..3 {
    //     let char_in_index = nome.chars().nth(i).unwrap();
    //     print(char_in_index);
    // }

    print("vamo só imprimir numeros num for...");
    for i in 0..3 {
        print(i); // 0, 1, 2
    }
    for i in 0..=3 {
        print(i); // 0,1,2,3
    }

    // Vectors --------------------------------------------
    // https://doc.rust-lang.org/std/vec/

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        print(name) // "Bob", "Frank", "Ferris"
    }

    // ERROR: Isso não é possivel já que a variavel é imutável
    // names.push("Vinicius");

    let mut new_names = names; // Moved: names -> new_names
    new_names.push("Vinicius");
    new_names.push("Bob");

    print("--");

    for name in new_names.iter() {
        print(name) // "Bob", "Frank", "Ferris", "Vinicius", "Bob"
    }

    // Eu posso iterar num vec movendo o owing dele para dentro do for-loop
    for name in new_names.into_iter() {
        print(name) // "Bob", "Frank", "Ferris", "Vinicius", "Bob"
    }

    // ERROR: borrow of moved value
    // new_names.push("Marley");

    let planetas: Vec<String> = Vec::new();
    // let planetas: Vec<String> = vec![];

    // Apesar de .push não ser valido em um immutable vec, isso é permitido
    let planetas = vec!["Mercurio"];
    let planetas = vec!["Mercurio", "Venus"];
    let planetas = vec!["Mercurio", "Venus", "Terra"];

    for (i, pl) in planetas.iter().enumerate() {
        println!("O {} planeta é {}", i + 1, pl);
    }

    // ---

    let mut idades = vec![19, 21, 17, 30];

    for pl in idades.iter() {
        // ERROR
        // *pl += 5;
    }

    for pl in idades.iter_mut() {
        // Ok
        *pl += 10;
        print(pl)
    }

    // Box -------------------------------------------
    //
    //
    // HashMap ---------------------------------------
    use std::collections::HashMap;

    struct User {
        name: String,
        password: String,
    }

    type TwoNumbers = (u8, u8);

    // Poderia também fazer o seguinte para usar uma struct como tipo valido numa Key do HashMap...
    // ..
    // #[derive(PartialEq, Eq, Hash)]
    // struct TwoNumbers(u8, u8);

    let mut users: HashMap<TwoNumbers, User> = HashMap::new();

    users.insert(
        (23, 2),
        User {
            name: "Talis".to_string(),
            password: "123".to_string(),
        },
    );

    print("Usuários:");
    print(" - Talis || PID: 23,2");
    print("");
    print("Digite o pid do usuario acima. Primeiro um numero depois o outros...");
    let p1: u8 = input("1) ").parse().unwrap(); // Como está sendo usado um 'unwrap' sem tratamento
                                                // ao inves de um match, se o parse dar errado ele
                                                // dispara um panic!
    let p2: u8 = input("2) ").parse().unwrap();

    let default = User {
        name: "Digitou errado".to_string(),
        password: "***".to_string(),
    };

    let user_selected = users.get(&(p1, p2)).unwrap_or(&default);

    println!("User => {}", user_selected.name);
    println!("Password => {}", user_selected.password);
}
