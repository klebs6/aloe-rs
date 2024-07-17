crate::ix!();

/// A trait defining the contract for identifying 'track' meta-events within a sequence of MIDI events.
///
/// This trait is intended for classes that encapsulate or interact with MIDI events. In the MIDI protocol,
/// meta-events convey non-musical data pertinent to the track itself, rather than musical instructions for
/// an instrument. Implementors of this trait should be capable of discerning whether a given event is a
/// 'track' meta-event, as distinguished from other types of MIDI events.
///
/// # Examples
///
/// ```rust
/// struct MidiEvent;
///
/// impl IsTrackMetaEvent for MidiEvent {
///     fn is_track_meta_event(&self) -> bool {
///         // Implement logic to determine if this is a 'track' meta-event
///         false
///     }
/// }
/// ```
///
pub trait IsTrackMetaEvent {
    /// Determines whether the instance corresponds to a 'track' meta-event in a MIDI sequence.
    ///
    /// This method should return `true` if the object represents a 'track' meta-event, which
    /// usually contains metadata pertinent to the track, such as track name or track settings.
    /// Otherwise, it should return `false`.
    ///
    /// # Returns
    ///
    /// * `bool`: A boolean value indicating whether the object represents a 'track' meta-event.
    fn is_track_meta_event(&self) -> bool;
}

