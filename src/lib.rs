extern crate hound;

use std::error::Error;

pub mod noise;

pub fn run(config: clap::ArgMatches) -> Result<(), Box<dyn Error>> {

    // init generator 
    let generator = noise::Noise::new(
        config.value_of("Color").unwrap_or(""), 
        config.value_of("Interpolation").unwrap_or(""),
    )?;

    // init file writer
    let writer_cfg = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("filename.wav", writer_cfg)?;

    // create output buffer
    let mut out_buf: [i16; 2048] = [0; 2048];

    // populate with samples
    generator.populate(&mut out_buf)?;

    // write out to file
    for sample in out_buf.iter() {
        writer.write_sample(sample.clone())?;
    }

    writer.finalize()?;

    Ok(())
}