crate::ix!();

pub trait SupportsHostMidiControllerPresence {

    fn supports_host_midi_controller_presence(&mut self, _0: bool) -> bool {
        true
    }
}

pub trait HostMidiControllerIsAvailable {

    fn host_midi_controller_is_available(&mut self, _0: bool) {}
}
