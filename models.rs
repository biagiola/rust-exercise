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