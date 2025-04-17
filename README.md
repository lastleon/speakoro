# Speakoro

_[Kokoro](https://huggingface.co/hexgrad/Kokoro-82M) library and CLI tool in Rust. Batteries included, just a single binary, no runtime dependencies[^1]._

[^1]: Apart from the usual suspects, such as `libc.so`.

<p align="center">
🚨 <b>This project is currently usable, but pretty barebones and far from finished. 🚨<br> Significant changes can happen.</b>
</p>

# Overview
Use Kokoro in your terminal with an everything-included binary, or easily embed it in your project as a library.

In short, this project embeds a Kokoro onnx file and various Kokoro voice files (currently not all of them), and runs the model using the [ort](https://github.com/pykeio/ort) crate, which is statically linked, meaning that everything is included in the final binary.

The CLI tool additionally uses [Phonemoro](https://github.com/lastleon/phonemoro) as its phonemizer, which also embeds everything it needs, resulting in a fully functioning text-to-speech system within a single binary.

**Features:**

- easy to build and use
- no special runtime dependencies
- single binary with everything embedded
- portable
- doesn't use espeak, so none of the licensing issues
- suitable for mobile use[^2]

[^2]: Depending on the platform, you might need to build the [onnx runtime](https://github.com/microsoft/onnxruntime/) yourself, though. Also yes, this is _kind of_ fast enough to properly run on a phone! :)


# Usage
Since this project is based on Kokoro, a model file (onnx) and the voice files are needed. You can either download them manually, or enable the `download-data` feature and automatically download them during build.
Both ways are described.

## As a Library
1. Add `speakoro` to your project:
- **Easy Way _(Recommended)_**
  - Add `speakoro` directly to your project, with the `download-data` flag enabled:
    ```shell
    $ cargo add --git https://github.com/lastleon/speakoro speakoro -F download-data
    ```

> ⚠️ **Warning:**
>
> This automatically downloads the necessary files from Huggingface. If you don't want that, proceed with **Harder Way**.
- **Harder Way**<br>
Use this only if you're uncomfortable downloading from the internet, or you want to use your own data.
  - Clone this repository to a location outside your project and enter it:
  ```shell
  $ git clone https://github.com/lastleon/speakoro && cd speakoro
  ```
  - Create the onnx model and voice directories:
  ```shell
  $ mkdir -p data/{onnx,voice}
  ```
  - Download the desired model and the english voices from [onnx-community/Kokoro-82M-v1.0-ONNX](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/tree/main), place the model in `data/onnx`, and place the voices in `data/voices`.
  - Back in your project, add `speakoro` as a dependency:
  ```shell
  $ cargo add --path <path-to-the-cloned-speakoro-repo> speakoro
  ```

2. Set the `SPEAKORO_MODEL_FILE` environment variable to choose which model should used (and downloaded, if enabled). You can either:
  - Set it within the `.cargo/config.toml` file in your project:
  ```toml
  [env]
  # See https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/tree/main/onnx for a list of available options. Note that not all models work, you have to test that out.
  # Recommendations: model.onnx, model_fp16.onnx, model_uint8.onnx
  SPEAKORO_MODEL_FILE = "model_uint8.onnx"
  ```
  - Or set the variable during the build:
  ```shell
  $ SPEAKORO_MODEL_FILE=model_uint8.onnx cargo build --release
  ```


3. Use the library like so:
```rust
use speakoro::{Kokoro, KokoroVoice};
use anyhow::Result;

fn main() -> Result<()> {
    let kokoro = Kokoro::new()?;
    let audio = kokoro.phonemes2audio("həlˈO wˈɜɹld", KokoroVoice::AF_BELLA, 1f32)?;
    speakoro::utils::write_to_wav(audio, "audio.wav")?;

    Ok(())
}
```

> 💡 **Note:**
>
> To see an end-to-end example, go to the `speakoro-cli` crate. It utilizes the closely related [Phonemoro](https://github.com/lastleon/phonemoro) project as the phonemizer.


## As a CLI tool
This uses [Phonemoro](https://github.com/lastleon/phonemoro) as the phonemizer.

> ⚠️ **Warning:**
>
> Building the CLI tool requires **downloading** the necessary data for both `speakoro` and `phonemoro`.

Since building `phonemoro` without downloading the data needs some setup in a directory outside this project, meaning it is kind of a involved process, the choice was made to not provide a feature flag for a build without downloading data. Having that flag would only be meaningful if `phonemoro` was also built without downloading data, but for that, it would need to be added as a dependency in a different way. At the end of this section, an offline build is described.

The onnx model and voice files are downloaded from Huggingface, the data `phonemoro` needs is downloaded from the releases page of `phonemoro`.


1. Clone this repository:
```shell
$ git clone https://github.com/lastleon/speakoro
```

2. _(Optional):_ Change the Kokoro model you want to use. For that, follow step 2 of [Usage > As a Library](#as-a-library). By default, `model_uint8.onnx` is used.

3. Build `speakoro-cli`:
```shell
$ cargo build -p speakoro-cli --release
```
<!-- - **Harder Way**
  - Prepare the files like in [Usage > As a Library (Step 1: Harder Way)](#as-a-library).
  - Run:
    ```shell
    $ cargo build -p speakoro-cli --release
    ``` -->

4. Usage:
```shell
$ ./target/release/speakoro-cli --help
Usage: speakoro-cli [OPTIONS] <text>

Arguments:
  <text>  Pass the text that should be converted to speech. If the flag --phonemes is set, this will be interpreted as raw phonemes.

Options:
  -v, --voice <voice>  Set which voice should be used to generate audio. [default: af_bella] [possible values: af_heart, af_bella, af_nicole, af_aoede, bf_emma, bf_isabella, am_adam, am_fenrir, bm_daniel]
  -p, --phonemes       If set, the passed text will be interpreted as phonemes.
  -o, --out <out>      Set filepath to where the audio will be written to. Note that the output format is WAV. [default: audio.wav]
  -h, --help           Print help
  -V, --version        Print version
```

**Offline Build:**

1. Clone this repository and add the necessary data as described in [Usage > As a Library (Harder Way)](#as-a-library)
2. Go to [Phonemoro](https://github.com/lastleon/phonemoro), and follow the offline build instructions (`Usage (lib) > Harder Way`) to use it as a library, but don't add it to `speakoro-cli` yet
3. Go to `speakoro-cli` and replace the `phonemoro` dependency like so:
```shell
$ cargo rm phonemoro && cargo add --path <path-to-the-cloned-phonemoro-repo> phonemoro
```
4. Remove the `download-data` feature from `speakoro`:
```shell
$ cargo rm speakoro && cargo add --path .. speakoro
```

5. Optionally change the Kokoro model like described before, then build `speakoro-cli`:
```shell
$ cargo build -p speakoro-cli --release
```


# Cross Compiling
_TODO (main limitation is [ort](https://github.com/pykeio/ort), which you might need to manually build)_

# Acknowledgements
- [hexgrad/Kokoro](https://github.com/hexgrad/kokoro): The model this library is based on.
- [onnx-community/Kokoro-82M-v1.0-ONNX](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX): The quantized and to onnx converted models this library uses.
- [lucasjinreal/Kokoros](https://github.com/lucasjinreal/Kokoros): Another "Kokoro in Rust" project I recently found out about. It has more features and almost certainly better phonemization, since it uses espeak as a backend. However, it needs Python (and possibly PyTorch) for the installation, requires vendored espeak, Kokoro onnx models and voice data in external directories. <br>So, if you need any of the additional features `Kokoros` provides, or better phonemization, use `Kokoros`. If you need a self contained binary, want easier installation or usage as a library, or don't want to use espeak because of licensing issues, use `speakoro`.

# Attribution

This project utilizes data from [onnx-community/Kokoro-82M-v1.0-ONNX](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX/tree/main), licensed under the Apache License 2.0.

# License

`speakoro` is licensed under the MIT License.
