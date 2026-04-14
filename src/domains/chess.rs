use shakmaty::{Chess, Position, Move, Role, Square};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ChessWorld {
    pub position: Chess,
    pub move_history: Vec<Move>,
}

impl ChessWorld {
    pub fn new() -> Self {
        Self {
            position: Chess::default(), // Posición inicial
            move_history: Vec::new(),
        }
    }

    /// Extrae variables para el análisis causal
    pub fn get_causal_variables(&self) -> HashMap<String, f32> {
        let mut vars = HashMap::new();

        // 1. Balance Material (Puntos)
        vars.insert("material_balance".to_string(), self.calculate_material());

        // 2. Control del Centro
        vars.insert("center_control".to_string(), self.calculate_center_control());

        // 3. Seguridad del Rey
        vars.insert("king_safety".to_string(), self.calculate_king_safety());

        // 4. Movilidad (Número de jugadas legales)
        vars.insert("mobility".to_string(), self.position.legal_moves().len() as f32);

        // 5. Desarrollo de Piezas
        vars.insert("development".to_string(), self.calculate_development());

        vars
    }

    fn calculate_material(&self) -> f32 {
        let mut white = 0;
        let mut black = 0;

        for square in shakmaty::Square::ALL {
            if let Some(piece) = self.position.board().piece_at(square) {
                let value = match piece.role {
                    Role::Pawn => 1,
                    Role::Knight | Role::Bishop => 3,
                    Role::Rook => 5,
                    Role::Queen => 9,
                    Role::King => 0,
                };

                if piece.color == shakmaty::Color::White {
                    white += value;
                } else {
                    black += value;
                }
            }
        }

        if self.position.turn() == shakmaty::Color::White {
            (white - black) as f32
        } else {
            (black - white) as f32
        }
    }

    fn calculate_center_control(&self) -> f32 {
        // Casillas centrales: e4, e5, d4, d5
        let center = [
            Square::E4, Square::E5, 
            Square::D4, Square::D5
        ];

        let mut control = 0.0;
        let turn = self.position.turn();

        for sq in center {
            if let Some(piece) = self.position.board().piece_at(sq) {
                control += if piece.color == turn { 1.0 } else { -1.0 };
            }
        }

        control
    }

    fn calculate_king_safety(&self) -> f32 {
        // Simplificado
        0.0 
    }

    fn calculate_development(&self) -> f32 {
        // Cuenta piezas menores desarrolladas (no en fila inicial)
        let mut developed = 0;
        let turn = self.position.turn();

        for square in shakmaty::Square::ALL {
            if let Some(piece) = self.position.board().piece_at(square) {
                if piece.color == turn 
                    && (piece.role == Role::Knight || piece.role == Role::Bishop)
                    && square.rank() != shakmaty::Rank::First
                    && square.rank() != shakmaty::Rank::Eighth
                {
                    developed += 1;
                }
            }
        }

        developed as f32
    }

    pub fn apply_move(&mut self, mv: &Move) -> Result<(), String> {
        if self.position.is_legal(*mv) {
            self.position = self.position.clone().play(*mv).map_err(|e| format!("{:?}", e))?;
            self.move_history.push(mv.clone());
            Ok(())
        } else {
            Err("Movimiento ilegal".to_string())
        }
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.position.legal_moves().into_iter().collect()
    }

    pub fn is_game_over(&self) -> bool {
        self.position.is_game_over()
    }
}
