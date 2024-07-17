crate::ix!();

pub struct MidiFileHeaderDetails
{
    bytes_read:       usize, // default = 0
    time_format:      i16, // default = 0
    file_type:        i16, // default = 0
    number_of_tracks: i16, // default = 0
}

pub fn parse_midi_header(
    initial_data: *const u8,
    max_size:     usize

) -> Option<MidiFileHeaderDetails> {
    
    todo!();
    /*
        auto* data = initialData;
            auto remaining = maxSize;

            auto ch = tryRead<uint32> (data, remaining);

            if (! ch.valid)
                return {};

            if (ch.value != ByteOrder::bigEndianInt ("MThd"))
            {
                auto ok = false;

                if (ch.value == ByteOrder::bigEndianInt ("RIFF"))
                {
                    for (int i = 0; i < 8; ++i)
                    {
                        ch = tryRead<uint32> (data, remaining);

                        if (! ch.valid)
                            return {};

                        if (ch.value == ByteOrder::bigEndianInt ("MThd"))
                        {
                            ok = true;
                            break;
                        }
                    }
                }

                if (! ok)
                    return {};
            }

            const auto bytesRemaining = tryRead<uint32> (data, remaining);

            if (! bytesRemaining.valid || bytesRemaining.value > remaining)
                return {};

            const auto optFileType = tryRead<uint16> (data, remaining);

            if (! optFileType.valid || 2 < optFileType.value)
                return {};

            const auto optNumTracks = tryRead<uint16> (data, remaining);

            if (! optNumTracks.valid || (optFileType.value == 0 && optNumTracks.value != 1))
                return {};

            const auto optTimeFormat = tryRead<uint16> (data, remaining);

            if (! optTimeFormat.valid)
                return {};

            MidiFileHeaderDetails result;

            result.fileType = (short) optFileType.value;
            result.timeFormat = (short) optTimeFormat.value;
            result.numberOfTracks = (short) optNumTracks.value;
            result.bytesRead = maxSize - remaining;

            return { result };
    */
}
