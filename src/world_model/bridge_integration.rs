use super::coordinator::{WorldModelCoordinator, QueryDecision};
use super::state::{WorldState, AgentAction, StateTransition};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeMessage {
    ExecuteAction {
        action: AgentAction,
        current_state: WorldState,
    },
    StateUpdate {
        new_state: WorldState,
        success: bool,
        reward: f32,
    },
    PredictNextState {
        current_state: WorldState,
        action: AgentAction,
    },
    PredictionResult {
        decision: QueryDecision,
    },
}

pub struct WorldModelBridge {
    coordinator: Arc<RwLock<WorldModelCoordinator>>,
    unreal_tx: mpsc::Sender<BridgeMessage>,
    unreal_rx: mpsc::Receiver<BridgeMessage>,
}

impl WorldModelBridge {
    pub fn new(
        coordinator: WorldModelCoordinator,
        channel_buffer_size: usize,
    ) -> (Self, mpsc::Sender<BridgeMessage>, mpsc::Receiver<BridgeMessage>) {
        Self::new_shared(Arc::new(RwLock::new(coordinator)), channel_buffer_size)
    }

    pub fn new_shared(
        coordinator: Arc<RwLock<WorldModelCoordinator>>,
        channel_buffer_size: usize,
    ) -> (Self, mpsc::Sender<BridgeMessage>, mpsc::Receiver<BridgeMessage>) {
        let (unreal_tx, agent_rx) = mpsc::channel(channel_buffer_size);
        let (agent_tx, unreal_rx) = mpsc::channel(channel_buffer_size);

        let bridge = Self {
            coordinator,
            unreal_tx,
            unreal_rx,
        };

        (bridge, agent_tx, agent_rx)
    }

    pub async fn run(mut self) {
        println!("🌉 World Model Bridge iniciado");

        while let Some(message) = self.unreal_rx.recv().await {
            match message {
                BridgeMessage::PredictNextState { current_state, action } => {
                    let mut coord = self.coordinator.write().await;
                    let decision = coord.should_query_unreal(
                        &current_state,
                        &action,
                    ).await;

                    if decision.query_unreal {
                        let _ = self.unreal_tx.send(BridgeMessage::ExecuteAction {
                            action,
                            current_state,
                        }).await;
                    } else {
                        let _ = self.unreal_tx.send(BridgeMessage::PredictionResult {
                            decision,
                        }).await;
                    }
                }

                BridgeMessage::StateUpdate { new_state, success, reward } => {
                    let transition = StateTransition {
                        state_before: new_state.clone(), // Idealmente se almacena el estado anterior
                        state_after: new_state,
                        success,
                        reward,
                    };

                    let mut coord = self.coordinator.write().await;
                    if let Some(discrepancy) = coord.record_transition(transition) {
                        println!(
                            "📊 Discrepancia: {:.2}% - Corrección: {}",
                            discrepancy.overall_discrepancy * 100.0,
                            if discrepancy.requires_correction { "SÍ" } else { "NO" }
                        );
                    }
                }

                _ => {}
            }
        }

        println!("🛑 World Model Bridge detenido");
    }

    pub async fn save_checkpoint(&self, path: &str) -> std::io::Result<()> {
        let coord = self.coordinator.read().await;
        coord.save(path)
    }
}
