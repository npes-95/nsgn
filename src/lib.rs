extern crate hound;

use std::error::Error;
use std::fmt;

pub mod noise;

#[derive(Debug)]
struct NsgnError(String);

impl fmt::Display for NsgnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for NsgnError {}

pub fn run(config: clap::ArgMatches) -> Result<(), Box<dyn Error>> {

    // init generator 
    let generator = noise::Noise::new(
        config.value_of("Color").unwrap_or(""), 
        config.value_of("Interpolation").unwrap_or(""),
        config.value_of("Distribution").unwrap_or(""),
        config.value_of("Length").unwrap_or(""),
    )?;

    // init file writer
    let writer_cfg = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let filename = config.value_of("Output File").unwrap_or("noise.wav");

    let mut writer = hound::WavWriter::create(filename, writer_cfg)?;

    let mut out_buf: Vec<i16> = generator.generate();
    

    // write out to file
    for sample in out_buf.drain(..) {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;

    Ok(())
}