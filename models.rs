#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub nick_name: String
}

#[derive(Debug)]
pub struct Avatar {
    pub id: u32,
    pub name: String,
    pub avatar_type: AvatarType,
}

#[derive(Debug)]
pub enum AvatarType {
    Magician,
    Hobbit,
    Elf,
    Dwarf,
    Ork
}

#[derive(Debug)]
pub struct UserAvatar {
    pub id: u32,
    pub user_id: u32,
    pub avatar_id: u32,
    pub level: u32
}

// This will be the struct if we want to keep a reference
// &str insted of cloning using the String heap data type
// pub struct User<'a> {
//     pub id: u32,
//     pub first_name: &'a str,
//     pub last_name: &'a str,
//     pub nick_name: &'a str,
// }
// this is something about lifetimes to discuss later