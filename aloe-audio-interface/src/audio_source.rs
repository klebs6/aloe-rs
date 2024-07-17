crate::ix!();

/**
  | Base class for objects that can produce
  | a continuous stream of audio.
  |
  | An AudioSource has two states: 'prepared' and
  | 'unprepared'.
  |
  | When a source needs to be played, it is first
  | put into a 'prepared' state by a call to
  | prepareToPlay(), and then repeated calls will
  | be made to its getNextAudioBlock() method to
  | process the audio data.
  |
  | Once playback has finished, the
  | releaseResources() method is called to put the
  | stream back into an 'unprepared' state.
  |
  | @see AudioFormatReaderSource,
  | ResamplingAudioSource
  |
  | @tags{Audio}
  */
pub trait AudioSource

/* 
  | Allows the source to release anything it
  | no longer needs after playback has stopped.
  |
  | This will be called when the source is no
  | longer going to have its
  | getNextAudioBlock() method called, so it
  | should release any spare memory, etc. that
  | it might have allocated during the
  | prepareToPlay() call.
  |
  | Note that there's no guarantee that
  | prepareToPlay() will actually have been
  | called before releaseResources(), and it
  | may be called more than once in
  | succession, so make sure your code is
  | robust and doesn't make any assumptions
  | about when it will be called.
  |
  | @see prepareToPlay, getNextAudioBlock
  */
: ReleaseResources 
+ PrepareToPlayAudioSource
+ GetNextAudioBlock { }
