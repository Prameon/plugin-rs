use nih_plug::prelude::*;

#[derive(Params)]
pub struct Parameters {
    
    #[id = "master_gain"]
    pub master_gain: FloatParam,

    #[id = "bypass"]
    pub bypass: BoolParam,

    #[id = "freq"]
    pub frequency: FloatParam,
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

            // 3 Frequency
            frequency: FloatParam::new(
                "Frequency",
                440.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            // We purposely don't specify a step size here, but the parameter should still be
            // displayed as if it were rounded. This formatter also includes the unit.
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(0))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
        }
    }
}

