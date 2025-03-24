use std::collections::HashMap;
mod models;
mod tables;
use models::{User, Avatar, AvatarType, UserAvatar};
use tables::{UserTable, AvatarTable, UserAvatarTable};

fn main() {
    // CREATE TABLES
    let mut user_table = UserTable::new();
    let mut avatar_table = AvatarTable::new();
    let mut user_avatar_table = UserAvatarTable::new();

    // create user
    user_table.insert(
        1,
        String::from("David"),
        String::from("Biagiola"),
        String::from("Deivi")
    );
    
    // update user
    user_table.update(
        1,
        String::from("David edited"),
        String::from("Biagiola edited"),
        String::from("Deivi edited")
    );
    println!("{:#?}", user_table.get(&1));

    // create avatars
    avatar_table.insert(
        1,
        String::from("Gandalf"),
        AvatarType::Magician
    );
    avatar_table.insert(
        2,
        String::from("Frodo"),
        AvatarType::Hobbit
    );

    // create user avatar
    user_avatar_table.insert(
        1,
        user_table.get(&1).unwrap().id,
        avatar_table.get(&2).unwrap().id,
        20
    );
}

// TODO: make CRUD operations for the tables

// Access HasMap data: safe mode
// if let Some(avatar) = tbl_avatar.get(&1) {
//     println!("Avatar name: {}", avatar.name);
// }
// Access HasMap data: unsafe mode
// println!("Avatar name: {}", tbl_avatar.get(&1).unwrap().name);