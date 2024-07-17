/*!
  Vst MIDI Controller Enumeration
  */

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstmidicontrollers.h]

crate::ix!();

/**
  | Controller Numbers (MIDI)
  |
  */

pub enum ControllerNumbers
{
    /**
      | Bank Select MSB
      |
      */
    CtrlBankSelectMSB,  

    /**
      | Modulation Wheel
      |
      */
    CtrlModWheel,  

    /**
      | Breath controller
      |
      */
    CtrlBreath,  

    /**
      | Foot Controller
      |
      */
    CtrlFoot,  

    /**
      | Portamento Time
      |
      */
    CtrlPortaTime,  

    /**
      | Data Entry MSB
      |
      */
    CtrlDataEntryMSB,  

    /**
      | Channel Volume (formerly Main Volume)
      |
      */
    CtrlVolume,  

    /**
      | Balance
      |
      */
    CtrlBalance,  

    /**
      | Pan
      |
      */
    CtrlPan, 

    /**
      | Expression
      |
      */
    CtrlExpression, 

    /**
      | Effect Control 1
      |
      */
    CtrlEffect1, 

    /**
      | Effect Control 2
      |
      */
    CtrlEffect2, 

    //---General Purpose Controllers #1 to #4---

    /**
      | General Purpose Controller #1
      |
      */
    CtrlGPC1, 

    /**
      | General Purpose Controller #2
      |
      */
    CtrlGPC2, 

    /**
      | General Purpose Controller #3
      |
      */
    CtrlGPC3, 

    /**
      | General Purpose Controller #4
      |
      */
    CtrlGPC4, 

    /**
      | Bank Select LSB
      |
      */
    CtrlBankSelectLSB, 

    /**
      | Data Entry LSB
      |
      */
    CtrlDataEntryLSB, 

    /**
      | Damper Pedal On/Off (Sustain)
      |
      */
    CtrlSustainOnOff, 

    /**
      | Portamento On/Off
      |
      */
    CtrlPortaOnOff, 

    /**
      | Sustenuto On/Off
      |
      */
    CtrlSustenutoOnOff, 

    /**
      | Soft Pedal On/Off
      |
      */
    CtrlSoftPedalOnOff, 

    /**
      | Legato Footswitch On/Off
      |
      */
    CtrlLegatoFootSwOnOff, 

    /**
      | Hold 2 On/Off
      |
      */
    CtrlHold2OnOff, 

    //---Sound Controllers #1 to #10---

    /**
      | Sound Variation
      |
      */
    CtrlSoundVariation, 

    /**
      | Filter Cutoff (Timbre/Harmonic Intensity)
      |
      */
    CtrlFilterCutoff, 

    /**
      | Release Time
      |
      */
    CtrlReleaseTime, 

    /**
      | Attack Time
      |
      */
    CtrlAttackTime, 

    /**
      | Filter Resonance (Brightness)
      |
      */
    CtrlFilterResonance, 

    /**
      | Decay Time
      |
      */
    CtrlDecayTime, 

    /**
      | Vibrato Rate
      |
      */
    CtrlVibratoRate, 

    /**
      | Vibrato Depth
      |
      */
    CtrlVibratoDepth, 

    /**
      | Vibrato Delay
      |
      */
    CtrlVibratoDelay, 

    /**
      | undefined
      |
      */
    CtrlSoundCtrler10, 

    //---General Purpose Controllers #5 to #8---

    /**
      | General Purpose Controller #5
      |
      */
    CtrlGPC5, 

    /**
      | General Purpose Controller #6
      |
      */
    CtrlGPC6, 

    /**
      | General Purpose Controller #7
      |
      */
    CtrlGPC7, 

    /**
      | General Purpose Controller #8
      |
      */
    CtrlGPC8, 

    /**
      | Portamento Control
      |
      */
    CtrlPortaControl, 

    /*---Effect Controllers---*/

    /**
      | Effect 1 Depth (Reverb Send Level)
      |
      */
    CtrlEff1Depth, 

    /**
      | Effect 2 Depth (Tremolo Level)
      |
      */
    CtrlEff2Depth, 

    /**
      | Effect 3 Depth (Chorus Send Level)
      |
      */
    CtrlEff3Depth, 

    /**
      | Effect 4 Depth (Delay/Variation/Detune
      | Level)
      |
      */
    CtrlEff4Depth, 

    /**
      | Effect 5 Depth (Phaser Level)
      |
      */
    CtrlEff5Depth, 

    /**
      | Data Increment (+1)
      |
      */
    CtrlDataIncrement, 

    /**
      | Data Decrement (-1)
      |
      */
    CtrlDataDecrement, 

    /**
      | NRPN Select LSB
      |
      */
    CtrlNRPNSelectLSB, 

    /**
      | NRPN Select MSB
      |
      */
    CtrlNRPNSelectMSB, 

    /**
      | RPN Select LSB
      |
      */
    CtrlRPNSelectLSB, 

    /**
      | RPN Select MSB
      |
      */
    CtrlRPNSelectMSB, 

    /*---Other Channel Mode Messages---*/

    /**
      | All Sounds Off
      |
      */
    CtrlAllSoundsOff, 

    /**
      | Reset All Controllers
      |
      */
    CtrlResetAllCtrlers, 

    /**
      | Local Control On/Off
      |
      */
    CtrlLocalCtrlOnOff, 

    /**
      | All Notes Off
      |
      */
    CtrlAllNotesOff, 

    /**
      | Omni Mode Off + All Notes Off
      |
      */
    CtrlOmniModeOff, 

    /**
      | Omni Mode On + All Notes Off
      |
      */
    CtrlOmniModeOn, 

    /**
      | Poly Mode On/Off + All Sounds Off
      |
      */
    CtrlPolyModeOnOff, 

    /**
      | Poly Mode On
      |
      */
    CtrlPolyModeOn, 

    /*---Extra--------------------------*/

    /**
      | After Touch (associated to Channel
      | Pressure)
      |
      */
    AfterTouch,          

    /**
      | Pitch Bend Change
      |
      */
    PitchBend,          

    /**
      | Count of Controller Number
      |
      */
    //CountCtrlNumber,           

    /*---Extra for kLegacyMIDICCOutEvent-*/

    /**
      | Program Change (use
      | LegacyMIDICCOutEvent.value only)
      |
      */
    CtrlProgramChange,   

    /**
      | Polyphonic Key Pressure (use
      | LegacyMIDICCOutEvent.value for pitch
      | and LegacyMIDICCOutEvent.value2 for
      | pressure)
      |
      */
    CtrlPolyPressure,   

    /**
      | Quarter Frame ((use
      | LegacyMIDICCOutEvent.value only)
      |
      */
    CtrlQuarterFrame,    
}

impl ControllerNumbers {

    pub const fn count_ctrl_number() -> usize {
        Self::CtrlProgramChange.value()
    }

    pub const fn value(&self) -> usize {

        use ControllerNumbers::*;

        match self {
            CtrlBankSelectMSB     => 0,
            CtrlModWheel          => 1,
            CtrlBreath            => 2,
            CtrlFoot              => 4,
            CtrlPortaTime         => 5,
            CtrlDataEntryMSB      => 6,
            CtrlVolume            => 7,
            CtrlBalance           => 8,
            CtrlPan               => 10,
            CtrlExpression        => 11,
            CtrlEffect1           => 12,
            CtrlEffect2           => 13,
            CtrlGPC1              => 16,
            CtrlGPC2              => 17,
            CtrlGPC3              => 18,
            CtrlGPC4              => 19,
            CtrlBankSelectLSB     => 32,
            CtrlDataEntryLSB      => 38,
            CtrlSustainOnOff      => 64,
            CtrlPortaOnOff        => 65,
            CtrlSustenutoOnOff    => 66,
            CtrlSoftPedalOnOff    => 67,
            CtrlLegatoFootSwOnOff => 68,
            CtrlHold2OnOff        => 69,
            CtrlSoundVariation    => 70,
            CtrlFilterCutoff      => 71,
            CtrlReleaseTime       => 72,
            CtrlAttackTime        => 73,
            CtrlFilterResonance   => 74,
            CtrlDecayTime         => 75,
            CtrlVibratoRate       => 76,
            CtrlVibratoDepth      => 77,
            CtrlVibratoDelay      => 78,
            CtrlSoundCtrler10     => 79,
            CtrlGPC5              => 80,
            CtrlGPC6              => 81,
            CtrlGPC7              => 82,
            CtrlGPC8              => 83,
            CtrlPortaControl      => 84,
            CtrlEff1Depth         => 91,
            CtrlEff2Depth         => 92,
            CtrlEff3Depth         => 93,
            CtrlEff4Depth         => 94,
            CtrlEff5Depth         => 95,
            CtrlDataIncrement     => 96,
            CtrlDataDecrement     => 97,
            CtrlNRPNSelectLSB     => 98,
            CtrlNRPNSelectMSB     => 99,
            CtrlRPNSelectLSB      => 100,
            CtrlRPNSelectMSB      => 101,
            CtrlAllSoundsOff      => 120,
            CtrlResetAllCtrlers   => 121,
            CtrlLocalCtrlOnOff    => 122,
            CtrlAllNotesOff       => 123,
            CtrlOmniModeOff       => 124,
            CtrlOmniModeOn        => 125,
            CtrlPolyModeOnOff     => 126,
            CtrlPolyModeOn        => 127,
            AfterTouch            => 128,
            PitchBend             => 129,
            CtrlProgramChange     => 130,
            CtrlPolyPressure      => 131,
            CtrlQuarterFrame      => 132,
        }
    }
}
