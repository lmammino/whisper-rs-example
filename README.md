# Whisper-rs speech-to-text example

An example of how to use [Whisper.cpp](https://github.com/ggerganov/whisper.cpp) [bindings for Rust](https://github.com/tazz4843/whisper-rs) to perform speech-to-text.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Download and install your preferred Whisper models](#download-and-install-whisper-models)
- [Provide some sample audio files](#provide-sample-audio-files)

### Download and install Whisper models

In order to run this example, you need to download and install the Whisper models.

Detailed instructions on the best way to do that can be found in the [Whisper.cpp README](https://github.com/ggerganov/whisper.cpp/blob/master/models/README.md).


### Provide sample audio files

Sample audio files can be placed in the `./samples` folder.

Audio files need to be **mono 16bit Wav** files.

You can use [ffmpeg](https://ffmpeg.org/) to convert your audio/video files to this format:

```bash
ffmpeg -i <source_file> -ar 16000 -ac 1 -c:a pcm_s16le <target_file>
```

Note: make sure to replace `<source_file>` and `<target_file>` with the appropriate paths

If you are looking for some interesting audio examples, you can check out the following resources:

- [Wikipedia Audio files of speeches](https://commons.wikimedia.org/wiki/Category:Audio_files_of_speeches)
- [Mozilla Common Voice dataset](https://commonvoice.mozilla.org/en/datasets)
- [LibriSpeech dataset](http://www.openslr.org/12/)
- [VoxForge dataset](http://www.voxforge.org/)
- [TED-LIUM dataset](https://lium.univ-lemans.fr/en/ted-lium3/)
- [University of Illinois Chicago CS 101 - Sample Sound Files](https://www2.cs.uic.edu/~i101/SoundFiles/)
- [UCL London - DIVISION OF PSYCHOLOGY AND LANGUAGE SCIENCES - sample audio files](https://www.uclass.psychol.ucl.ac.uk/Release2/Conversation/AudioOnly/wav/)



## Build and run

```bash
cargo +nightly run --release -- <path_to_audio_file> [path_to_model]
```

Where:

- `path_to_audio_file` - path to a **mono 16bit Wav** audio file to be transcribed, for example `./samples/whisper_demo_16k.wav`.
- `path_to_model` - path to the folder containing the model files, for example `./models/en_16k`. If not provided, it will try to load the first `.bin` file found in the `./models` folder.


## 🙌 Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/oidc-authorizer/issues).


## 👨‍⚖️ License

Licensed under [MIT License](LICENSE). © Luciano Mammino.