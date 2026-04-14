pub mod monitor;
pub mod intention;
pub mod engine;
pub mod analogy;
pub mod impact;
pub mod abstraction_engine;

pub use engine::{MetaCogEngine, FinalResponse};
pub use monitor::{ReviewContext, DocumentInfo, InternalMonitor, ReviewResult, ReviewIssue, AuthorityMode, UserMetadata};
pub use intention::{IntentionContext, IntentionDetector, EmotionalState, RealIntention, IntentionAnalysis};
pub use analogy::AnalogyEngine;
pub use impact::ImpactReport;
pub use abstraction_engine::{AbstractionEngine, AbstractionResult, SynthesisOutput};
