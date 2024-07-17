crate::ix!();

pub trait UpdateTrackProperties {

    /**
      | Informs the AudioProcessor that track
      | properties such as the track's name
      | or colour has been changed.
      | 
      | If you are hosting this AudioProcessor
      | then use this method to inform the
      | 
      | AudioProcessor about which track the
      | AudioProcessor is loaded on. This method
      | may only be called on the message thread.
      | 
      | If you are implementing an AudioProcessor
      | then you can override this callback
      | to do something useful with the track
      | properties such as changing the colour
      | of your AudioProcessor's editor. It's
      | entirely up to the host when and how often
      | this callback will be called.
      | 
      | The default implementation of this
      | callback will do nothing.
      |
      */
    fn update_track_properties(&mut self, properties: &AudioProcessorTrackProperties);
}
