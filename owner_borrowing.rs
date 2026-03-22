______________________________________
fn main(){
    let s1 = String::from("Hello world") ;
    let _s2 = s1.clone();
    println!("S1 is no longer valid right now :{:?}" , s1) ;
}
________________________________________
fn main(){
    let mut name : String = String::from("NOOB") ;
    let full_name : String = get_it(&mut name) ;
    println!("Full name is :{:?}" , full_name) ;
}
fn get_it(name : &mut String)-> String{
    name.push_str(" Why") ;
    return name.clone() ;
}
_______________________
fn main(){
    let value : &str = "name" ;
    let byte : &[u8] = value.as_bytes() ; //bytes are always &[u8] 
    println!("Byte is :{:?}" , byte) ;
} 
