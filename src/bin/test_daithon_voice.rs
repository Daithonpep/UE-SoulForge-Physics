use daithon_bridge::voice::{VoxisPhysicalService, EmotionContext, VoiceGenerator};
use hound;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎭 --- EL DEBUT DE DAITHON ---");
    println!("(Usando ADN vocal de Joseph + Estilo propio de Daithon)");

    let sample_rate = 22050.0;
    let mut service = VoxisPhysicalService::new(sample_rate);

    // Frases para el estreno
    let frases = vec![
        ("eee eeee iiii eeee", EmotionContext::Neutral, "debut_balbuceo.wav"),
        ("ei ei eeeaaa eeeeee", EmotionContext::Excited, "debut_emocion.wav"),
        ("uuuuuaaaa", EmotionContext::Neutral, "test_barrido_ua.wav"),
    ];

    for (text, emotion, filename) in frases {
        println!("🗣️ Daithon está articulando: '{}'...", text);
        let streams = service.generate_speech_stream(text.to_string(), emotion);
        
        let mut final_samples = Vec::new();
        for stream in streams {
            final_samples.extend(stream.samples);
        }

        let path = format!("voice_training/evolved_models/{}", filename);
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: sample_rate as u32,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(&path, spec)?;
        for sample in final_samples {
            writer.write_sample((sample * 32767.0) as i16)?;
        }
        writer.finalize()?;
        println!("✅ Audio generado: {}", path);
    }

    println!("\n🚀 EL APARATO FONADOR ESTÁ VIVO.");
    Ok(())
}
