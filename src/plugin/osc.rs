use std::f32::consts::TAU;
#[derive(Default, Clone)]
pub struct Osc {
    sample_rate: f32, 
    phase: f32,
}
impl Osc{
    ///Настройки 
    pub fn setting(&mut self, sample_rate: f32){
        self.sample_rate = sample_rate;
    }
    /// Базовая волна синусоида - с отслеживанием фазы
    #[allow(dead_code)]
    pub fn sine(&mut self, frequency: f32) -> f32{
        let output = (TAU * self.phase).sin();
        self.inc(frequency);
        self.phase();
        output
    }
    fn phase(&mut self){
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        };
    }
    fn inc(&mut self, frequency: f32){
        let phase_delta = frequency / self.sample_rate / 2.0;
        self.phase += phase_delta;
    }
}

///Запустить тест можно командой в терминале:
///```rust
///$ cargo test
///```
#[cfg(test)] 
mod tests {
    use super::*;
   
    // Аргументы методов.
    const SAMPLE_RATE: f32 = 44100.0;
    const FREQUENCY: f32 = 440.0;

    #[test]
    fn test_sine(){
        let mut audio_data = [0.0; 512];
        let mut osc = Osc::default();
        osc.setting(SAMPLE_RATE);
        for n in 0..512{
            audio_data[n] = osc.sine(FREQUENCY);
        } 
        println!("{:?}", audio_data); 
        assert_eq!(audio_data[0], 0.0);
        //panic!(" ");              
    }
}