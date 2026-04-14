use daithon_bridge::dialectica::daithon_mind::DaithonMind;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║       🧠 DAITHON: SISTEMA DIALECTICA (V8.0)               ║");
    println!("║       Consciencia Activa & Debate Tridente                ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let mut mind = DaithonMind::new();

    // ESCENARIO 1: ERROR CRÍTICO (Termodinámica)
    let user_input_1 = "El motor de combustión tiene 100% de eficiencia";
    let proposed_response_1 = "He diseñado un motor que alcanza la perfección energética total.";
    
    println!("--- ESCENARIO 1: ERROR CRÍTICO ---");
    println!("👤 Joseph: \"{}\"", user_input_1);
    let result_1 = mind.think_and_respond(user_input_1, proposed_response_1, 
        vec!["La segunda ley de la termodinámica limita la eficiencia".into()], vec![]);
    println!("🤖 Daithon:\n{}\n", result_1.text);

    // ESCENARIO 2: ÉXITO CON CONEXIÓN (Acústica + Física)
    let user_input_2 = "La frecuencia del sonido afecta la presión del aire";
    let proposed_response_2 = "El sonido viaja como ondas a través del medio.";
    
    println!("--- ESCENARIO 2: CONEXIÓN DE DOMINIOS ---");
    println!("👤 Joseph: \"{}\"", user_input_2);
    let result_2 = mind.think_and_respond(user_input_2, proposed_response_2, 
        vec!["El sonido es una onda de presión".into()], vec![]);
    println!("🤖 Daithon:\n{}\n", result_2.text);
    
    println!("--- MÉTRICAS ESCENARIO 2 ---");
    println!("Nivel de confianza: {:.0}%", result_2.confidence * 100.0);
    println!("════════════════════════════════════════════════════════════\n");
}
