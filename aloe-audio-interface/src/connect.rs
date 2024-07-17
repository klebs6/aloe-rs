crate::ix!();

pub trait AddConnection {

    /**
      | Attempts to connect two specified channels
      | of two nodes.
      | 
      | If this isn't allowed (e.g. because
      | you're trying to connect a midi channel
      | to an audio one or other such nonsense),
      | then it'll return false.
      |
      */
    fn add_connection(&mut self, _0: &AudioProcessorGraphConnection) -> bool;
}

