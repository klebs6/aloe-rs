crate::ix!();

pub struct MidiRPNDetectorChannelState
{
    parametermsb: u8,
    parameterlsb: u8,
    valuemsb:     u8,
    valuelsb:     u8,
    isnrpn:       bool,
}

impl Default for MidiRPNDetectorChannelState {

    fn default() -> Self {
    
        todo!();
        /*
            : parameterMSB (0xff), parameterLSB (0xff), valueMSB (0xff), valueLSB (0xff), isNRPN (false)
        */
    }
}

impl MidiRPNDetectorChannelState {
    
    pub fn handle_controller(&mut self, 
        channel:           i32,
        controller_number: i32,
        value:             i32,
        result:            &mut MidiRPNMessage) -> bool {
        
        todo!();
        /*
            switch (controllerNumber)
        {
            case 0x62:  parameterLSB = uint8 (value); resetValue(); isNRPN = true;  break;
            case 0x63:  parameterMSB = uint8 (value); resetValue(); isNRPN = true;  break;

            case 0x64:  parameterLSB = uint8 (value); resetValue(); isNRPN = false; break;
            case 0x65:  parameterMSB = uint8 (value); resetValue(); isNRPN = false; break;

            case 0x06:  valueMSB = uint8 (value); return sendIfReady (channel, result);
            case 0x26:  valueLSB = uint8 (value); break;

            default:  break;
        }

        return false;
        */
    }
    
    pub fn reset_value(&mut self)  {
        
        todo!();
        /*
            valueMSB = 0xff;
        valueLSB = 0xff;
        */
    }
    
    pub fn send_if_ready(&mut self, 
        channel: i32,
        result:  &mut MidiRPNMessage) -> bool {
        
        todo!();
        /*
            if (parameterMSB < 0x80 && parameterLSB < 0x80)
        {
            if (valueMSB < 0x80)
            {
                result.channel = channel;
                result.parameterNumber = (parameterMSB << 7) + parameterLSB;
                result.isNRPN = isNRPN;

                if (valueLSB < 0x80)
                {
                    result.value = (valueMSB << 7) + valueLSB;
                    result.is14BitValue = true;
                }
                else
                {
                    result.value = valueMSB;
                    result.is14BitValue = false;
                }

                return true;
            }
        }

        return false;
        */
    }
}
