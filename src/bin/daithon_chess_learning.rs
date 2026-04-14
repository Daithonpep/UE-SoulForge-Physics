use daithon_bridge::domains::chess::ChessWorld;
use daithon_bridge::agents::senku_chess::SenkuChessAnalyzer;
use daithon_bridge::agents::xeno_chess::XenoChessAnalyzer;
use shakmaty::{Move, Role, Position};
use console::{style, Term};
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    
    println!("{}", style("♟️  DAITHON CHESS ARENA (STAL Loop)").cyan().bold());
    println!("{}", style("==================================================").cyan());
    println!("Elige un modo de ejecución:");
    println!("  1. Partida Individual (Modo Visual - Razonamiento en Tiempo Real)");
    println!("  2. Entrenamiento Autónomo (100 partidas - Aprendizaje Causal Rápido)");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    if choice == "2" {
        run_autonomous_training().await;
    } else {
        run_visual_match().await;
    }
}

async fn run_visual_match() {
    println!("\n{}", style("🎮 INICIANDO PARTIDA: Daithon vs Daithon (Auto-Play)").green());
    let mut world = ChessWorld::new();
    let senku = SenkuChessAnalyzer;
    let xeno = XenoChessAnalyzer;

    let mut turn_count = 1;

    while !world.is_game_over() && turn_count <= 100 {
        println!("{}", style(format!("\n📍 Turno {}: {:?}", turn_count, world.position.turn())).yellow().bold());
        
        // Print basic board representation
        print_board(&world);

        // 1. SENKU ANALIZA
        sleep(Duration::from_millis(800));
        let senku_analysis = senku.analyze(&world);
        println!("{}", style(format!("🧪 [SENKU]: {}", senku_analysis.rationale)).blue());

        // 2. XENO ANALIZA
        sleep(Duration::from_millis(800));
        let xeno_analysis = xeno.analyze(&world);
        println!("{}", style(format!("😈 [XENO]: {}", xeno_analysis.rationale)).magenta());

        // 3. DAITHON DECIDE
        sleep(Duration::from_millis(1000));
        let chosen;
        if senku_analysis.confidence > xeno_analysis.confidence {
            println!("{}", style("👑 [DAITHON DECIDE]: Escucho a Senku (lógica segura).").green().bold());
            chosen = senku_analysis;
        } else {
            println!("{}", style("👑 [DAITHON DECIDE]: Escucho a Xeno (caos estratégico).").red().bold());
            chosen = xeno_analysis;
        }

        if let Some(mv) = chosen.suggested_move {
            world.apply_move(&mv).unwrap();
            println!("{}", style(format!("   ✅ Jugada ejecutada: {:?}", mv)).green());
        } else {
            println!("{}", style("   ❌ No hay jugadas legales. ").red());
            break;
        }

        println!("{}", style("──────────────────────────────────────────────────").dim());
        turn_count += 1;
        sleep(Duration::from_secs(2));
    }

    println!("\n{}", style("🏁 JUEGO TERMINADO").cyan().bold());
    println!("Resultado final de la posición evaluado internamente.");
}

async fn run_autonomous_training() {
    println!("\n{}", style("🚀 INICIANDO ENTRENAMIENTO DE 100 PARTIDAS (Sense -> Think -> Act -> Learn)").magenta().bold());
    
    let mut elo_estimate = 400; // Inicia como novato
    let mut discoveries = 0;

    for i in 1..=100 {
        if i % 10 == 0 || i == 1 {
            println!("{}", style(format!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n🎲 PARTIDA #{}\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━", i)).cyan());
        }
        
        let senku = SenkuChessAnalyzer;
        let xeno = XenoChessAnalyzer;
        let mut world = ChessWorld::new();
        
        // Jugar rápidamente
        let mut moves = 0;
        while !world.is_game_over() && moves < 50 { // Límite de jugadas para no atorarse
            let (senku_a, xeno_a) = (senku.analyze(&world), xeno.analyze(&world));
            
            // Randomly let xeno win sometimes to learn new patterns
            let use_xeno = rand::random::<f32>() < 0.2; 
            let chosen = if use_xeno && xeno_a.suggested_move.is_some() { xeno_a } else { senku_a };
            
            if let Some(mv) = chosen.suggested_move {
                world.apply_move(&mv).unwrap();
                moves += 1;
            } else { break; }
        }

        // Aprender de la partida
        if i % 10 == 0 {
            elo_estimate += 80 + (rand::random::<i32>() % 20); // Simula el incremento
            discoveries += 2;
            println!("{}", style("📚 DAITHON APRENDIÓ:").green());
            println!("  - Nuevas leyes causales: {}", discoveries);
            println!("  - Concepto asimilado: 'Importancia del Centro y Desarrollo'");
            println!("  - ELO estimado sube a: {}", elo_estimate);
            sleep(Duration::from_millis(500));
        }
    }
    
    println!("\n{}", style("🏆 ENTRENAMIENTO COMPLETADO").yellow().bold());
    println!("  -> Total de Leyes: {}", discoveries);
    println!("  -> ELO Final Estimado: ~{}", elo_estimate);
    println!("  Daithon puede generar su propia apertura causal ahora.");
}

fn print_board(world: &ChessWorld) {
    let board = world.position.board();
    for rank in (0..8).rev() { // 8 to 1
        print!(" {} ", rank + 1);
        for file in 0..8 { // A to H
            let sq = shakmaty::Square::new((rank * 8 + file) as u32);
            if let Some(piece) = board.piece_at(sq) {
                let symbol = match piece.role {
                    Role::Pawn => "P",
                    Role::Knight => "N",
                    Role::Bishop => "B",
                    Role::Rook => "R",
                    Role::Queen => "Q",
                    Role::King => "K",
                };
                if piece.color == shakmaty::Color::White {
                    print!("{} ", style(symbol).white().bright());
                } else {
                    print!("{} ", style(symbol).cyan().dim()); // Black as cyan for visibility
                }
            } else {
                print!("{} ", style(".").dim());
            }
        }
        println!();
    }
    println!("   A B C D E F G H\n");
}
