crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/vsttypes.h]

pub type PointerSizedInt = usize;

/**
  | \defgroup vst3typedef Vst 3 Data Types
  |
  */

/* ----------------- String Types  ----------------- */

pub type TChar     = u16;          // UTF-16 character
pub type String128 = [TChar; 128]; // 128 character UTF-16 string
pub type CString   = *const u8;    // C-String

/* -------------------- General  -------------------- */

pub type MediaType     = i32; // media type (audio/event)
pub type BusDirection  = i32; // bus direction (in/out)
pub type BusType       = i32; // bus type (main/aux)
pub type IoMode        = i32; // I/O mode (see \ref vst3IoMode)
pub type UnitID        = i32; // unit identifier
pub type ParamValue    = f64; // parameter value type
pub type ParamID       = u32; // parameter identifier
pub type ProgramListID = i32; // program list identifier
pub type CtrlNumber    = i16; // MIDI controller number (see \ref ControllerNumbers for allowed values)
pub type TQuarterNotes = f64; // time expressed in quarter notes
pub type TSamples      = i64; // time expressed in audio samples
pub type ColorSpec     = u32; // color defining by 4 component ARGB value (Alpha/Red/Green/Blue)

/**
   default for uninitialized parameter ID
  */
pub const k_no_param_id: ParamID = 0xffffffff; 

// static const ParamID kNoParamId = std::numeric_limits<ParamID>::max ();

/* ------------------ Audio Types  ------------------ */

pub type Sample32           = f32;// 32-bit precision audio sample
pub type Sample64           = f64;// 64-bit precision audio sample
pub type SampleRate         = f64;// sample rate

/* ---------- Speaker Arrangements Types  ---------- */

pub type SpeakerArrangement = u64;// Bitset of speakers
pub type Speaker            = u64;// Bit for one speaker

