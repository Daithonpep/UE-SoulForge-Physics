use crate::domains::chess::ChessWorld;
use crate::agents::senku_chess::ChessAnalysis;

pub struct XenoChessAnalyzer;

impl XenoChessAnalyzer {
    pub fn analyze(&self, world: &ChessWorld) -> ChessAnalysis {
        let mut best_chaos_move = None;
        let mut max_chaos = f32::NEG_INFINITY;

        for mv in world.get_legal_moves() {
            // Xeno busca jugadas que DESESTABILICEN
            let chaos_score = self.calculate_chaos(&world, &mv);

            if chaos_score > max_chaos {
                max_chaos = chaos_score;
                best_chaos_move = Some(mv);
            }
        }

        ChessAnalysis {
            agent: "Xeno".to_string(),
            rationale: format!(
                "MOVIMIENTO CAÓTICO: {:?}. Lyapunov λ={:.2}. Rompe su estructura.",
                best_chaos_move, max_chaos
            ),
            suggested_move: best_chaos_move,
            confidence: 0.70, // Menos confianza, más riesgo
        }
    }

    fn calculate_chaos(&self, _world: &ChessWorld, mv: &shakmaty::Move) -> f32 {
        // Heurística de caos:
        // - Sacrificios (Capturas inusuales)
        // - Movimientos de peón al azar

        let mut chaos = 0.0;

        if mv.is_capture() {
            chaos += 5.0;
        }
        
        // Bonus aleatorio para simular 'Atractores extraños' en profundidad no precalculada
        chaos += rand::random::<f32>();

        chaos
    }
}
