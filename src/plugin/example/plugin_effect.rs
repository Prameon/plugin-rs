use nih_plug::prelude::*;
use std::sync::Arc;
use crate::parameters::Parameters;

pub struct App {
    //Parameters
    params: Arc<Parameters>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            params: Arc::new(Parameters::default()),
        }
    }
}

impl Plugin for App {
    const NAME: &'static str = "1111111111111111111111";
    const VENDOR: &'static str = "by Prameon";
    const URL: &'static str = "https://www.youtube.com/c/prameon/";
    const EMAIL: &'static str = "gusev11v13@yandex.ru";
    const VERSION: &'static str = "0.0.1";
    const DEFAULT_INPUT_CHANNELS: u32 = 2;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;
    const MIDI_INPUT: MidiConfig = MidiConfig::None;
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
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext,
    ) -> bool {
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
        //!!! Каждый канал. (L, R). - channel
        for channel in buffer.iter_samples() {

            // Громкость параметр
            let master_gain = self.params.gain.smoothed.next();

            // !!!Каждый семпл буфера.
            for sample in channel{
                //effect
                *sample *= util::db_to_gain(master_gain);
            }
        }
        ProcessStatus::Normal
    }
}




