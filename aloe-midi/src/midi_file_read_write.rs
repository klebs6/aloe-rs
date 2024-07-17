crate::ix!();

pub fn midi_file_write_variable_length_int<W: Write>(
    out: &mut W,
    v:   u32

) {
    
    todo!();
    /*
        auto buffer = v & 0x7f;

            while ((v >>= 7) != 0)
            {
                buffer <<= 8;
                buffer |= ((v & 0x7f) | 0x80);
            }

            for (;;)
            {
                out.writeByte ((char) buffer);

                if (buffer & 0x80)
                    buffer >>= 8;
                else
                    break;
            }
    */
}

lazy_static!{
    /*
    template <typename Integral>
        struct ReadTrait;

        template <>
        struct ReadTrait<uint32> { static constexpr auto read = ByteOrder::bigEndianInt; };

        template <>
        struct ReadTrait<uint16> { static constexpr auto read = ByteOrder::bigEndianShort; };
    */
}

pub fn midi_file_try_read<Integral>(
    data:      &mut *const u8,
    remaining: &mut usize

) -> Option<Integral> {

    todo!();
    /*
        using Trait = ReadTrait<Integral>;
            constexpr auto size = sizeof (Integral);

            if (remaining < size)
                return {};

            const Optional<Integral> result { Trait::read (data) };

            data += size;
            remaining -= size;

            return result;
    */
}

pub fn midi_file_read_track(
    data: *const u8,
    size: i32

) -> MidiMessageSequence {

    todo!();
    /*
        double time = 0;
            uint8 lastStatusByte = 0;

            MidiMessageSequence result;

            while (size > 0)
            {
                const auto delay = MidiMessage::readVariableLengthValue (data, (int) size);

                if (! delay.isValid())
                    break;

                data += delay.bytesUsed;
                size -= delay.bytesUsed;
                time += delay.value;

                if (size <= 0)
                    break;

                int messSize = 0;
                const MidiMessage mm (data, size, messSize, lastStatusByte, time);

                if (messSize <= 0)
                    break;

                size -= messSize;
                data += messSize;

                result.addEvent (mm);

                auto firstByte = *(mm.getRawData());

                if ((firstByte & 0xf0) != 0xf0)
                    lastStatusByte = firstByte;
            }

            return result;
    */
}
