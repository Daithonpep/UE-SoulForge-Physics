#[tokio::main]
async fn main() {
    println!("🧪 [DAITHON EXPERIMENT] Contextual Intelligence & Registry...");
    
    let mut daithon = daithon_bridge::contextus::engine::DaithonContext::new();

    // Caso 1: Gravedad Alta + Técnico
    println!("\n--- TEST 1: Técnico / Gravedad Alta ---");
    let res1 = daithon.process_user_input("Explícame los agujeros negros a fondo.").await;
    println!("{}", res1);

    // Caso 2: Casual + Humor + Meta
    println!("\n--- TEST 2: Casual / Humor / Meta ---");
    let res2 = daithon.process_user_input("jaja Daithon, que loco sos xd").await;
    println!("{}", res2);
}
