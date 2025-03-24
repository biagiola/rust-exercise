use std::collections::HashMap;
use crate::models::{User, Avatar, AvatarType, UserAvatar};

pub struct UserTable {
    data: HashMap<u32, User>
}

impl UserTable {
    pub fn new() -> Self {
        UserTable {
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, id: u32, first_name: String, last_name: String, nick_name: String) {
        let user = User {
            id,
            first_name,
            last_name,
            nick_name
        };
        self.data.insert(id, user);
    }

    pub fn update(&mut self, id: u32, first_name: String, last_name: String, nick_name: String) {
        let user = User {
            id,
            first_name,
            last_name,
            nick_name
        };
        self.data.insert(id, user);
    }

    pub fn get(&self, id: &u32) -> Option<&User> {
        self.data.get(id)
    }
}

pub struct AvatarTable {
    data: HashMap<u32, Avatar>
}

impl AvatarTable {
    pub fn new() -> Self {
        AvatarTable {
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, id: u32, name: String, avatar_type: AvatarType) {
        let avatar = Avatar {
            id,
            name,
            avatar_type
        };
        self.data.insert(id, avatar);
    }

    pub fn get(&self, id: &u32) -> Option<&Avatar> {
        self.data.get(id)
    }
}

pub struct UserAvatarTable {
    data: HashMap<u32, UserAvatar>
}

impl UserAvatarTable {
    pub fn new() -> Self {
        UserAvatarTable {
            data: HashMap::new()
        }
    }

    pub fn insert(&mut self, id: u32, user_id: u32, avatar_id: u32, level: u32) {
        let user_avatar = UserAvatar {
            id,
            user_id,
            avatar_id,
            level
        };
        self.data.insert(id, user_avatar);
    }

    pub fn get(&self, id: &u32) -> Option<&UserAvatar> {
        self.data.get(id)
    }
} 