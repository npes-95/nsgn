extern crate hound;

use std::error::Error;

pub mod noise;

pub fn run(config: clap::ArgMatches) -> Result<(), Box<dyn Error>> {

    // parse user input
    let alpha: f32 = match config.value_of("Color").unwrap_or("").to_lowercase().trim() {
        "white" => 0f32,
        "pink" => 1f32,
        "brownian" => 2f32,
        "blue" => -1f32,
        "violet" => -2f32,
        "grey" | "gray" => 0f32,
        _ => 0f32,
    };
    let distr: &str = config.value_of("Distribution").unwrap_or("");
    let chunk_len: f32 = config.value_of("Length").unwrap_or("1").parse()?;

    // init generator 
    let mut generator = noise::Noise::new(alpha, distr, chunk_len)?;

    // init file writer
    let writer_cfg = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let filename = config.value_of("Output File").unwrap_or("noise.wav");

    let mut writer = hound::WavWriter::create(filename, writer_cfg)?;

    let mut out_buf: Vec<f32> = generator.generate_chunk();    

    // write out to file
    for sample in out_buf.drain(..) {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;

    Ok(())
}