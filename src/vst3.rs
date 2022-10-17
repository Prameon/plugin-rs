use nih_plug::prelude::*;
use crate::plugin::App;

//Vst3 (+Plugin)
impl Vst3Plugin for App {
    const VST3_CLASS_ID: [u8; 16] = *b"Synth Sine Plug ";
    const VST3_CATEGORIES: &'static str = "Instrument|Synth|Tools";
}
nih_export_vst3!(App);