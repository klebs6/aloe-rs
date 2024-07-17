crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/midi/ump/aloe_UMPacket.h]

pub type UniversalMidiPacketX1<'a> = UniversalMidiPacket<'a,1>;
pub type UniversalMidiPacketX2<'a> = UniversalMidiPacket<'a,2>;
pub type UniversalMidiPacketX3<'a> = UniversalMidiPacket<'a,3>;
pub type UniversalMidiPacketX4<'a> = UniversalMidiPacket<'a,4>;

pub trait PacketTypes<'a> {
    type Contents;
    type ConstIterator: Iterator<Item = &'a u32> + 'a;
}

/**
  | Holds a single Universal MIDI UniversalMidiPacket.
  | 
  | @tags{Audio}
  |
  */
pub struct UniversalMidiPacket<'a, const UNIVERSAL_MIDI_PACKET_NUM_WORDS: usize> {
    contents: <Self as PacketTypes<'a>>::Contents, // default = {} 
}

impl<'a, const UNIVERSAL_MIDI_PACKET_NUM_WORDS: usize> 
Default for UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> 
{
    fn default() -> Self {
        todo!();
    }
}

impl<'a, const UNIVERSAL_MIDI_PACKET_NUM_WORDS: usize> 
PacketTypes<'a> for 
UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> 
{
    type Contents = [u32; UNIVERSAL_MIDI_PACKET_NUM_WORDS];
    type ConstIterator = std::slice::Iter<'a, u32>;
}

impl<'a, const UNIVERSAL_MIDI_PACKET_NUM_WORDS: usize> Index<usize> 
for UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {

    type Output = u32;
    
    #[inline] fn index(&self, index: usize) -> &Self::Output {
        todo!();
        /*
            return contents[index];
        */
    }
}

impl<'a> UniversalMidiPacket<'a, 1> {
    pub fn new(a: u32) -> Self {
    
        todo!();
        /*


            : contents { { a } }
                jassert (Utils::getNumWordsForMessageType (a) == 1);
        */
    }
}

impl<'a> UniversalMidiPacket<'a, 2> {
    pub fn new(a: u32, b: u32) -> Self {
    
        todo!();
        /*


            : contents { { a, b } }
                jassert (Utils::getNumWordsForMessageType (a) == 2);
        */
    }
}

impl<'a> UniversalMidiPacket<'a, 3> {
    pub fn new(
        a: u32,
        b: u32,
        c: u32) -> Self {
    
        todo!();
        /*


            : contents { { a, b, c } }
                jassert (Utils::getNumWordsForMessageType (a) == 3);
        */
    }
}

impl<'a> UniversalMidiPacket<'a, 4> {
    pub fn new(
        a: u32,
        b: u32,
        c: u32,
        d: u32) -> Self {
    
        todo!();
        /*


            : contents { { a, b, c, d } }
                jassert (Utils::getNumWordsForMessageType (a) == 4);
        */
    }
}

impl<'a, const UNIVERSAL_MIDI_PACKET_NUM_WORDS: usize> UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {
    
    pub fn new_with_full_packet(
        full_packet: &[u32; UNIVERSAL_MIDI_PACKET_NUM_WORDS]

    ) -> Self {

        todo!();
        /*
           : contents (fullPacket)
           jassert (Utils::getNumWordsForMessageType (fullPacket.front()) == UNIVERSAL_MIDI_PACKET_NUM_WORDS);
           */
    }
    
    pub fn with_message_type(&self, ty: u8) -> UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {
        
        todo!();
        /*
            return withU4<0> (type);
        */
    }
    
    pub fn with_group(&self, group: u8) -> UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {
        
        todo!();
        /*
            return withU4<1> (group);
        */
    }
    
    pub fn with_status(&self, status: u8) -> UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {
        
        todo!();
        /*
            return withU4<2> (status);
        */
    }
    
    pub fn with_channel(&self, channel: u8) -> UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {
        
        todo!();
        /*
            return withU4<3> (channel);
        */
    }
    
    pub fn get_message_type(&self) -> u8 {
        
        todo!();
        /*
            return getU4<0>();
        */
    }
    
    pub fn get_group(&self) -> u8 {
        
        todo!();
        /*
            return getU4<1>();
        */
    }
    
    pub fn get_status(&self) -> u8 {
        
        todo!();
        /*
            return getU4<2>();
        */
    }
    
    pub fn get_channel(&self) -> u8 {
        
        todo!();
        /*
            return getU4<3>();
        */
    }
    
    pub fn withu4<const index: usize>(&self, value: u8) -> UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {
    
        todo!();
        /*
            constexpr auto word = index / 8;
                auto copy = *this;
                std::get<word> (copy.contents) = Utils::U4<index % 8>::set (copy.template getU32<word>(), value);
                return copy;
        */
    }
    
    pub fn withu8<const index: usize>(&self, value: u8) -> UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {
    
        todo!();
        /*
            constexpr auto word = index / 4;
                auto copy = *this;
                std::get<word> (copy.contents) = Utils::U8<index % 4>::set (copy.template getU32<word>(), value);
                return copy;
        */
    }
    
    pub fn withu16<const index: usize>(&self, value: u16) -> UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {
    
        todo!();
        /*
            constexpr auto word = index / 2;
                auto copy = *this;
                std::get<word> (copy.contents) = Utils::U16<index % 2>::set (copy.template getU32<word>(), value);
                return copy;
        */
    }
    
    pub fn withu32<const index: usize>(&self, value: u32) -> UniversalMidiPacket<'a, UNIVERSAL_MIDI_PACKET_NUM_WORDS> {
    
        todo!();
        /*
            auto copy = *this;
                std::get<index> (copy.contents) = value;
                return copy;
        */
    }
    
    pub fn getu4<const index: usize>(&self) -> u8 {
    
        todo!();
        /*
            return Utils::U4<index % 8>::get (this->template getU32<index / 8>());
        */
    }
    
    pub fn getu8<const index: usize>(&self) -> u8 {
    
        todo!();
        /*
            return Utils::U8<index % 4>::get (this->template getU32<index / 4>());
        */
    }
    
    pub fn getu16<const index: usize>(&self) -> u16 {
    
        todo!();
        /*
            return Utils::U16<index % 2>::get (this->template getU32<index / 2>());
        */
    }
    
    pub fn getu32<const index: usize>(&self) -> u32 {
    
        todo!();
        /*
            return std::get<index> (contents);
            */
    }

    // Gets an iterator over immutable references
    pub fn iter(&self) -> std::slice::Iter<u32> {
        self.contents.iter()
    }

    // Gets an iterator over mutable references
    pub fn iter_mut(&mut self) -> std::slice::IterMut<u32> {
        self.contents.iter_mut()
    }

    pub fn data(&self) -> *const u32 {
        
        todo!();
        /*
            return contents.data();
        */
    }
    
    pub fn front(&self) -> &u32 {
        
        todo!();
        /*
            return contents.front();
        */
    }
    
    pub fn back(&self) -> &u32 {
        
        todo!();
        /*
            return contents.back();
        */
    }
}
