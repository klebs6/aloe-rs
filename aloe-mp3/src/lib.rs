#[macro_use] mod imports; use imports::*;

/**
  | The below notice is included verbatim from the aloe c++
  | project.
  |
  | Currently, in the rust project, we use the feature "mp3"
  | to uphold this same functionality.
  |
  ------------------
  | IMPORTANT DISCLAIMER: By choosing
  | to enable the ALOE_USE_MP3AUDIOFORMAT
  | flag and to compile this MP3 code into
  | your software, you do so AT YOUR OWN RISK!
  | By doing so, you are agreeing that Raw
  | Material Software Limited is in no way
  | responsible for any patent, copyright,
  | or other legal issues that you may suffer
  | as a result.
  | 
  | The code in aloe_MP3AudioFormat.cpp
  | is NOT guaranteed to be free from infringements
  | of 3rd-party intellectual property.
  | If you wish to use it, please seek your
  | own independent advice about the legality
  | of doing so. If you are not willing to
  | accept full responsibility for the
  | consequences of using this code, then
  | do not enable the ALOE_USE_MP3AUDIOFORMAT
  | setting.
  |
  */
#[cfg(feature = "mp3")] x!{lameencoderaudioformat}
#[cfg(feature = "mp3")] x!{lameencoderaudioformat_writer}
#[cfg(feature = "mp3")] x!{mp3_audio_format}
#[cfg(feature = "mp3")] x!{mp3_decoder_allocation_table}
#[cfg(feature = "mp3")] x!{mp3_decoder_band_info}
#[cfg(feature = "mp3")] x!{mp3_decoder_bits_to_table_map}
#[cfg(feature = "mp3")] x!{mp3_decoder_constants}
#[cfg(feature = "mp3")] x!{mp3_decoder_dct}
#[cfg(feature = "mp3")] x!{mp3_decoder_decode_window}
#[cfg(feature = "mp3")] x!{mp3_decoder_huffman_table}
#[cfg(feature = "mp3")] x!{mp3_decoder_layer3_side_info}
#[cfg(feature = "mp3")] x!{mp3_decoder_vbr_tag_data}
#[cfg(feature = "mp3")] x!{mp3_frame}
#[cfg(feature = "mp3")] x!{mp3_reader}
#[cfg(feature = "mp3")] x!{mp3_stream}
