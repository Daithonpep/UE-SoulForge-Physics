use crate::domains::chess::ChessWorld;
use shakmaty::Move;

pub struct SenkuChessAnalyzer;

impl SenkuChessAnalyzer {
    pub fn analyze(&self, world: &ChessWorld) -> ChessAnalysis {
        let _vars = world.get_causal_variables();

        // Lógica simple: prioriza material y centro
        let mut best_move = None;
        let mut best_score = f32::NEG_INFINITY;

        for mv in world.get_legal_moves() {
            let mut simulated = world.clone();
            simulated.apply_move(&mv).unwrap();

            let vars_after = simulated.get_causal_variables();
            // Invertimos el puntaje para simular que vemos desde el lado del oponente o que calculamos hacia el futuro,
            // (Shakmaty invierte el turno), pero para simplificar lo asumimos directo si lo corregimos.
            // Nota: En get_causal_variables ya miramos desde la perspectiva del jugador que tiene el turno!
            // Al aplicar el movimiento, es el turno del OTRO. 
            // Así que el puntaje del estado resultante para nosotros será negativo del puntaje de vars_after.
            // Excepto si ajustamos variables. Asumiendo f32 "absoluto" simple:
            let score = -(vars_after["material_balance"] + vars_after["center_control"] * 0.5);

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        ChessAnalysis {
            agent: "Senku".to_string(),
            rationale: format!(
                "Movimiento lógico: {:?}. Margen de mejora (+{:.2}).",
                best_move, best_score
            ),
            suggested_move: best_move,
            confidence: 0.85,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChessAnalysis {
    pub agent: String,
    pub rationale: String,
    pub suggested_move: Option<shakmaty::Move>,
    pub confidence: f32,
}
