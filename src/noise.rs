extern crate rand;

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
}

impl Noise {
    pub fn new(color: &str, interpolation: &str) -> Result<Noise, &'static str> {

        let color = match color.to_lowercase().trim() {
            "white" => Color::White,
            "pink" => Color::Pink,
            "brownian" => Color::Brownian,
            "blue" => Color::Blue,
            "violet" => Color::Violet,
            "grey" | "gray" => Color::Grey,
            _ => return Err("Invalid input."),
        }; 

        let interpolation = match interpolation.to_lowercase().trim() {
            "linear" => Interpolation::Linear,
            "polynomial" => Interpolation::Polynomial,
            "spline" => Interpolation::Spline,
            "" => Interpolation::None,
            _ => return Err("Invalid input."),
        }; 

        Ok(Noise {color, interpolation})      
    }

    pub fn populate(&self, buf: &mut [i16; 2048]) -> Result<(), &'static str> {
        for sample in buf.iter_mut() {
            *sample = rand::random();
        }
        Ok(())
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
        assert!(Noise::new("not_an option", "not_an_option").is_err());        
    }

    #[test]
    fn test_populate() -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

}