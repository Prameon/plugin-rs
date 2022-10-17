use nih_plug::prelude::*;

#[derive(Params)]
pub struct Parameters {
    
    #[id = "master_gain"]
    pub master_gain: FloatParam,

    #[id = "bypass"]
    pub bypass: BoolParam,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            // 1 Master Gain
            master_gain: FloatParam::new(
                "Master Gain",
                -10.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 6.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(3.0))
            .with_step_size(0.01)
            .with_unit(" dB"),

            // 2 Bypass
            bypass: BoolParam::new(
                "Bypass", 
                false
            ),
        }
    }
}

