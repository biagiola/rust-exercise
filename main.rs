use std::collections::HashMap; // for our pseudo tables
mod models; // Import our pseudo models
use models::{User, Avatar, AvatarType, UserAvatar}; // Bring structs/enum

fn tbl_insert_avatar(
    avatar_tbl: &mut HashMap<u32, Avatar>,
    id: u32,
    avatar: Avatar
) {
    avatar_tbl.insert(id, avatar);
}

fn main() {
    // create user
    let david_user: User = User {
        id: 1,
        first_name: String::from("David"),
        last_name: String::from("Biagiola"),
        nick_name: String::from("Deivi")
    };
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