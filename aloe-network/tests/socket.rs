crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct SocketTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for SocketTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Sockets", UnitTestCategories::networking
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl SocketTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            auto localHost = IPAddress::local();
            int portNum = 12345;

            beginTest ("StreamingSocket");
            {
                StreamingSocket socketServer;

                expect (socketServer.isConnected() == false);
                expect (socketServer.getHostName().isEmpty());
                expect (socketServer.getBoundPort() == -1);
                expect (static_cast<SocketHandle> (socketServer.getRawSocketHandle()) == invalidSocket);

                expect (socketServer.createListener (portNum, localHost.toString()));

                StreamingSocket socket;

                expect (socket.connect (localHost.toString(), portNum));

                expect (socket.isConnected() == true);
                expect (socket.getHostName() == localHost.toString());
                expect (socket.getBoundPort() != -1);
                expect (static_cast<SocketHandle> (socket.getRawSocketHandle()) != invalidSocket);

                socket.close();

                expect (socket.isConnected() == false);
                expect (socket.getHostName().isEmpty());
                expect (socket.getBoundPort() == -1);
                expect (static_cast<SocketHandle> (socket.getRawSocketHandle()) == invalidSocket);
            }

            beginTest ("DatagramSocket");
            {
                DatagramSocket socket;

                expect (socket.getBoundPort() == -1);
                expect (static_cast<SocketHandle> (socket.getRawSocketHandle()) != invalidSocket);

                expect (socket.bindToPort (portNum, localHost.toString()));

                expect (socket.getBoundPort() == portNum);
                expect (static_cast<SocketHandle> (socket.getRawSocketHandle()) != invalidSocket);

                socket.shutdown();

                expect (socket.getBoundPort() == -1);
                expect (static_cast<SocketHandle> (socket.getRawSocketHandle()) == invalidSocket);
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static SocketTests socketTests;
    */
}
