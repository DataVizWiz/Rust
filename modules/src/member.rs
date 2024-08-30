pub struct Member {
    pub name: String,
    pub member_id: u32,
}

impl Member {
    pub fn new(name: &str, member_id: u32) -> Member {
        Member {
            name: name.to_string(),
            member_id
        }   
    }
}