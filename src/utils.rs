use std::path::Path;

use anyhow::Result;

pub fn write_to_wav<P: AsRef<Path>>(samples: Vec<f32>, path: P) -> Result<()> {
    // metadata
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 24000,
        sample_format: hound::SampleFormat::Float,
        bits_per_sample: 32,
    };

    // write samples
    let mut wav_writer = hound::WavWriter::create(path, spec)?;
    for &sample in samples.iter() {
        wav_writer.write_sample(sample)?;
    }
    wav_writer.finalize().unwrap();

    Ok(())
}
