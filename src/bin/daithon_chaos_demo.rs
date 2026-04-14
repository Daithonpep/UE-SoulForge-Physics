#[tokio::main]
async fn main() {
    println!("🧪 [DAITHON CHAOS DEMO] Forzando el Factor Caos y la Inteligencia Contextual...");
    
    let mut daithon = daithon_bridge::contextus::engine::DaithonContext::new();

    // Simulamos un input que DAITHON detecta como casual/humorístico
    // pero sobre un tema que el buscador asocia erróneamente con algo formal (como Mato Jajalo)
    println!("\n--- TEST: El 'Mato Jajalo' Paradox (Registro Playful + Caos) ---");
    let res = daithon.process_user_input("jaja Daithon, que loco sos xd").await;
    
    println!("\n--- RESPUESTA DE DAITHON ---\n");
    println!("{}", res);

    println!("\n------------------------------------------------\n");
    println!("🧪 [DAITHON CHAOS DEMO] Caso 2: Tema serio con ruptura de hielo interna...");
    
    // Para este caso, simularemos que el 5% de caos ocurrió en un tema grave.
    let res_serious = daithon.process_user_input("Háblame de la singularidad gravitacional.").await;
    println!("\n--- RESPUESTA DE DAITHON (CON POSIBLE CAOS) ---\n");
    println!("{}", res_serious);
}
