crate::ix!();

/**
  | Base class for some extra functions
  | that can be attached to a VST plugin instance.
  |
  */
pub trait VstPluginFormatExtraFunctions
{
    /**
      | This should return 10000 * the BPM at
      | this position in the current edit.
      |
      */
    fn get_tempo_at(&mut self, sample_pos: i64) -> i64;

    /**
      | This should return the host's automation
      | state.
      | 
      | 
      | -----------
      | @return
      | 
      | 0 = not supported, 1 = off, 2 = read, 3 =
      | write, 4 = read/write
      |
      */
    fn get_automation_state(&mut self) -> i32;
}
