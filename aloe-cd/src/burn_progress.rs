crate::ix!();

/**
  | Receives progress callbacks during
  | a cd-burn operation. @see AudioCDBurner::burn()
  |
  */
pub trait AudioCDBurnerBurnProgressListener {

    /**
      | Called at intervals to report on the
      | progress of the AudioCDBurner.
      | 
      | To cancel the burn, return true from
      | this method.
      |
      */
    fn audio_cd_burn_progress(&mut self, proportion_complete: f32) -> bool;
}
