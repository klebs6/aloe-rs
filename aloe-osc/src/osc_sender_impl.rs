crate::ix!();

#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct OSCSenderImpl {
    socket:             OptionalScopedPointer<DatagramSocket>,
    target_host_name:   String,
    target_port_number: i32, // default = 0
}

impl Drop for OSCSenderImpl {

    fn drop(&mut self) {
        todo!();
        /*
            disconnect();
        */
    }
}

impl OSCSenderImpl {
    
    pub fn connect(
        &mut self, 
        new_target_host: &String,
        new_target_port: i32

    ) -> bool {
        
        todo!();
        /*
            if (! disconnect())
                return false;

            socket.setOwned (new DatagramSocket (true));
            targetHostName = newTargetHost;
            targetPortNumber = newTargetPort;

            if (socket->bindToPort (0)) // 0 = use any local port assigned by the OS.
                return true;

            socket.reset();
            return false;
        */
    }
    
    pub fn connect_to_socket(
        &mut self, 
        new_socket:      &mut DatagramSocket,
        new_target_host: &String,
        new_target_port: i32

    ) -> bool {
        
        todo!();
        /*
            if (! disconnect())
                return false;

            socket.setNonOwned (&newSocket);
            targetHostName = newTargetHost;
            targetPortNumber = newTargetPort;
            return true;
        */
    }
    
    pub fn disconnect(&mut self) -> bool {
        
        todo!();
        /*
            socket.reset();
            return true;
        */
    }
    
    pub fn send_osc_message_to_host(
        &mut self, 
        message:     &OSCMessage,
        host_name:   &String,
        port_number: i32

    ) -> bool {
        
        todo!();
        /*
            OSCOutputStream outStream;

            return outStream.writeMessage (message)
                && sendOutputStream (outStream, hostName, portNumber);
        */
    }
    
    pub fn send_bundle_to_host(
        &mut self, 
        bundle:      &OSCBundle,
        host_name:   &String,
        port_number: i32

    ) -> bool {
        
        todo!();
        /*
            OSCOutputStream outStream;

            return outStream.writeBundle (bundle)
                && sendOutputStream (outStream, hostName, portNumber);
        */
    }
    
    pub fn send_osc_message(&mut self, message: &OSCMessage) -> bool {
        
        todo!();
        /*
            return send (message, targetHostName, targetPortNumber);
        */
    }
    
    pub fn send_bundle(&mut self, bundle: &OSCBundle) -> bool {
        
        todo!();
        /*
            return send (bundle,  targetHostName, targetPortNumber);
        */
    }
    
    pub fn send_output_stream(
        &mut self, 
        out_stream:  &mut OSCOutputStream,
        host_name:   &String,
        port_number: i32

    ) -> bool {
        
        todo!();
        /*
            if (socket != nullptr)
            {
                const int streamSize = (int) outStream.getDataSize();

                const int bytesWritten = socket->write (hostName, portNumber,
                                                        outStream.getData(), streamSize);
                return bytesWritten == streamSize;
            }

            // if you hit this, you tried to send some OSC data without being
            // connected to a port! You should call OSCSender::connect() first.
            jassertfalse;

            return false;
        */
    }
}
