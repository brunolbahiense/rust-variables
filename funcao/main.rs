// a seta -> indica qual o valor de retorno de uma função

fn soma(a:i32, b:i32) -> i32{
    println!("a + b = {}", a + b);
    return a + b
}

fn main(){
    println!("{}",soma(3,4));   
}