use std::{io, vec};

struct User {
    id: u8,
    name: String,
    lastname: String,
    email: String,
    age: u8,
    active: bool
}

fn main() {
    let mut users:Vec<User> = Vec::new();

    let mut menu_running:bool = true;
    let mut option = String::new();

    while menu_running {
        println!("¿Que desea hacer?");
        println!("1)- Crear usuario");
        println!("2)- Listar usuarios");
        println!("0)- Salir");

        let input = loop {
            option.clear();

            io::stdin().read_line(&mut option).expect("Error al ingresar opción");
            match option.trim().parse::<u8>() {
                Ok(number) => break number,
                Err(_) => {
                    println!("Ingresá un número válido");
                    continue;
                }
            }
        };

        match input {
            1 => {
                let user: User = create_user(&users);
                users.push(user);
            },
            2 => {
                list_users(&users);
            }
            0 => {
                menu_running = false;
            },
            _ => println!("Se espera una opción válida")
        }
    }
}

fn create_text(text:&str) -> String{
    loop {
        let mut input = String::new();
        println!("Ingresá el {}", text);
        io::stdin().read_line(&mut input).expect("Error al ingresar");
        let input = input.trim().to_lowercase();
        if !input.is_empty() {
            return input;
        }
        println!("El campo no puede estar vacío");
    }
}

fn create_number() -> u8 {
    loop {
        let mut input = String::new();
        println!("Ingresá la edad");
        io::stdin().read_line(&mut input).expect("Error al ingresar");
        match input.trim().parse::<u8>() {
            Ok(number) => break number,
            Err(_) => {
                println!("Debe ingresar un número válido");
                continue;
            }
        }
    }
}

fn create_user(users: &Vec<User>) -> User {
    let name = create_text("nombre");
    let lastname = create_text("apellido");
    let age:u8 = create_number();
    let email = create_text("email");

    let mut last_id:u8 = users.len().try_into().expect("El valor tiene más de 255 elementos");

    last_id = last_id + 1;

    return User {
        id: last_id,
        name,
        lastname,
        age,
        email,
        active: true
    };
}

fn list_users(users: &Vec<User>) {
    if users.is_empty() {
        println!("No hay usuarios para listar");
    } else {
        for user in users {
            println!("Id: {}", user.id);
            println!("Nombre: {}", user.name);
            println!("Apellido: {}", user.lastname);
            println!("Edad: {}", user.age);
            println!("Correo: {}", user.email);
            if user.active {
                println!("Estado: activo");
            }else {
                println!("Estado: inactivo");
            }
        }
    }
}