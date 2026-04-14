fn main() {
    let input = "Daithon, genera un reporte a fondo sobre la Teoría de Cuerdas";
    let words: Vec<&str> = input.split(|c: char| !c.is_alphabetic()).filter(|s| !s.is_empty()).collect();
    
    println!("--- ANALIZADOR DE DAITHON ---");
    for w in words {
        let low = w.to_lowercase();
        println!("Palabra: '{}' (Len: {})", low, low.len());
    }
}
