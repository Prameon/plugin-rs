/* Примеры базовых плагинов 
Для запуска - раскомментировать модуль и нужный пример, 
и закомментировать основной код без зависимостей.
*/

// Инструмент
//pub mod example;
//pub use example::plugin_instrument::App;

// Эффект
//pub mod example;
//pub use example::plugin_effect::App;



// Основной код.===============================================
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
    const NAME: &'static str = "Test Plugin";
    const VENDOR: &'static str = "Prameon";
    const URL: &'static str = "https://dzen.ru/rust";
    const EMAIL: &'static str = "gusev11v13@yandex.ru";
    const VERSION: &'static str = "0.0.1";
    const DEFAULT_INPUT_CHANNELS: u32 = 2;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    const HARD_REALTIME_ONLY: bool = false;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone() as Arc<dyn Params>
    }


    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        //https://github.com/robbert-vdh/nih-plug-template/blob/master/%7B%7B%20cookiecutter.project_name%20%7D%7D/src/lib.rs
        //Поддерживает ли плагин конфигурацию ввод шины.
        (config.num_input_channels == 0) && (config.num_input_channels > 0)
    }

    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext,
    ) -> bool {
        //Получить данные про загрузке.
        //=============================
        // Sample Rate 44100.0 или 48000.0
        //let sample_rate: f32 = buffer_config.sample_rate; 
        //=============================
        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        // !!! channel - Каждый канал (L, R).
        // !!! sample - Каждый семпл для каждого канала в буфере.
        for channel in buffer.iter_samples() {
            for sample in channel{
                if self.params.bypass.value() {
                    *sample = *sample; 
                } else {
                    let master_gain = self.params.master_gain.smoothed.next();
                    *sample *= util::db_to_gain(master_gain);
                }
            }
        }
        ProcessStatus::Normal
    }
}


