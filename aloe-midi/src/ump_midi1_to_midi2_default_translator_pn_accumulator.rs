crate::ix!();

pub struct UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnAccumulator {
    bytes: [u8; 4],
    index: u8,     // default = 0
    kind:  UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnKind, // default = UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnKind::nrpn
}

impl UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnAccumulator {

    pub fn get_bytes(&self) -> &[u8; 4] {
        
        todo!();
        /*
            return bytes;
        */
    }
    
    pub fn get_kind(&self) -> UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnKind {
        
        todo!();
        /*
            return kind;
        */
    }
    
    pub fn add_byte(&mut self, cc: u8, byte: u8) -> bool {
        
        todo!();
        /*
            const auto isStart = cc == 99 || cc == 101;

        if (isStart)
        {
            kind = cc == 99 ? UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnKind::nrpn : UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnKind::rpn;
            index = 0;
        }

        bytes[index] = byte;

        const auto shouldContinue = [&]
        {
            switch (index)
            {
                case 0: return isStart;
                case 1: return kind == UniversalMidiPacketsMidi1ToMidi2DefaultTranslatorPnKind::nrpn ? cc == 98 : cc == 100;
                case 2: return cc == 6;
                case 3: return cc == 38;
            }

            return false;
        }();

        index = shouldContinue ? index + 1 : 0;

        if (index != bytes.size())
            return false;

        index = 0;
        return true;
        */
    }
}
