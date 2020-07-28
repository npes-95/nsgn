extern crate rand;
extern crate rand_distr;

use rand::Rng;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::num_traits::One;


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
    clip_len: usize,
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

        let clip_len = match clip_len.trim().parse::<f32>() {
            Ok(n) => (n*44100f32) as usize,
            Err(_) => return Err("Invalid input."),
        };

        Ok(Noise {color, interpolation, distribution, clip_len})      
    }

    pub fn generate(&self) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        let distr = rand_distr::Normal::new(0f32, 1f32);

        let alpha = match self.color {
            Color::White => 0f32,
            Color::Pink => 1f32,
            Color::Brownian => 2f32,
            Color::Blue => -1f32,
            Color::Violet => -2f32,
            Color::Grey => 0f32,
        };
        
        let norm_factor = (self.clip_len as f32).sqrt();
        let mut output: Vec<Complex<f32>> = vec![Complex::zero(); self.clip_len];

        let mut white_noise: Vec<Complex<f32>> = (0..self.clip_len).map(|_| Complex::new(rng.sample(distr.unwrap()), 0f32)).collect();
        let mut white_noise_ft: Vec<Complex<f32>> = vec![Complex::zero(); self.clip_len];

        let mut coeffs: Vec<Complex<f32>> = vec![Complex::one(); self.clip_len];
        let mut coeffs_ft: Vec<Complex<f32>> = vec![Complex::zero(); self.clip_len];

        for i in 1..self.clip_len {
            coeffs[i] = coeffs[i-1] * (0.5 * alpha + (i as f32 - 1f32))/i as f32;
        }

        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(self.clip_len);
        fft.process(&mut coeffs, &mut coeffs_ft);
        fft.process(&mut white_noise, &mut white_noise_ft);

        for bin in white_noise_ft.iter_mut() {
            *bin /= norm_factor;
        }

        for coeff in coeffs_ft.iter_mut() {
            *coeff /= norm_factor;
        }
        
        for i in 0..self.clip_len {
            let wn_re = white_noise_ft[i].re;
            let wn_im = white_noise_ft[i].im;

            white_noise_ft[i].re = wn_re * coeffs_ft[i].re - wn_im * coeffs_ft[i].im;
            white_noise_ft[i].im = wn_im * coeffs_ft[i].re - wn_re * coeffs_ft[i].im;
        }

        planner = FFTplanner::new(true);
        let ifft = planner.plan_fft(self.clip_len);
        ifft.process(&mut white_noise_ft, &mut output);

        output.iter().map(|x| x.re / norm_factor).collect()        
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