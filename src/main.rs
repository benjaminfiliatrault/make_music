use std::f32::consts::PI;
use std::fs::File;
use std::path::Path;

const SAMPLE_RATE: f32 = 44100.0;
const DURATION: f32 = 2.0;
// Make sure to change the casting on `let int_sample` line to fit
// to make sure it fits the right number of bits
const BIT_DEPTH: f32 = 16.0;

const MAX_LOOP: i32 = (SAMPLE_RATE * DURATION) as i32;

fn main() -> std::io::Result<()> {
    let mut sin_oscillator = SinOscillator::new(440.0, 0.5);
    // Signed 16 bits goes from -32,768 till 32,767 so 2^15 - 1
    let max_amplitude = 2.0_f32.powf(BIT_DEPTH - 1.0) - 1.0;

    let path = Path::new("waveform.wav");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut audio_data = Vec::new();

    for _ in 0..MAX_LOOP {
        let sample = sin_oscillator.process();
        // Cast it to a 16 bits integer, forget the floating point
        // check max_amplitude comment to understand why it is casted to a 16 bits
        let max_amplitude_sample = (sample * max_amplitude).round() as i16;
        audio_data.push(max_amplitude_sample);
    }

    let header = wav::Header::new(
        1, 
        1, 
        SAMPLE_RATE as u32, 
        BIT_DEPTH as u16
    );
    let track = wav::BitDepth::Sixteen(audio_data); 

    wav::write(header, &track, &mut file)?;

    Ok(())
}

pub struct SinOscillator {
    amplitude: f32,
    offset: f32,
    angle: f32,
}

impl SinOscillator {
    pub fn new(frequency: f32, amplitude: f32) -> SinOscillator {
        SinOscillator { 
            amplitude, 
            angle: 0.0,
            offset: 2.0 * PI * frequency / SAMPLE_RATE
        }
    }

    pub fn process(&mut self) -> f32 {
        let sample = self.amplitude * self.angle.sin();
        self.angle += self.offset;
        return sample;
    }
}
