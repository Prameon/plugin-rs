use nih_plug::prelude::*;
use crate::plugin::App;

//Clap (+Plugin)
impl ClapPlugin for App {
    const CLAP_ID: &'static str = "com.prameon.synth";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Tutorial");
    const CLAP_MANUAL_URL: Option<&'static str> = Some("https://www.youtube.com/c/prameon/");
    const CLAP_SUPPORT_URL: Option<&'static str> = Some("https://t.me/rust_lung");
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Stereo];
    const CLAP_POLY_MODULATION_CONFIG: Option<PolyModulationConfig> = None;
}
nih_export_clap!(App);