/*!
  | Component Types used as subCategories
  | in PClassInfo2
  |
  */
crate::ix!();

/**
  | \defgroup plugType Plug-in Type used
  | for subCategories
  |
  */
pub const PLUG_TYPE_FX_ANALYZER:              &'static str = "Fx|Analyzer";              // /< Scope, FFT-Display, Loudness Processing...
pub const PLUG_TYPE_FX_DELAY:                 &'static str = "Fx|Delay";                 // /< Delay, Multi-tap Delay, Ping-Pong Delay...
pub const PLUG_TYPE_FX_DISTORTION:            &'static str = "Fx|Distortion";            // /< Amp Simulator, Sub-Harmonic, SoftClipper...
pub const PLUG_TYPE_FX_DYNAMICS:              &'static str = "Fx|Dynamics";              // /< Compressor, Expander, Gate, Limiter, Maximizer, Tape Simulator, EnvelopeShaper...
pub const PLUG_TYPE_FXEQ:                     &'static str = "Fx|EQ";                    // /< Equalization, Graphical EQ...
pub const PLUG_TYPE_FX_FILTER:                &'static str = "Fx|Filter";                // /< WahWah, ToneBooster, Specific Filter,...
pub const PLUG_TYPE_FX:                       &'static str = "Fx";                       // /< others type (not categorized)
pub const PLUG_TYPE_FX_INSTRUMENT:            &'static str = "Fx|Instrument";            // /< Fx which could be loaded as Instrument too
pub const PLUG_TYPE_FX_INSTRUMENT_EXTERNAL:   &'static str = "Fx|Instrument|External";   // /< Fx which could be loaded as Instrument too and is external (wrapped Hardware)
pub const PLUG_TYPE_FX_SPATIAL:               &'static str = "Fx|Spatial";               // /< MonoToStereo, StereoEnhancer,...
pub const PLUG_TYPE_FX_GENERATOR:             &'static str = "Fx|Generator";             // /< Tone Generator, Noise Generator...
pub const PLUG_TYPE_FX_MASTERING:             &'static str = "Fx|Mastering";             // /< Dither, Noise Shaping,...
pub const PLUG_TYPE_FX_MODULATION:            &'static str = "Fx|Modulation";            // /< Phaser, Flanger, Chorus, Tremolo, Vibrato, AutoPan, Rotary, Cloner...
pub const PLUG_TYPE_FX_PITCH_SHIFT:           &'static str = "Fx|Pitch Shift";           // /< Pitch Processing, Pitch Correction, Vocal Tuning...
pub const PLUG_TYPE_FX_RESTORATION:           &'static str = "Fx|Restoration";           // /< Denoiser, Declicker,...
pub const PLUG_TYPE_FX_REVERB:                &'static str = "Fx|Reverb";                // /< Reverberation, Room Simulation, Convolution Reverb...
pub const PLUG_TYPE_FX_SURROUND:              &'static str = "Fx|Surround";              // /< dedicated to surround processing: LFE Splitter, Bass Manager...
pub const PLUG_TYPE_FX_TOOLS:                 &'static str = "Fx|Tools";                 // /< Volume, Mixer, Tuner...
pub const PLUG_TYPE_FX_NETWORK:               &'static str = "Fx|Network";               // /< using Network
pub const PLUG_TYPE_INSTRUMENT:               &'static str = "Instrument";               // /< Effect used as instrument (sound generator), not as insert
pub const PLUG_TYPE_INSTRUMENT_DRUM:          &'static str = "Instrument|Drum";          // /< Instrument for Drum sounds
pub const PLUG_TYPE_INSTRUMENT_EXTERNAL:      &'static str = "Instrument|External";      // /< External Instrument (wrapped Hardware)
pub const PLUG_TYPE_INSTRUMENT_PIANO:         &'static str = "Instrument|Piano";         // /< Instrument for Piano sounds
pub const PLUG_TYPE_INSTRUMENT_SAMPLER:       &'static str = "Instrument|Sampler";       // /< Instrument based on Samples
pub const PLUG_TYPE_INSTRUMENT_SYNTH:         &'static str = "Instrument|Synth";         // /< Instrument based on Synthesis
pub const PLUG_TYPE_INSTRUMENT_SYNTH_SAMPLER: &'static str = "Instrument|Synth|Sampler"; // /< Instrument based on Synthesis and Samples
pub const PLUG_TYPE_SPATIAL:                  &'static str = "Spatial";                  // /< used for SurroundPanner
pub const PLUG_TYPE_SPATIAL_FX:               &'static str = "Spatial|Fx";               // /< used for SurroundPanner and as insert effect
pub const PLUG_TYPE_ONLY_REAL_TIME:           &'static str = "OnlyRT";                   // /< indicates that it supports only realtime process call, no processing faster than realtime
pub const PLUG_TYPE_ONLY_OFFLINE_PROCESS:     &'static str = "OnlyOfflineProcess";       // /< used for plug-in offline processing  (will not work as normal insert plug-in)
pub const PLUG_TYPE_ONLYARA:                  &'static str = "OnlyARA";                  // /< used for plug-ins that require ARA to operate (will not work as normal insert plug-in)
pub const PLUG_TYPE_NO_OFFLINE_PROCESS:       &'static str = "NoOfflineProcess";         // /< will be NOT used for plug-in offline processing (will work as normal insert plug-in)
pub const PLUG_TYPE_UP_DOWN_MIX:              &'static str = "Up-Downmix";               // /< used for Mixconverter/Up-Mixer/Down-Mixer
pub const PLUG_TYPE_ANALYZER:                 &'static str = "Analyzer";                 // /< Meter, Scope, FFT-Display, not selectable as insert plug-in
pub const PLUG_TYPE_AMBISONICS:               &'static str = "Ambisonics";               // /< used for Ambisonics channel (FX or Panner/Mixconverter/Up-Mixer/Down-Mixer when combined with other category)
pub const PLUG_TYPE_MONO:                     &'static str = "Mono";                     // /< used for Mono only plug-in [optional]
pub const PLUG_TYPE_STEREO:                   &'static str = "Stereo";                   // /< used for Stereo only plug-in [optional]
pub const PLUG_TYPE_SURROUND:                 &'static str = "Surround";                 // /< used for Surround only plug-in [optional]
