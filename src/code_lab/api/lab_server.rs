use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RunCodeRequest {
    pub code: String,
    pub language: String,
}

#[derive(Deserialize)]
pub struct StartLabRequest {
    pub exercise_id: Option<String>,
    pub auto_mode: bool,
}

/// API del Code Lab
pub async fn run_code(
    req: web::Json<RunCodeRequest>,
) -> HttpResponse {
    let sandbox_py = crate::code_lab::sandbox::python_sandbox::PythonSandbox::new();
    let sandbox_rs = crate::code_lab::sandbox::rust_sandbox::RustSandbox::new();

    let result = match req.language.as_str() {
        "python" | "py" => sandbox_py.execute(&req.code),
        "rust" | "rs" => sandbox_rs.execute(&req.code),
        _ => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Lenguaje no soportado. Usa 'python' o 'rust'"
        })),
    };

    HttpResponse::Ok().json(result)
}

pub async fn start_lab(
    req: web::Json<StartLabRequest>,
) -> HttpResponse {
    let mut lab = crate::code_lab::auto_iteration::iteration_loop::AutoIterationLoop::new();

    if req.auto_mode {
        let sessions = lab.run_auto_session();
        HttpResponse::Ok().json(serde_json::json!({
            "sessions": sessions.len(),
            "completed": sessions.iter().filter(|s| s.success).count(),
        }))
    } else {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "Lab ready",
            "next_exercise": lab.exercise_engine.get_progress_report(),
        }))
    }
}

pub async fn get_exercises() -> HttpResponse {
    let engine = crate::code_lab::curriculum::exercise_engine::ExerciseEngine::new();
    HttpResponse::Ok().json(serde_json::json!({
        "progress": engine.get_progress_report(),
    }))
}

pub async fn start_code_lab_server() -> std::io::Result<()> {
    println!("[CODE LAB] API iniciada en http://localhost:8091");

    HttpServer::new(|| {
        App::new()
            .route("/api/lab/run", web::post().to(run_code))
            .route("/api/lab/start", web::post().to(start_lab))
            .route("/api/lab/exercises", web::get().to(get_exercises))
    })
    .bind("127.0.0.1:8091")?
    .run()
    .await
}
