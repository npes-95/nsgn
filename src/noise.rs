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
            _ => Color::White,
        }; 

        let interpolation = match interpolation.to_lowercase().trim() {
            "linear" => Interpolation::Linear,
            "polynomial" => Interpolation::Polynomial,
            "spline" => Interpolation::Spline,
            _ => Interpolation::None,
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
        // invalid unit
        assert!(Temperature::new(String::from("32z")).is_err());
        
        // invalid input
        assert!(Temperature::new(String::from("wefigyudgqdt%$34571234723gqwe87")).is_err());

        // out of bounds (below absolute zero)
        assert!(Temperature::new(String::from("-10000c")).is_err());
        assert!(Temperature::new(String::from("-10000f")).is_err());
        assert!(Temperature::new(String::from("-10000k")).is_err());

        // valid input
        assert!(!Temperature::new(String::from("30c")).is_err());
        assert!(!Temperature::new(String::from("30f")).is_err());
        assert!(!Temperature::new(String::from("30k")).is_err());
    }

    #[test]
    fn test_convert() -> Result<(), Box<dyn Error>> {
        let abs_zero_k = Temperature::new(String::from(ABSOLUTE_ZERO_K.to_string() + "k"))?;
        let abs_zero_f = Temperature::new(String::from(ABSOLUTE_ZERO_F.to_string() + "f"))?;
        let abs_zero_c = Temperature::new(String::from(ABSOLUTE_ZERO_C.to_string() + "c"))?;

        assert_eq!(abs_zero_f, abs_zero_c.to_fahrenheit());
        assert_eq!(abs_zero_f, abs_zero_k.to_fahrenheit());

        assert_eq!(abs_zero_c, abs_zero_f.to_celcius());
        assert_eq!(abs_zero_c, abs_zero_k.to_celcius());

        assert_eq!(abs_zero_k, abs_zero_c.to_kelvin());
        assert_eq!(abs_zero_k, abs_zero_f.to_kelvin());        

        Ok(())
    }

    #[test]
    fn test_to_string() -> Result<(), Box<dyn Error>> {
        let abs_zero_k = Temperature::new(String::from(ABSOLUTE_ZERO_K.to_string() + "k"))?;

        assert_eq!(abs_zero_k.to_string(), String::from(ABSOLUTE_ZERO_K.to_string() + "k"));

        Ok(())
    }

    #[test]
    fn test_unit() -> Result<(), Box<dyn Error>> {
        let abs_zero_k = Temperature::new(String::from(ABSOLUTE_ZERO_K.to_string() + "k"))?;
        let abs_zero_f = Temperature::new(String::from(ABSOLUTE_ZERO_F.to_string() + "f"))?;
        let abs_zero_c = Temperature::new(String::from(ABSOLUTE_ZERO_C.to_string() + "c"))?;

        assert_eq!(abs_zero_k.unit(), Unit::Kelvin);
        assert_eq!(abs_zero_f.unit(), Unit::Fahrenheit);
        assert_eq!(abs_zero_c.unit(), Unit::Celcius);        

        Ok(())

    }
}