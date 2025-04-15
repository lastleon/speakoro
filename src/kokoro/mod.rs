use anyhow::{Context, Result};
use ndarray::{Array1, Array2};
use ort::execution_providers;
use ort::session::Session;
use speakoro_macros::associate_static_data;
use strum::{EnumString, VariantNames};
use tokenizer::KokoroTokenizer;

mod tokenizer;

/// This macro includes a binary file and transmutes it to the requested type with correct alignment. Note that
/// safety still needs to be guaranteed by the user.
///
/// Source of the trick: https://jack.wrenn.fyi/blog/include-transmute/
macro_rules! include_transmute {
    ($file:expr) => {
        &core::mem::transmute(*include_bytes!($file))
    };
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
/// All currently supported voices for Kokoro, which is a selection of voices I liked.
pub enum KokoroVoice {
    // Female - American
    AF_HEART,
    AF_BELLA,
    AF_NICOLE,
    AF_AOEDE,
    // Female - British
    BF_EMMA,
    BF_ISABELLA,
    // Male - American
    AM_ADAM,
    AM_FENRIR,
    // Male - British
    BM_DANIEL,
}

associate_static_data!(
    type Enum = KokoroVoice;
    type Data = &'static [f32; 130_560];

    // KokoroVoice::AF_BELLA   => unsafe { include_transmute!("../../data/voices/af_bella.bin") },
    KokoroVoice::AF_HEART       => unsafe { include_transmute!("../../data/voices/af_heart.bin") },
    KokoroVoice::AF_BELLA       => unsafe { include_transmute!("../../data/voices/af_bella.bin") },
    KokoroVoice::AF_NICOLE      => unsafe { include_transmute!("../../data/voices/af_nicole.bin") },
    KokoroVoice::AF_AOEDE       => unsafe { include_transmute!("../../data/voices/af_aoede.bin") },
    KokoroVoice::BF_EMMA        => unsafe { include_transmute!("../../data/voices/bf_emma.bin") },
    KokoroVoice::BF_ISABELLA    => unsafe { include_transmute!("../../data/voices/bf_isabella.bin") },
    KokoroVoice::AM_ADAM        => unsafe { include_transmute!("../../data/voices/am_adam.bin") },
    KokoroVoice::AM_FENRIR      => unsafe { include_transmute!("../../data/voices/am_fenrir.bin") },
    KokoroVoice::BM_DANIEL      => unsafe { include_transmute!("../../data/voices/bm_daniel.bin") },
);

static KOKORO_STYLE_DIM: usize = 256;
impl KokoroVoice {
    // Better output type: &[f32; KOKORO_STYLE_DIM]
    /// Return style data used by Kokoro for a given token num.
    pub fn style(&self, token_num: usize) -> &[f32] {
        let voice_data = self.static_data();
        let offset = token_num * KOKORO_STYLE_DIM;

        &voice_data[offset..offset + KOKORO_STYLE_DIM]
    }
}

static KOKORO_MODEL_BIN: &[u8] =
    include_bytes!(concat!("../../data/onnx/", env!("SPEAKORO_MODEL_FILE")));

/// Struct representing the Kokoro model.
pub struct Kokoro {
    model: Session,
}

impl Kokoro {
    /// Create a new Kokoro instance.
    pub fn new() -> Result<Self> {
        #[cfg(target_os = "android")]
        let ep = [execution_providers::NNAPIExecutionProvider::default().build()];
        #[cfg(not(target_os = "android"))]
        let ep = [
            execution_providers::OpenVINOExecutionProvider::default().build(),
            execution_providers::CPUExecutionProvider::default().build(),
        ];

        Ok(Kokoro {
            model: Session::builder()?
                .with_execution_providers(ep)?
                .commit_from_memory(KOKORO_MODEL_BIN)
                .with_context(|| "Loading onnx model failed")?, // TODO: Add options like optimization level
        })
    }

    /// Generate audio from phonemes. Output are WAV samples.
    pub fn phonemes2audio(
        &self,
        phonemes: &str,
        voice: KokoroVoice,
        speed: f32,
    ) -> Result<Vec<f32>> {
        // tokenize and prepare input
        let input_ids = {
            let tokens: Vec<i64> = KokoroTokenizer::tokenize(phonemes)
                .into_iter()
                .map(|n| n as i64)
                .collect();

            Array2::from_shape_vec((1, tokens.len()), tokens)?
        };

        // Prepare voice style
        let style =
            Array2::from_shape_vec((1, KOKORO_STYLE_DIM), voice.style(input_ids.len()).to_vec())?;

        // Speed
        let speed = Array1::from_vec(vec![speed]);

        // INFERENCE
        let outputs = self.model.run(ort::inputs! {
            "input_ids" => input_ids,
            "style" => style,
            "speed" => speed
        }?)?;

        let wav_samples = outputs["waveform"].try_extract_tensor::<f32>()?;

        Ok(wav_samples.iter().map(|v| *v).collect())
    }
}
