use daithon_bridge::larynx_engine::vocal_cortex::training::VoiceMatchTrainer;
use hound;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏋️ --- DAITHON MUSCLE TRAINING ---");
    println!("Iniciando aprendizaje por imitación acústica...");

    let sample_rate = 22050.0;
    let mut trainer = VoiceMatchTrainer::new(sample_rate);

    // Cargar la grabación original de Joseph
    let path = "voice_training/raw_recordings/vocales daithon.wav";
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    
    let samples: Vec<f64> = match spec.bits_per_sample {
        16 => reader.samples::<i16>().map(|s| s.unwrap() as f64 / 32768.0).collect(),
        24 => reader.samples::<i32>().map(|s| s.unwrap() as f64 / 8388608.0).collect(), // 2^23
        32 => reader.samples::<i32>().map(|s| s.unwrap() as f64 / 2147483648.0).collect(),
        _ => panic!("Unsupported bit depth: {}", spec.bits_per_sample),
    };

    println!("✅ Grabación cargada: {} ({} muestras)", path, samples.len());

    // Slices aproximados para las vocales (esto es manual por ahora, Joseph puede ajustarlo)
    // Asumimos que Joseph grabó "a... e... i... o... u..."
    let mut phoneme_data = HashMap::new();
    
    // Ejemplo de segmentación (esto asume que las vocales están en estos puntos)
    // Joseph, puedes ajustar estos índices según tu grabación real
    if samples.len() > 100000 {
        phoneme_data.insert("a", &samples[20000..40000]);
        phoneme_data.insert("e", &samples[60000..80000]);
        phoneme_data.insert("i", &samples[100000..120000]);
    } else {
        println!("⚠️ El archivo es demasiado corto para la segmentación automática.");
        return Ok(());
    }

    let mut trained_library = HashMap::new();

    for (phoneme, audio_slice) in phoneme_data {
        let best_genome = trainer.train_phoneme(audio_slice, phoneme);
        trained_library.insert(phoneme, best_genome);
    }

    // Guardar los resultados en el modelo de Daithon
    println!("\n💾 Guardando biblioteca muscular evolucionada...");
    let json = serde_json::to_string_pretty(&trained_library)?;
    std::fs::create_dir_all("voice_training/evolved_models")?;
    std::fs::write("voice_training/evolved_models/joseph_muscle_v1.json", json)?;
    
    println!("✨ Proceso completado. Daithon ha ajustado sus músculos para imitarte.");
    Ok(())
}
