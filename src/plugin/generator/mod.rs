pub mod osc;
use osc::Osc as Oscillator;
pub trait GeneratorTrait{
    ///Настройки
    fn setting(&mut self, sample_rate: f32);
    fn output(&mut self, frequency: f32)->f32;
}

impl GeneratorTrait for Oscillator{
    ///Настройки
    fn setting(&mut self, sample_rate: f32){
        self.setting(sample_rate);
    }
    fn output(&mut self, frequency: f32)->f32{
        self.sine(frequency)
    }   
}


#[derive(Default, Clone)]
pub struct Generator{
    osc: Oscillator
}
impl GeneratorTrait for Generator {
    fn setting(&mut self, sample_rate: f32) {
       self.osc.setting(sample_rate)
    }

    fn output(&mut self, frequency: f32)->f32 {
        self.osc.output(frequency)
    }
}
