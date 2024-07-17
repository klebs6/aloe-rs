crate::ix!();

#[cfg(ALOE_DEBUG)]
#[derive(Default)]
pub struct OpenStreamCounter
{
    num_open_streams: i32, // default = 0
}

#[cfg(ALOE_DEBUG)]
impl Drop for OpenStreamCounter {

    fn drop(&mut self) {
        todo!();
        /* 
        /* If you hit this assertion, it means you've created a stream to read one of the items in the
           zipfile, but you've forgotten to delete that stream object before deleting the file..
           Streams can't be kept open after the file is deleted because they need to share the input
           stream that is managed by the ZipFile object.
        */
        jassert (numOpenStreams == 0);
 */
    }
}
