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
use crate::parameters::Parameters;
use nih_plug::prelude::*;
use std::sync::Arc;

pub mod synth;
use synth::Synth;
pub mod generator;
use generator::{Generator, GeneratorTrait};
pub mod midi;
use midi::Midi;

pub struct App {
    //Parameters
    params: Arc<Parameters>,
    //Synth
    synth: Synth,
}

impl Default for App {
    fn default() -> Self {
        Self {
            params: Arc::new(Parameters::default()),
            synth: Synth::default(),
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
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext,
    ) -> bool {
        //Получить данные при загрузке.
        //=============================
        // Sample Rate 44100.0 или 48000.0
        let sample_rate: f32 = buffer_config.sample_rate;
        self.synth.setting(sample_rate);
        //=============================
        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        // !!! channel - Каждый канал (L, R).
        // !!! time_buffer -(время буфера) Индекс семпла для каждого канала в буфере.
        // !!! sample - Каждый семпл для каждого канала в буфере.
        for (_time_buffer, channel) in buffer.iter_samples().enumerate() {
            for sample in channel {
                //Bypass
                if self.params.bypass.value() {
                    *sample = *sample;
                } else {
                    // Получить midi событие
                    self.synth.input_midi(context);
                    // Создать `signal` - это аудио сигнал, генератора волны
                    let output_signal = self.synth.output(&self.params);
                    // Громкость параметр
                    let master_gain = self.params.master_gain.smoothed.next();
                    // Вывод
                    *sample = output_signal * util::db_to_gain(master_gain);
                }
            }
        }
        ProcessStatus::Normal
    }
}
