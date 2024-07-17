crate::ix!();

pub union CommandReceiverU { 
    data: [u8; size_of::<usize>()],
    len:  usize,
}

pub trait CommandReceiverResponder
{
    fn handle_command(
        &mut self, 
        cmd:   &String,
        param: &Var
    );

    fn receiver_had_error(&mut self);
}

//-------------------------------------
pub struct CommandReceiver {

    responder:        *mut dyn CommandReceiverResponder, // default = nullptr
    in_channel:       i32, // default = 0
    pos:              usize, // default = 0
    receiving_length: bool, // default = true
    buffer_length:    command_receiver::CommandReceiverU,
    buffer:           HeapBlock<u8>,
}

impl CommandReceiver {

    pub fn new(
        responder_to_use:     *mut dyn CommandReceiverResponder,
        input_channel_to_use: i32) -> Self {
    
        todo!();
        /*
        : responder(responderToUse),
        : in_channel(inputChannelToUse),

            setBlocking (inChannel, false);
        */
    }
    
    pub fn set_blocking(
        fd:           i32,
        should_block: bool)  {
        
        todo!();
        /*
            auto flags = fcntl (fd, F_GETFL);
            fcntl (fd, F_SETFL, (shouldBlock ? (flags & ~O_NONBLOCK)
                                             : (flags | O_NONBLOCK)));
        */
    }
    
    pub fn get_fd(&self) -> i32 {
        
        todo!();
        /*
            return inChannel;
        */
    }
    
    pub fn try_next_read(&mut self)  {
        
        todo!();
        /*
            for (;;)
            {
                auto len = (receivingLength ? sizeof (size_t) : bufferLength.len);

                if (! receivingLength)
                    buffer.realloc (len);

                auto* dst = (receivingLength ? bufferLength.data : buffer.getData());

                auto actual = read (inChannel, &dst[pos], static_cast<size_t> (len - pos));

                if (actual < 0)
                {
                    if (errno == EINTR)
                        continue;

                    break;
                }

                pos += static_cast<size_t> (actual);

                if (pos == len)
                {
                    pos = 0;

                    if (! receivingLength)
                        parseJSON (String (buffer.getData(), bufferLength.len));

                    receivingLength = (! receivingLength);
                }
            }

            if (errno != EAGAIN && errno != EWOULDBLOCK && responder != nullptr)
                responder->receiverHadError();
        */
    }
    
    pub fn send_command(
        out_channel: i32,
        cmd:         &String,
        params:      &Var)  {
        
        todo!();
        /*
            DynamicObject::Ptr obj = new DynamicObject;

            obj->setProperty (getCmdIdentifier(), cmd);

            if (! params.isVoid())
                obj->setProperty (getParamIdentifier(), params);

            auto json = JSON::toString (var (obj.get()));

            auto jsonLength = static_cast<size_t> (json.length());
            auto len        = sizeof (size_t) + jsonLength;

            HeapBlock<char> buffer (len);
            auto* dst = buffer.getData();

            memcpy (dst, &jsonLength, sizeof (size_t));
            dst += sizeof (size_t);

            memcpy (dst, json.toRawUTF8(), jsonLength);

            ssize_t ret;

            for (;;)
            {
                ret = write (outChannel, buffer.getData(), len);

                if (ret != -1 || errno != EINTR)
                    break;
            }
        */
    }
    
    pub fn parsejson(&mut self, json: &String)  {
        
        todo!();
        /*
            auto object = JSON::fromString (json);

            if (! object.isVoid())
            {
                auto cmd    = object.getProperty (getCmdIdentifier(),   {}).toString();
                auto params = object.getProperty (getParamIdentifier(), {});

                if (responder != nullptr)
                    responder->handleCommand (cmd, params);
            }
        */
    }
    
    pub fn get_cmd_identifier() -> Identifier {
        
        todo!();
        /*
            static Identifier Id ("cmd");    return Id;
        */
    }
    
    pub fn get_param_identifier() -> Identifier {
        
        todo!();
        /*
            static Identifier Id ("params"); return Id;
        */
    }
}
