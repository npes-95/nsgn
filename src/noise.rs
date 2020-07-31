extern crate rand;
extern crate rand_distr;

use rand::Rng;
use rustfft::FFT;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::num_traits::One;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Distribution {
    Normal,
    Uniform,
}

/// Noise generator.
///
/// Uses 1/f^alpha method to produce different colours of noise.
/// See https://ieeexplore.ieee.org/document/381848 for more details.
/// 
pub struct Noise {
    alpha: f32,
    distr: Distribution, 
    chunk_len: usize,
    rng: rand::rngs::ThreadRng,
    fft: std::sync::Arc<dyn FFT<f32>>,
    ifft: std::sync::Arc<dyn FFT<f32>>
}

impl Noise {
    /// Construct a new Noise generator with the given alpha. distribution and
    /// chunk length.
    pub fn new(alpha: f32, distribution: &str, chunk_len: usize) -> Result<Noise, &'static str> {
        // verify input
        if alpha < -2f32 || alpha > 2f32 {
            return Err("Invalid input (alpha).")
        }

        let distr = match distribution.to_lowercase().trim() {
            "normal" => Distribution::Normal,
            "uniform" => Distribution::Uniform,
            "" => Distribution::Uniform,
            _ => return Err("Invalid input (distribution)."),
        };

        if chunk_len == 0 {
            return Err("Invalid input (chunk length).")
        }

        // create rng and fft objects
        let rng = rand::thread_rng();

        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(chunk_len);

        planner = FFTplanner::new(true);
        let ifft = planner.plan_fft(chunk_len);

        Ok(Noise {alpha, distr, chunk_len, rng, fft, ifft})      
    }

    /// Generates a noise vector of the given chunk size and returns it.
    pub fn generate_chunk(&mut self) -> Vec<f32> {        
        // noise source distribution (mean 0, deviation 1)
        let distr = rand_distr::Normal::new(0f32,1f32).unwrap();
        
        // FFT normalisation factor
        let norm_factor = (self.chunk_len as f32).sqrt();

        // vector initialisation
        let mut output: Vec<Complex<f32>> = vec![Complex::zero(); self.chunk_len];

        let mut white_noise: Vec<Complex<f32>> = (0..self.chunk_len).map(|_| Complex::new(self.rng.sample(distr), 0f32)).collect();
        let mut white_noise_ft: Vec<Complex<f32>> = vec![Complex::zero(); self.chunk_len];

        let mut coeffs: Vec<Complex<f32>> = vec![Complex::one(); self.chunk_len];
        let mut coeffs_ft: Vec<Complex<f32>> = vec![Complex::zero(); self.chunk_len];

        // calculate coefficients
        for i in 1..self.chunk_len {
            coeffs[i] = coeffs[i-1] * (0.5 * self.alpha + (i as f32 - 1f32))/i as f32;
        }

        // take FFT of white noise and coeffs
        self.fft.process(&mut coeffs, &mut coeffs_ft);
        self.fft.process(&mut white_noise, &mut white_noise_ft);

        // normalise result
        for bin in white_noise_ft.iter_mut() {
            *bin /= norm_factor;
        }

        for coeff in coeffs_ft.iter_mut() {
            *coeff /= norm_factor;
        }
        
        // product of white noise and coeffs
        for i in 0..self.chunk_len {
            let wn_re = white_noise_ft[i].re;
            let wn_im = white_noise_ft[i].im;

            white_noise_ft[i].re = wn_re * coeffs_ft[i].re - wn_im * coeffs_ft[i].im;
            white_noise_ft[i].im = wn_im * coeffs_ft[i].re - wn_re * coeffs_ft[i].im;
        }

        // take inverse FFT of the result
        self.ifft.process(&mut white_noise_ft, &mut output);

        // return real part of the output
        output.iter().map(|x| x.re / norm_factor).collect()        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_new() {
        // invalid input returns error
        assert!(Noise::new(-2.5, "not_an_option", 0).is_err());        
    }

    #[test]
    fn test_generate_chunk() -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

}