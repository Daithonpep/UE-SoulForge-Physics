use daithon_bridge::thalamus::dispatcher;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let prompt = if args.len() > 1 {
        &args[1]
    } else {
        "hola"
    };

    // Lanzamos el proceso de pensamiento convergente
    let response = dispatcher::process_thought(prompt).await;
    
    // Imprimimos el resultado final
    println!("--- FINAL_RESPONSE ---");
    println!("{}", response);
}
