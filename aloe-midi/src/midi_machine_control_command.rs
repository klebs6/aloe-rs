crate::ix!();

/** 
  | Types of MMC command.
  |
  | @see isMidiMachineControlMessage,
  | getMidiMachineControlCommand,
  | midiMachineControlCommand
  */
pub enum MidiMachineControlCommand
{
    mmc_stop            = 1,
    mmc_play            = 2,
    mmc_deferredplay    = 3,
    mmc_fastforward     = 4,
    mmc_rewind          = 5,
    mmc_recordStart     = 6,
    mmc_recordStop      = 7,
    mmc_pause           = 9
}

