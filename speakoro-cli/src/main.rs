use std::{path::Path, str::FromStr};

use anyhow::Result;
use clap::{Arg, Command, crate_version};
use phonemoro::en::phonemizer::EnPhonemizer;
use speakoro::{Kokoro, KokoroVoice};
use strum::VariantNames;

fn main() -> Result<()> {
    let matches = Command::new("speakoro")
        .version(concat!(crate_version!(), ", uses kokoro model '", env!("SPEAKORO_MODEL_FILE"), "'"))
        .arg(
            Arg::new("text")
                .index(1)
                .help("Pass the text that should be converted to speech. If the flag --phonemes is set, this will be interpreted as raw phonemes.")
                .required(true),
        )
        .arg(
            Arg::new("voice")
                .short('v')
                .long("voice")
                // .value_parser(KokoroVoice::from_str)
                .value_parser(KokoroVoice::VARIANTS.to_owned())
                .default_value("af_bella")
                .help("Set which voice should be used to generate audio."),
        )
        .arg(
            Arg::new("phonemes")
                .short('p')
                .long("phonemes")
                .help("If set, the passed text will be interpreted as phonemes.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("out")
                .short('o')
                .long("out")
                .default_value("audio.wav")
                .help("Set filepath to where the audio will be written to. Note that the output format is WAV."),
        )
        .get_matches();

    // CLI arguments
    let text = matches.get_one::<String>("text").unwrap();
    let voice = KokoroVoice::from_str(matches.get_one::<String>("voice").unwrap()).unwrap();

    let is_phonemes = matches.get_flag("phonemes");
    let out = Path::new(matches.get_one::<String>("out").unwrap());

    // inference
    let text = if !is_phonemes {
        let phonemizer = EnPhonemizer::new()?;

        phonemizer.phonemize(text)?.replace("A", "eɪ") // Unsure why needed, but A isn't pronounced like eI, but lika ah
    } else {
        text.to_owned()
    };

    let kokoro = Kokoro::new()?;
    let audio = kokoro.phonemes2audio(&text, voice, 1f32)?;
    speakoro::utils::write_to_wav(audio, out)?;

    Ok(())
}
