fn main() {
    println!("cargo::rerun-if-env-changed=SPEAKORO_MODEL_FILE"); // maybe behind feature?
    if env!("SPEAKORO_MODEL_FILE").is_empty() {
        println!("cargo::error=SPEAKORO_MODEL_FILE env variable must not be empty.");
    }

    #[cfg(feature = "download-data")]
    {
        println!("cargo::warning=Feature 'download-data' is enabled.");
        // TODO: This needs to be split up into functions for checking if data is present, and for downloading.
        // Then, downloading can be hidden behind a feature flag, and a better compile error can be thrown if
        // data is not present, but `download-data` feature is disabled. See phonemoro build.rs for that.
        download::download_data_if_necessary().unwrap()

    }
}

#[cfg(feature = "download-data")]
mod download {
    use anyhow::{Context, Result};
    use std::{
        fs::File,
        io::BufWriter,
        path::{Path, PathBuf},
    };

    static VOICES: &'static [&'static str; 9] = &[
        "af_heart",
        "af_bella",
        "af_nicole",
        "af_aoede",
        "bf_emma",
        "bf_isabella",
        "am_adam",
        "bm_daniel",
        "am_fenrir",
    ];

    /// Download all data needed for building speakoro, if necessary.
    pub fn download_data_if_necessary() -> Result<()> {
        // Config
        let base_url = "https://hf.co/onnx-community/Kokoro-82M-v1.0-ONNX/resolve/main";

        let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
        let onnx_dir = data_dir.join("onnx");
        let voices_dir = data_dir.join("voices");

        // Create dirs
        if !data_dir.try_exists()? {
            std::fs::create_dir(&data_dir)?;
        }
        if !onnx_dir.try_exists()? {
            std::fs::create_dir(&onnx_dir)?;
        }
        if !voices_dir.try_exists()? {
            std::fs::create_dir(&voices_dir)?;
        }

        // Create download tasks, if not already exists:
        // => voices
        let mut work_queue: Vec<(String, PathBuf)> = VOICES
            .iter()
            .filter_map(|&v| {
                let url = format!("{base_url}/voices/{v}.bin?download=true");
                let target_path = voices_dir.join(format!("{v}.bin"));

                if !target_path.exists() {
                    Some((url, target_path))
                } else {
                    None
                }
            })
            .collect();

        // => model (defined in .cargo/config.toml)
        let model_file = env!("SPEAKORO_MODEL_FILE");

        let url = format!("{base_url}/onnx/{model_file}?download=true");
        let target_path = onnx_dir.join(model_file);

        if !target_path.exists() {
            work_queue.push((url, target_path));
        }

        if !work_queue.is_empty() {
            work_queue
                .iter()
                .map(|(url, target_path)| download(url, target_path))
                .collect::<Result<()>>()?;
        }

        Ok(())
    }

    /// Download file from url, and stores it in target directory. Does not overwrite file, if it already exists.
    fn download<S: AsRef<str>, P: AsRef<Path>>(url: S, target_path: P) -> Result<()> {
        let url = url.as_ref();
        let target_path = target_path.as_ref();

        // check if file already exists (it shouldn't, as it is already checked before, but safe is safe)
        if target_path.try_exists()? {
            anyhow::bail!(format!("Provided path {:?} already exists", target_path))
        }

        // download file
        let resp = ureq::get(url)
            .call()
            .with_context(|| "Download request failed.")?;

        if resp.status() != ureq::http::StatusCode::OK {
            return Err(anyhow::anyhow!(
                "Request failed with status: {}",
                resp.status()
            ));
        }

        // write to filesystem
        let file = File::create(target_path)?;

        let mut body = resp.into_body();
        let mut reader = body.as_reader();
        let mut writer = BufWriter::new(file);

        let _ = std::io::copy(&mut reader, &mut writer)?;

        Ok(())
    }
}
