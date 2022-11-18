#![allow(dead_code)]
use nih_plug::prelude::{NoteEvent, ProcessContext, Smoother};
use std::collections:: HashMap;

#[derive(Default)]
pub struct Midi {
    ///Активные(нажатые) ноты.
    midi: HashMap<u8, NoteEvent>,
    sample_rate: f32,
}

impl Midi {
    ///Настройки 
    pub fn setting(&mut self, sample_rate: f32){
        self.sample_rate = sample_rate;
    }

    pub fn input_midi(&mut self, context: &mut impl ProcessContext) {
        // Получить новое midi сообщение
        let mut next_event = context.next_event();

        //В цикле, реагируем на midi сообщение.
        while let Some(event) = next_event {
            match event {
                NoteEvent::NoteOn { note, .. } => {
                    self.midi.insert(note, event);
                }
                NoteEvent::NoteOff { note, .. } if self.midi.contains_key(&note) => {
                    self.midi.remove(&note);
                }
                _ => {}
            }

            // Получить следующее midi сообщение, и повторяем.
            next_event = context.next_event();
        }
    }

    pub fn quantity(&self) -> bool {
        self.midi.len() == 0
    }

    pub fn map_active_notes<F>(&self, mut f: F) -> ()
    where
        F: FnMut(u8, Smoother<f32>, usize){
        let mut count = 0;
        for note in self.midi.keys() {
            //Получить ноту и велосити
            let (note, velocity) = self
                .midi
                .get(note)
                .map(|&event| -> (u8, f32) {
                    match event {
                        NoteEvent::NoteOn { note, velocity, .. } => (note, velocity),
                        _ => todo!(),
                    }
                })
                .unwrap();
            //Сглаживание 
            let note_gain: Smoother<f32> = Smoother::default();
            note_gain.set_target(self.sample_rate, velocity);
            
            f(note, note_gain, count);
            count+=1;
        }
    }

}

