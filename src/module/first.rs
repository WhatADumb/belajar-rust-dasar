use crate::information::dev_status;

#[allow(dead_code)]
pub fn say_hello(){
    println!("Hello World everyone from first module");
    println!("{}", dev_status());
}