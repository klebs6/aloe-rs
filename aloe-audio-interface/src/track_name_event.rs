crate::ix!();

/// A trait specifying the protocol for identifying 'track name' meta-events within a MIDI sequence.
///
/// This trait is geared towards types that manage or interface with MIDI events. Within the scope of the MIDI protocol,
/// 'track name' meta-events serve to encapsulate metadata specifically designating the name of a track. Implementors
/// are expected to differentiate 'track name' meta-events from other MIDI events and can utilize the
/// `getTextFromTextMetaEvent()` method to retrieve the actual track name.
///
/// # Examples
///
/// ```rust
/// struct MidiEvent;
///
/// impl IsTrackNameEvent for MidiEvent {
///     fn is_track_name_event(&self) -> bool {
///         // Implement logic to identify if this constitutes a 'track name' meta-event
///         false
///     }
/// }
/// ```
///
pub trait IsTrackNameEvent {
    /// Ascertains whether the encapsulated object represents a 'track name' meta-event in a MIDI data stream.
    ///
    /// The method should return `true` if the object embodies a 'track name' meta-event, typified by metadata
    /// that specifies the name of the MIDI track. In such cases, the `getTextFromTextMetaEvent()` method can
    /// be employed to extract the actual track name. Otherwise, the method should return `false`.
    ///
    /// # Returns
    ///
    /// * `bool`: A Boolean flag indicating the presence of a 'track name' meta-event.
    fn is_track_name_event(&self) -> bool;
}

