____________________________________________________________________________________________________________________
fn main(){
    let name : String = String::from("Hello world") ;
    let a_name = &name[..5] ;
    println!("NAme is {:?}" ,a_name) ;
}
line 3 -> what is the explicit type of the veriable a_name ? Its not String or &String ...it's become now &str because it's slice type ; 
if i write code like -> let a_name  : &String = &name[..5] it will be worng .......then if i want to use the type &String i have to write in this way 
let a_name : &String = &name[..5].to_string() ; 

now see the diffirence according to this new 2 codes 
  ----> 
fn get_first_word(name: &String) -> String {
    let bytes = name.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return name[..i].to_string(); //converting slice to string
        }
    }
    name[..].to_string() // returning full string as same way 
}

but this code perfectly aligned -----> 
fn get_first_word(name: &String) -> &str {
    let bytes = name.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &name[..i]; // no String converstion needed ....returning direct &str
        }
    }
    &name[..]    //same way , cool !
}

______________________________________________________________________________________________
//not the same code at all ;
fn main(){
    let name : String = String::from("Hello world") ;
    let first_word : &String = &get_first_word(&name) ;
    println!("First word is {:?}" , first_word) ; 
}
fn get_first_word(word : &String)-> String{
    let byte : &[u8] = word.as_bytes() ;
    for(i , &item) in byte.iter().enumerate(){
        if item == b' '{
            return word[..i].to_string() ;
        }
    }
    return word[..].to_string() ;
}
