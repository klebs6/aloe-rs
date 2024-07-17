crate::ix!();

/**
  | This is an AudioTransportSource which
  | will own it's assigned source
  |
  */
#[no_copy]
#[leak_detector]
pub struct AudioSourceOwningTransportSource<'a> {
    base:   AudioTransportSource<'a>,
    source: Box<dyn PositionableAudioSource>,
}

impl<'a> Drop for AudioSourceOwningTransportSource<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            setSource (nullptr);
         */
    }
}

impl<'a> AudioSourceOwningTransportSource<'a> {

    pub fn new(
        s:  *mut dyn PositionableAudioSource,
        sr: f64) -> Self {
    
        todo!();
        /*
        : source(s),

            AudioTransportSource::setSource (s, 0, nullptr, sr);
        */
    }
}
