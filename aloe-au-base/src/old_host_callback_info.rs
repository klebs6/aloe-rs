crate::ix!();

/**
  | This is for Aloe, added as a workaround
  | for problems in older hosts following
  | an API change.
  |
  */
pub struct OldHostCallbackInfo
{
    host_user_data:             *mut c_void,
    beat_and_tempo_proc:        HostCallback_GetBeatAndTempo,
    musical_time_location_proc: HostCallback_GetMusicalTimeLocation,
    transport_state_proc:       HostCallback_GetTransportState,
}

