use daithon_bridge::larynx_engine::gym::PhoneticGym;
use hound;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 --- DAITHON PHONETIC GYM ---");
    let sample_rate = 22050.0;
    let mut gym = PhoneticGym::new(sample_rate);

    // Lista de archivos y los fonemas que representan
    let training_set = vec![
        ("vocales daithon.wav", vec!["a", "e", "i", "o", "u"]),
        ("transiciones vocales daithon.wav", vec!["ae", "io", "ou"]),
    ];

    for (file, phonemes) in training_set {
        let path = format!("voice_training/raw_recordings/{}", file);
        if !std::path::Path::new(&path).exists() { 
            println!("⚠ Archivo no encontrado: {}", file);
            continue; 
        }
        
        println!("📂 Cargando y procesando {}...", file);
        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();
        
        // Carga robusta de samples según el formato del WAV
        let samples: Vec<f64> = match spec.sample_format {
            hound::SampleFormat::Int => {
                let max_val = u32::pow(2, spec.bits_per_sample as u32 - 1) as f64;
                reader.samples::<i32>().map(|s| s.unwrap() as f64 / max_val).collect()
            },
            hound::SampleFormat::Float => {
                reader.samples::<f32>().map(|s| s.unwrap() as f64).collect()
            }
        };

        println!("   Formato detectado: {:?} ({} bits, {} Hz)", spec.sample_format, spec.bits_per_sample, spec.sample_rate);

        // Dividir el audio en trozos iguales por fonema
        let chunk_len = samples.len() / phonemes.len();
        for (i, &ph) in phonemes.iter().enumerate() {
            let start = i * chunk_len;
            let end = (i + 1) * chunk_len;
            gym.train_phoneme(ph.to_string(), &samples[start..end]);
        }
    }

    println!("💾 Guardando resultados en evolved_models/joseph_voice_v1.json...");
    let json = serde_json::to_string_pretty(&gym.trained_phonemes)?;
    std::fs::create_dir_all("voice_training/evolved_models")?;
    std::fs::write("voice_training/evolved_models/joseph_voice_v1.json", json)?;

    println!("✅ ENTRENAMIENTO COMPLETADO. Daithon ya tiene tu huella vocal.");
    Ok(())
}
