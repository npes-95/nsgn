extern crate rand;

use rand::{distributions::Uniform, Rng};

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

#[derive(Debug)]
pub struct Noise {
    color: Color,
    interpolation: Interpolation,
    clip_len: f32,
}

impl Noise {
    pub fn new(color: &str, interpolation: &str, clip_len: &str) -> Result<Noise, &'static str> {

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

        let clip_len: f32 = match clip_len.trim().parse::<f32>() {
            Ok(n) => n*44100f32,
            Err(_) => return Err("Invalid input."),
        };

        Ok(Noise {color, interpolation, clip_len})      
    }

    pub fn generate(&self) -> Vec<i16> {

        let mut rng = rand::thread_rng();
        let range: Uniform<i16> = Uniform::new(std::i16::MIN, std::i16::MAX);

        (0..self.clip_len as u32).map(|_| rng.sample(&range)).collect()
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
        assert!(Noise::new("not_an option", "not_an_option", "not an option").is_err());        
    }

    #[test]
    fn test_generate() -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

}