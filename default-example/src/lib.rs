use derive_new::new;
use rand::Rng;

#[derive(Debug)]
pub enum Role {
    Guest,
    Viewer,
    Creator,
    Admin,
}

#[derive(Debug)]
pub struct User {
    id: u32,
    pub username: String,
    pub role: Role,
}

impl User {
    pub fn new(username: String) -> Result<Self, String> {
        if username == "testuser123" {
            return Err("username already exists!".to_string());
        }
        Ok(Self {
            id: rand::thread_rng().gen_range(0..9999999),
            username,
            role: Role::Creator,
        })
    }
}

impl Default for User {
    fn default() -> Self {
        let id = rand::thread_rng().gen_range(0..9999999);
        Self {
            id,
            username: format!("guest{id}"),
            role: Role::Guest,
        }
    }
}

// all of Post's field implements Default, so we can derive Default
#[derive(Debug, Default, new)]
pub struct Post {
    content: String,

    // when calling the new function, tags should be set to this value
    #[new(value = "vec![\"rusty\".to_string()]")]
    tags: Vec<String>,

    // when calling the new function, likes should be set to this value
    #[new(default)]
    likes: u32,
}
