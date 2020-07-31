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

pub struct Noise {
    alpha: f32,
    distr: Distribution, 
    chunk_len: usize,
    rng: rand::rngs::ThreadRng,
    fft: std::sync::Arc<dyn FFT<f32>>,
    ifft: std::sync::Arc<dyn FFT<f32>>
}

impl Noise {
    pub fn new(alpha: f32, distribution: &str, chunk_len: f32) -> Result<Noise, &'static str> {

        if alpha < -2f32 || alpha > 2f32 {
            return Err("Invalid input (alpha).")
        }

        let distr = match distribution.to_lowercase().trim() {
            "normal" => Distribution::Normal,
            "uniform" => Distribution::Uniform,
            "" => Distribution::Uniform,
            _ => return Err("Invalid input (distribution)."),
        };

        let chunk_len = (chunk_len*44100f32).abs() as usize;

        let rng = rand::thread_rng();

        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(chunk_len);

        planner = FFTplanner::new(true);
        let ifft = planner.plan_fft(chunk_len);

        Ok(Noise {alpha, distr, chunk_len, rng, fft, ifft})      
    }

    pub fn generate_chunk(&mut self) -> Vec<f32> {
        
        let distr = rand_distr::Normal::new(0f32, 1f32).unwrap();
        
        let norm_factor = (self.chunk_len as f32).sqrt();

        let mut output: Vec<Complex<f32>> = vec![Complex::zero(); self.chunk_len];

        let mut white_noise: Vec<Complex<f32>> = (0..self.chunk_len).map(|_| Complex::new(self.rng.sample(distr), 0f32)).collect();
        let mut white_noise_ft: Vec<Complex<f32>> = vec![Complex::zero(); self.chunk_len];

        let mut coeffs: Vec<Complex<f32>> = vec![Complex::one(); self.chunk_len];
        let mut coeffs_ft: Vec<Complex<f32>> = vec![Complex::zero(); self.chunk_len];

        for i in 1..self.chunk_len {
            coeffs[i] = coeffs[i-1] * (0.5 * self.alpha + (i as f32 - 1f32))/i as f32;
        }

        self.fft.process(&mut coeffs, &mut coeffs_ft);
        self.fft.process(&mut white_noise, &mut white_noise_ft);

        for bin in white_noise_ft.iter_mut() {
            *bin /= norm_factor;
        }

        for coeff in coeffs_ft.iter_mut() {
            *coeff /= norm_factor;
        }
        
        for i in 0..self.chunk_len {
            let wn_re = white_noise_ft[i].re;
            let wn_im = white_noise_ft[i].im;

            white_noise_ft[i].re = wn_re * coeffs_ft[i].re - wn_im * coeffs_ft[i].im;
            white_noise_ft[i].im = wn_im * coeffs_ft[i].re - wn_re * coeffs_ft[i].im;
        }

        self.ifft.process(&mut white_noise_ft, &mut output);

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
        assert!(Noise::new(-2.5, "not_an_option", -3.0).is_err());        
    }

    #[test]
    fn test_generate_chunk() -> Result<(), Box<dyn Error>> {
        // TODO
        Ok(())
    }

}