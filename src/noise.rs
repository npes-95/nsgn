extern crate rand;
extern crate rand_distr;

use rand::Rng;


#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    White,
    Pink,
    Brownian,
    Blue,
    Violet,
    Grey,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Interpolation {
    None,
    Linear,
    Polynomial,
    Spline,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Distribution {
    Normal,
    Uniform,
}

#[derive(Debug)]
pub struct Noise {
    color: Color,
    interpolation: Interpolation,
    distribution: Distribution, 
    clip_len: f32,
}

impl Noise {
    pub fn new(color: &str, interpolation: &str, distribution: &str, clip_len: &str) -> Result<Noise, &'static str> {

        let color = match color.to_lowercase().trim() {
            "white" => Color::White,
            "pink" => Color::Pink,
            "brownian" => Color::Brownian,
            "blue" => Color::Blue,
            "violet" => Color::Violet,
            "grey" | "gray" => Color::Grey,
            "" => Color::White,
            _ => return Err("Invalid input."),
        }; 

        let interpolation = match interpolation.to_lowercase().trim() {
            "linear" => Interpolation::Linear,
            "polynomial" => Interpolation::Polynomial,
            "spline" => Interpolation::Spline,
            "" => Interpolation::None,
            _ => return Err("Invalid input."),
        };
        let distribution = match distribution.to_lowercase().trim() {
            "normal" => Distribution::Normal,
            "uniform" => Distribution::Uniform,
            "" => Distribution::Uniform,
            _ => return Err("Invalid input."),
        };

        let clip_len: f32 = match clip_len.trim().parse::<f32>() {
            Ok(n) => n*44100f32,
            Err(_) => return Err("Invalid input."),
        };

        Ok(Noise {color, interpolation, distribution, clip_len})      
    }

    pub fn generate(&self) -> Vec<i16> {

        let mut rng = rand::thread_rng();

        match self.distribution {
            Distribution::Normal => (0..self.clip_len as u32).map(|_| (((rng.sample::<f32, rand_distr::StandardNormal>(rand_distr::StandardNormal) - 0.5) * 2.0) * std::i16::MAX as f32) as i16).collect(),
            Distribution::Uniform => (0..self.clip_len as u32).map(|_| rng.sample(rand_distr::Uniform::new(std::i16::MIN, std::i16::MAX))).collect(),
        }
    }
}

impl Noise {

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_new() {
        // invalid input returns error
        assert!(Noise::new("not_an option", "not_an_option", "not an option", "not an option").is_err());        
    }

    #[test]
    fn test_generate() -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

}