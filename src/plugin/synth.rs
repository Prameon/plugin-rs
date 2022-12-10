use crate::parameters::Parameters;
use nih_plug::prelude::*;
use std::sync::Arc;
use super::*;

#[derive(Default)]
pub struct Synth {
    //Частота дискретизации
    sample_rate: f32,
    //Активные(нажатые) ноты.
    midi: Midi,
    //Осциллятор
    osc: Generator,
    //Голоса.
    voice: Vec<Generator>,
}
 
impl Synth {
    ///Настройки
    pub fn setting(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.osc.setting(sample_rate);
        self.midi.setting(sample_rate);
    }
    ///Получить midi-событие
    #[allow(dead_code)]
    pub fn input_midi(&mut self, context: &mut impl ProcessContext) {
        self.midi.input_midi(context);
    }
    ///Аудио вывод
    pub fn output(&mut self, _params: &Arc<Parameters>) -> f32 {

        //Сумма голосов
        let mut mix = vec![0.0];

        self.midi.map_active_notes(
            |note, note_gain, count_voice|{
            //Копия генератора для голоса 
            self.voice.push((&self.osc).clone());

            //Добавить голос
            mix.push(
                self.voice[count_voice]
                .output(util::midi_note_to_freq(note))
                 * note_gain.next()
            );
        });
        
        //Результат - Сумма голосов
        mix.into_iter().sum()
    }
}