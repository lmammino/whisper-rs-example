use std::fs;
use std::{env::args, ffi::CStr};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

// ugly hack because the callback for new segment is not safe
extern "C" fn whisper_on_segment(
    _ctx: *mut whisper_rs_sys::whisper_context,
    state: *mut whisper_rs_sys::whisper_state,
    _n_new: std::os::raw::c_int,
    _user_data: *mut std::os::raw::c_void,
) {
    let last_segment = unsafe { whisper_rs_sys::whisper_full_n_segments_from_state(state) } - 1;
    let ret =
        unsafe { whisper_rs_sys::whisper_full_get_segment_text_from_state(state, last_segment) };
    if ret.is_null() {
        panic!("Failed to get segment text")
    }
    let c_str = unsafe { CStr::from_ptr(ret) };
    let r_str = c_str.to_str().expect("invalid segment text");
    println!("-> Segment ({}) text: {}", last_segment, r_str)
}

fn main() {
    let audio_file_path = args().nth(1).expect("No audio file path provided");
    let model_file_path = args().nth(2).unwrap_or_else(|| {
        println!("No model file path provided, looking for a .bin file in the models folder");
        fs::read_dir("./models")
            .expect("failed to read './models' folder")
            .filter_map(|entry| {
                let entry = entry.expect("failed to read entry");
                let path = entry.path();
                if path.is_file() {
                    let extension = path.extension().and_then(|s| s.to_str());
                    if extension == Some("bin") {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .next()
            .expect("no model file found")
            .to_str()
            .expect("invalid model file path")
            .to_string()
    });

    let ctx = WhisperContext::new_with_params(
        model_file_path.as_str(),
        WhisperContextParameters::default(),
    )
    .expect("failed to load model");

    // let bar = ProgressBar::new(100);

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    // params.set_n_threads(8);
    params.set_translate(false);
    params.set_language(Some("en"));
    unsafe {
        params.set_new_segment_callback(Some(whisper_on_segment));
    }
    params.set_progress_callback_safe(|progress| println!("Progress: {}", progress));

    let st = std::time::Instant::now();
    let mut reader = hound::WavReader::open(audio_file_path).unwrap();

    // Convert the audio to floating point samples.
    let audio_data = whisper_rs::convert_integer_to_float_audio(
        &reader
            .samples::<i16>()
            .map(|s| s.expect("invalid sample"))
            .collect::<Vec<_>>(),
    );
    let et = std::time::Instant::now();
    println!(
        "-> Loaded and converted audio file (took {}ms)",
        (et - st).as_millis()
    );

    let st = std::time::Instant::now();
    let mut state = ctx.create_state().expect("failed to create state");
    state
        .full(params, &audio_data[..])
        .expect("failed to run model");

    let num_segments = state
        .full_n_segments()
        .expect("failed to get number of segments");
    for i in 0..num_segments {
        let segment = state
            .full_get_segment_text(i)
            .expect("failed to get segment");
        let start_timestamp = state
            .full_get_segment_t0(i)
            .expect("failed to get segment start timestamp");
        let end_timestamp = state
            .full_get_segment_t1(i)
            .expect("failed to get segment end timestamp");
        println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
        // TODO: format those as json
    }
    let et = std::time::Instant::now();

    println!("-> Finished (took {}ms)", (et - st).as_millis());
}
