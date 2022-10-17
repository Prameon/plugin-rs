use nih_plug::prelude::*;
use std::{sync::Arc};
use crate::parameters::Parameters;


pub struct App {
    //Parameters
    params: Arc<Parameters>,

    //Spec
    sample_rate:f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            params: Arc::new(Parameters::default()),
            sample_rate:44100.0,
        }
    }
}

impl Plugin for App {
    const NAME: &'static str = "1111111111111111111111";
    const VENDOR: &'static str = "by Prameon";
    const URL: &'static str = "https://www.youtube.com/c/prameon/";
    const EMAIL: &'static str = "gusev11v13@yandex.ru";
    const VERSION: &'static str = "0.0.1";
    const DEFAULT_INPUT_CHANNELS: u32 = 0;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone() as Arc<dyn Params>
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        //Поддерживает ли плагин конфигурацию ввода-вывода. 
        (config.num_input_channels == 0) && (config.num_input_channels > 0)
    }

    fn editor(&self) -> Option<Box<dyn Editor>> {
        None
    }
    
    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext,
    ) -> bool {
        //Получить данные про загрузке.
        let sample_rate = buffer_config.sample_rate; //44100.0
        self.sample_rate = sample_rate;
        //=============================
        true
    }

    fn reset(&mut self) {

    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        //!!! Каждый канал. (L, R). - channel_samples
        //!!! Индекс семпла для каждого канала. - id
        for (_id, channel) in buffer.iter_samples().enumerate() {

            // Создать `signal` - это аудио сигнал, генератора волны
            let signal= 0.0; 
            
            // Громкость параметр
            let master_gain = self.params.gain.smoothed.next();

            // !!!Каждый семпл буфера.
            for sample in channel{
                //instrument
                *sample = signal * util::db_to_gain(master_gain);
            }
        }
        ProcessStatus::Normal
    }

}




