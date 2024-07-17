crate::ix!();

pub struct ProcessorState
{
    synth_voices:           i32,
    legacy_mode_enabled:    bool,
    legacy_channels:        Range<i32>,
    legacy_pitchbend_range: i32,
    voice_stealing_enabled: bool,
    mpe_zone_layout:        MPEZoneLayout,
    reader_factory:         Box<dyn AudioFormatReaderFactory>,
    loop_points_seconds:    Range<f64>,
    centre_frequency_hz:    f64,
    loop_mode:              LoopMode,
}
