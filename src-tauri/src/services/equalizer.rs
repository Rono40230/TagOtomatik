use rodio::{Source};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// --- EQ PARAMETERS ---
#[derive(Clone)]
pub struct EqParams {
    pub bass_gain: f32,   // -10.0 to 10.0 dB
    pub mid_gain: f32,    // -10.0 to 10.0 dB
    pub treble_gain: f32, // -10.0 to 10.0 dB
}

impl Default for EqParams {
    fn default() -> Self {
        Self {
            bass_gain: 0.0,
            mid_gain: 0.0,
            treble_gain: 0.0,
        }
    }
}

// --- BASIC BIQUAD FILTER IMPL ---
pub struct Biquad {
    a0: f32, a1: f32, a2: f32,
    b1: f32, b2: f32,
    z1: f32, z2: f32,
}

impl Biquad {
    pub fn new() -> Self {
        Self { a0: 1.0, a1: 0.0, a2: 0.0, b1: 0.0, b2: 0.0, z1: 0.0, z2: 0.0 }
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        let out = sample * self.a0 + self.z1;
        self.z1 = sample * self.a1 + self.z2 - self.b1 * out;
        self.z2 = sample * self.a2 - self.b2 * out;
        out
    }

    // Low Shelf for Bass (approx 200Hz)
    pub fn set_low_shelf(&mut self, gain_db: f32, rate: u32) {
        let _avg_gain = 10.0f32.powf(gain_db / 40.0); // Amplification factor
        let w0 = 2.0 * std::f32::consts::PI * 250.0 / rate as f32;
        let alpha = w0.sin() / 2.0; 
        
        let a = 10.0f32.powf(gain_db / 40.0); // For low shelf formula A parameter
        // Simplified shelving approximation to avoid full complexity if gain is 0
        if gain_db.abs() < 0.1 {
             self.a0 = 1.0; self.a1 = 0.0; self.a2 = 0.0; self.b1 = 0.0; self.b2 = 0.0;
             return;
        }
        
        // RBJ Cookbook Low Shelf
        let cos_w0 = w0.cos();
        let sqrt_a = a.sqrt();
        
        let b0_n = min_check(a*((a+1.0) - (a-1.0)*cos_w0 + 2.0*sqrt_a*alpha));
        let b1_n = min_check(2.0*a*((a-1.0) - (a+1.0)*cos_w0));
        let b2_n = min_check(a*((a+1.0) - (a-1.0)*cos_w0 - 2.0*sqrt_a*alpha));
        let a0_n = min_check((a+1.0) + (a-1.0)*cos_w0 + 2.0*sqrt_a*alpha);
        let a1_n = min_check(-2.0*((a-1.0) + (a+1.0)*cos_w0));
        let a2_n = min_check((a+1.0) + (a-1.0)*cos_w0 - 2.0*sqrt_a*alpha);

        self.a0 = b0_n / a0_n;
        self.a1 = b1_n / a0_n;
        self.a2 = b2_n / a0_n;
        self.b1 = a1_n / a0_n;
        self.b2 = a2_n / a0_n;
    }

    // Peaking for Mid (approx 1000Hz)
    pub fn set_peaking(&mut self, gain_db: f32, rate: u32) {
        if gain_db.abs() < 0.1 {
             self.a0 = 1.0; self.a1 = 0.0; self.a2 = 0.0; self.b1 = 0.0; self.b2 = 0.0;
             return;
        }
        let a = 10.0f32.powf(gain_db / 40.0);
        let w0 = 2.0 * std::f32::consts::PI * 1000.0 / rate as f32;
        let alpha = w0.sin() / (2.0 * 1.0); // Q = 1.0

        let cos_w0 = w0.cos();

        let b0_n = min_check(1.0 + alpha * a);
        let b1_n = min_check(-2.0 * cos_w0);
        let b2_n = min_check(1.0 - alpha * a);
        let a0_n = min_check(1.0 + alpha / a);
        let a1_n = min_check(-2.0 * cos_w0);
        let a2_n = min_check(1.0 - alpha / a);

        self.a0 = b0_n / a0_n;
        self.a1 = b1_n / a0_n;
        self.a2 = b2_n / a0_n;
        self.b1 = a1_n / a0_n;
        self.b2 = a2_n / a0_n;
    }

    // High Shelf for Treble (approx 4000Hz)
    pub fn set_high_shelf(&mut self, gain_db: f32, rate: u32) {
        if gain_db.abs() < 0.1 {
            self.a0 = 1.0; self.a1 = 0.0; self.a2 = 0.0; self.b1 = 0.0; self.b2 = 0.0;
            return;
       }
       let a = 10.0f32.powf(gain_db / 40.0);
       let w0 = 2.0 * std::f32::consts::PI * 4000.0 / rate as f32;
       let alpha = w0.sin() / 2.0;

       let cos_w0 = w0.cos();
       let sqrt_a = a.sqrt();

       let b0_n = min_check(a*((a+1.0) + (a-1.0)*cos_w0 + 2.0*sqrt_a*alpha));
       let b1_n = min_check(-2.0*a*((a-1.0) + (a+1.0)*cos_w0));
       let b2_n = min_check(a*((a+1.0) + (a-1.0)*cos_w0 - 2.0*sqrt_a*alpha));
       let a0_n = min_check((a+1.0) - (a-1.0)*cos_w0 + 2.0*sqrt_a*alpha);
       let a1_n = min_check(2.0*((a-1.0) - (a+1.0)*cos_w0));
       let a2_n = min_check((a+1.0) - (a-1.0)*cos_w0 - 2.0*sqrt_a*alpha);

       self.a0 = b0_n / a0_n;
       self.a1 = b1_n / a0_n;
       self.a2 = b2_n / a0_n;
       self.b1 = a1_n / a0_n;
       self.b2 = a2_n / a0_n;
    }
}

// Helper to prevent NaN/Infinity issues slightly
fn min_check(val: f32) -> f32 {
    if val.is_nan() { 0.0 } else { val }
}

// --- EQUALIZER SOURCE ---
pub struct EqualizerSource<I> {
    input: I,
    params: Arc<Mutex<EqParams>>,
    channels: u16,
    sample_rate: u32,
    filters: Vec<[Biquad; 3]>,
    update_counter: u32,
    current_channel: usize,
}

impl<I> EqualizerSource<I> 
where I: Source<Item = f32> 
{
    pub fn new(input: I, params: Arc<Mutex<EqParams>>) -> Self {
        let channels = input.channels();
        let sample_rate = input.sample_rate();
        
        let mut filters = Vec::with_capacity(channels as usize);
        for _ in 0..channels {
            filters.push([Biquad::new(), Biquad::new(), Biquad::new()]);
        }

        Self {
            input,
            params,
            channels,
            sample_rate,
            filters,
            update_counter: 0,
            current_channel: 0,
        }
    }

    fn update_coefficients(&mut self) {
        if let Ok(p) = self.params.lock() {
            for ch in 0..self.channels as usize {
                self.filters[ch][0].set_low_shelf(p.bass_gain, self.sample_rate);
                self.filters[ch][1].set_peaking(p.mid_gain, self.sample_rate);
                self.filters[ch][2].set_high_shelf(p.treble_gain, self.sample_rate);
            }
        }
    }
}

impl<I> Iterator for EqualizerSource<I>
where I: Source<Item = f32>
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.input.next()?;
        
        // Use local copy of current channel
        let ch = self.current_channel;

        // Apply filters
        let mut out = sample;
        // Safety check for bounds (though channels should be constant)
        if ch < self.filters.len() {
             out = self.filters[ch][0].process(out);
             out = self.filters[ch][1].process(out);
             out = self.filters[ch][2].process(out);
        }

        // Advance channel
        self.current_channel = (self.current_channel + 1) % (self.channels as usize);

        // Update coefficients periodically (once per full frame cycle roughly)
        if self.current_channel == 0 {
            self.update_counter += 1;
            if self.update_counter > 500 { // Every ~500 frames updates params (10-12ms at 44.1k)
                self.update_coefficients();
                self.update_counter = 0;
            }
        }
        
        Some(out)
    }
}

impl<I> Source for EqualizerSource<I> 
where I: Source<Item = f32> 
{
    fn current_frame_len(&self) -> Option<usize> { self.input.current_frame_len() }
    fn channels(&self) -> u16 { self.channels }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { self.input.total_duration() }
}
