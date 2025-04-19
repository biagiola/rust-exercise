use std::collections::HashMap; // for our pseudo tables
use std::process;
use std::io::stdin;
use std::io::Error;
use models::{User, Avatar, AvatarType, UserAvatar}; // Bring structs/enum
mod models; // Import our pseudo models

fn tbl_insert_avatar(
    avatar_tbl: &mut HashMap<u32, Avatar>,
    id: u32,
    avatar: Avatar
) {
    avatar_tbl.insert(id, avatar);
}

fn read_user_identification(message: &str, retries: i32) -> String {
    let mut user_input: String = String::new();
    let mut i: i32 = 0;

    loop {
        if i >= retries {
            eprintln!("Too many failed attempts. Exiting...");
            process::exit(1)
        }
        println!("{message} (Attempt {} of {})", i + 1, retries);


        user_input.clear(); // just in case

        if let Err(error) = stdin().read_line(&mut user_input) { // I know, very rare scenario
            eprintln!("There was an error with the user input. {error:?}");
            i += 1;
            continue;
        }
        break
    }
    user_input.trim().to_string()
}

fn main() {
    let retries: i32 = 3;
    let input_first_name: String = read_user_identification("Please enter your first name.", retries);
    let input_last_name: String = read_user_identification("Please enter your last name.", retries);
    let input_nick_name: String = read_user_identification("Please enter your nick name.", retries);

    // create user
    let david_user: User = User {
        id: 1,
        first_name: input_first_name,
        last_name: input_last_name,
        nick_name: input_nick_name
    };
    println!("{david_user:?}");
    // create avatar
    let gandalf_avatar: Avatar = Avatar {
        id: 1,
        name: String::from("Gandalf"),
        avatar_type: AvatarType:: Magician
    };
    let frodo_avatar: Avatar = Avatar {
        id: gandalf_avatar.id + 1,
        name: String::from("Frodo"),
        avatar_type: AvatarType::Hobbit
    };
    // fake save
    let mut avatar_tbl: HashMap<u32, Avatar> = HashMap::new();
    tbl_insert_avatar(&mut avatar_tbl, gandalf_avatar.id, gandalf_avatar);
    tbl_insert_avatar(&mut avatar_tbl, frodo_avatar.id, frodo_avatar);
    // println!("Insertion: {:#?}", avatar_tbl.get(&1));

    // Access HasMap data: safe mode
    // if let Some(avatar) = avatar_tbl.get(&1) {
    //     println!("Avatar name: {}", avatar.name);
    // }
    // Access HasMap data: unsafe mode
    // println!("Avatar name: {}", avatar_tbl.get(&1).unwrap().name);

    // create userAvatar: user -> avatar
    let david_frodo_avatar: UserAvatar = UserAvatar {
        id: 1,
        user_id: david_user.id,
        avatar_id: avatar_tbl.get(&2).unwrap().id,
        level: 20
    };
    println!("{:#?}", david_frodo_avatar);
}

// TODO: make more modular fn's for saving values into the hashMap and others.