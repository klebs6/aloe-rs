crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/SamplerPluginDemo.h]

pub mod IDs {
    use super::*;

    macro_rules! declare_id {
        ($name:ident) => {
            /*
                    const Identifier name (#name);
            */
        }
    }

    declare_id!{ DATA_MODEL }
    declare_id!{ sampleReader }
    declare_id!{ centreFrequencyHz }
    declare_id!{ loopMode }
    declare_id!{ loopPointsSeconds }
    declare_id!{ MPE_SETTINGS }
    declare_id!{ synthVoices }
    declare_id!{ voiceStealingEnabled }
    declare_id!{ legacyModeEnabled }
    declare_id!{ mpeZoneLayout }
    declare_id!{ legacyFirstChannel }
    declare_id!{ legacyLastChannel }
    declare_id!{ legacyPitchbendRange }
    declare_id!{ VISIBLE_RANGE }
    declare_id!{ totalRange }
    declare_id!{ visibleRange }
}

/**
  | We want to send type-erased commands to the
  | audio thread, but we also want those commands
  | to contain move-only resources, so that we can
  | construct resources on the gui thread, and then
  | transfer ownership cheaply to the audio
  | thread. We can't do this with std::function
  | because it enforces that functions are
  | copy-constructible.  Therefore, we use a very
  | simple templated type-eraser here.
  */
pub trait Command<Proc> {
    fn run(&mut self, proc: &mut Proc);
}
