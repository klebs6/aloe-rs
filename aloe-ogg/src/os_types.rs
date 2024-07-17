/*!
 function: Define a consistent set of types on
 each platform.
*/

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_formats/codecs/oggvorbis/os_types.h]

/**
  | make it easy on the folks that want to
  | compile the libs with a different malloc
  | than stdlib
  |
  */
macro_rules! ogg_malloc  { () => { /* malloc */ } }
macro_rules! ogg_calloc  { () => { /* calloc */ } }
macro_rules! ogg_realloc { () => { /* realloc */ } }
macro_rules! ogg_free    { () => { /* free */ } }

