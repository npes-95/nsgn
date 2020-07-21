extern crate hound;

use std::error::Error;

pub mod noise;

pub fn run(config: clap::ArgMatches) -> Result<(), Box<dyn Error>> {

    // init generator 
    let generator = noise::Noise::new(
        config.value_of("Color").unwrap_or(""), 
        config.value_of("Interpolation").unwrap_or(""),
        config.value_of("Distribution").unwrap_or(""),
        config.value_of("Length").unwrap_or("1"),
    )?;

    // init file writer
    let writer_cfg = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let filename = config.value_of("Output File").unwrap_or("noise.wav");

    let mut writer = hound::WavWriter::create(filename, writer_cfg)?;

    let mut out_buf: Vec<f32> = generator.generate();
    

    // write out to file
    for sample in out_buf.drain(..) {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;

    Ok(())
}