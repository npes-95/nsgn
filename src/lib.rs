extern crate hound;

use std::error::Error;

pub mod noise;

pub fn run(config: clap::ArgMatches) -> Result<(), Box<dyn Error>> {

    // parse user input
    let sample_rate = match config.value_of("Sample Rate").unwrap() {
        "16000" => 16000,
        "44100" => 44100,
        "48000" => 48000,
        _ => 44100
    };

    let alpha : f32 = if config.is_present("Alpha") {
        config.value_of("Alpha").unwrap().parse().unwrap()  
    } else {
        match config.value_of("Color").unwrap_or("").to_lowercase().trim() {
            "white" => 0f32,
            "pink" => 1f32,
            "brownian" => 2f32,
            "blue" => -1f32,
            "violet" => -2f32,
            "grey" | "gray" => 0f32,
            _ => 0f32,
        }
    };

    let distr: &str = config.value_of("Distribution").unwrap_or("");

    let output_len: f32 = config.value_of("Length").unwrap_or("1").parse()?;
    let chunk_len: usize = (output_len * sample_rate as f32).abs() as usize;

    let filename = config.value_of("Output File").unwrap_or("noise.wav");
    

    // init generator 
    let mut generator = noise::Noise::new(alpha, distr, chunk_len)?;

    // init file writer
    let writer_cfg = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(filename, writer_cfg)?;


    // get noise
    let mut out_buf: Vec<f32> = generator.generate_chunk();    

    // write out to file
    for sample in out_buf.drain(..) {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;

    Ok(())
}