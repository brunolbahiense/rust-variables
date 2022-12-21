// ./main para executar o código
// O const é imutavel e pode ser setado fora do escopo
// o static funciona ao const porém pode ser mutado, apesar de dever ser inciada apenas com valores constantes
// o mut da ao static a mutabilidade, existe o unsafe para fazer o bypass das limitações que o rust tras, porém não é recomendado

const PI:f32 = 3.14;
static VARIAVEL_GLOBAL:u8 = 1;
static mut GLOBAL:u8 = 2;

fn main(){
    println!("pi = {}", PI);
    println!("variavel_global = {}", VARIAVEL_GLOBAL);
    unsafe{
        println!("GLOBAL = {}", GLOBAL)
    }
}

