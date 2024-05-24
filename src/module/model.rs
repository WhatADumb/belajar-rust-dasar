#[allow(dead_code)]
pub struct User{
    pub name: String,
    pub level: u32,
    pub role: String
}


impl User {
    #[allow(dead_code)]
    pub fn greeting(&self, name: &str){
        println!("Hello {}, my name is {}", name, self.name);
    }
}