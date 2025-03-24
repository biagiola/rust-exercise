use std::collections::HashMap; // for our pseudo tables
mod models; // Import our pseudo models
use models::{User, Avatar, AvatarType, UserAvatar}; // Bring structs/enum

fn tbl_user_insert(
    tbl_user: &mut HashMap<u32, User>,
    id: u32,
    first_name: String,
    last_name: String,
    nick_name: String
) {
    let user: User = User {
        id: 1,
        first_name: first_name,
        last_name: last_name,
        nick_name: nick_name
    };
    tbl_user.insert(id, user);
}

fn tbl_avatar_insert(
    tbl_avatar: &mut HashMap<u32, Avatar>,
    id: u32,
    name: String,
    avatar_type: AvatarType
) {
    let avatar: Avatar = Avatar {
        id: id,
        name: name,
        avatar_type: avatar_type
    };
    tbl_avatar.insert(id, avatar);
}

fn tbl_user_avatar_insert(
    tbl_user_avatar: &mut HashMap<u32, UserAvatar>,
    id: u32,
    user_id: u32,
    avatar_id: u32,
    level: u32
) {
    let user_avatar: UserAvatar = UserAvatar {
        id: id,
        user_id: user_id,
        avatar_id: avatar_id,
        level: level
    };
    tbl_user_avatar.insert(id, user_avatar);
}

fn main() {
    // CREATE TABLES
    let mut tbl_user: HashMap<u32, User> = HashMap::new();
    let mut tbl_avatar: HashMap<u32, Avatar> = HashMap::new();
    let mut tbl_user_avatar: HashMap<u32, UserAvatar> = HashMap::new();

    // create user
    tbl_user_insert(
        &mut tbl_user,
        1,
        String::from("David"),
        String::from("Biagiola"),
        String::from("Deivi")
    );

    // create avatars
    tbl_avatar_insert(
        &mut tbl_avatar,
        1,
        String::from("Gandalf"),
        AvatarType::Magician
    );
    tbl_avatar_insert(
        &mut tbl_avatar,
        2,
        String::from("Frodo"),
        AvatarType::Hobbit
    );
    // println!("Insertion: {:#?}", tbl_avatar.get(&1));

    // create userAvatar: user -> avatar
    tbl_user_avatar_insert(
        &mut tbl_user_avatar,
        1,
        tbl_user.get(&1).unwrap().id,
        tbl_avatar.get(&2).unwrap().id,
        20
    );
    // println!("{:#?}", tbl_user_avatar.get(&1));
}

// TODO: make more modular fn's for saving values into the hashMap and others.

// Access HasMap data: safe mode
// if let Some(avatar) = tbl_avatar.get(&1) {
//     println!("Avatar name: {}", avatar.name);
// }
// Access HasMap data: unsafe mode
// println!("Avatar name: {}", tbl_avatar.get(&1).unwrap().name);