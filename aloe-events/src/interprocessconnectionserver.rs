crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/interprocess/aloe_InterprocessConnectionServer.h]

/**
  | An object that waits for client sockets
  | to connect to a port on this host, and
  | creates InterprocessConnection objects
  | for each one.
  | 
  | To use this, create a class derived from
  | it which implements the createConnectionObject()
  | method, so that it creates suitable
  | connection objects for each client
  | that tries to connect.
  | 
  | @see InterprocessConnection
  | 
  | @tags{Events}
  |
  */
#[no_copy]
#[leak_detector]
pub struct InterprocessConnectionServer {
    base:   Thread,
    socket: Box<StreamingSocket>,
}

pub trait InterprocessConnectionServerInterface {

    /**
      | Creates a suitable connection object
      | for a client process that wants to connect
      | to this one.
      | 
      | This will be called by the listener thread
      | when a client process tries to connect,
      | and must return a new InterprocessConnection
      | object that will then run as this end
      | of the connection.
      | 
      | @see InterprocessConnection
      |
      */
    fn create_connection_object(&mut self) -> *mut InterprocessConnection;
}

impl Drop for InterprocessConnectionServer {

    fn drop(&mut self) {

        todo!();
        /* 
           stop();
           */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/interprocess/aloe_InterprocessConnectionServer.cpp]
impl Default for InterprocessConnectionServer {

    /**
      | Creates an uninitialised server object.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : thread("Aloe IPC server"),
        */
    }
}

impl InterprocessConnectionServer {
    
    /**
      | Starts an internal thread which listens
      | on the given port number.
      | 
      | While this is running, if another process
      | tries to connect with the InterprocessConnection::connectToSocket()
      | method, this object will call createConnectionObject()
      | to create a connection to that client.
      | 
      | Use stop() to stop the thread running.
      | 
      | -----------
      | @param portNumber
      | 
      | The port on which the server will receive
      | connections
      | ----------
      | @param bindAddress
      | 
      | The address on which the server will
      | listen for connections. An empty string
      | indicates that it should listen on all
      | addresses assigned to this machine.
      | 
      | @see createConnectionObject, stop
      |
      */
    pub fn begin_waiting_for_socket(&mut self, 
        port_number:  i32,
        bind_address: Option<&String>) -> bool {

        let bind_address = bind_address.unwrap_or(&String::new());
        
        todo!();
        /*
            stop();

        socket.reset (new StreamingSocket());

        if (socket->createListener (portNumber, bindAddress))
        {
            startThread();
            return true;
        }

        socket.reset();
        return false;
        */
    }
    
    /**
      | Terminates the listener thread, if
      | it's active.
      | 
      | @see beginWaitingForSocket
      |
      */
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            signalThreadShouldExit();

        if (socket != nullptr)
            socket->close();

        stopThread (4000);
        socket.reset();
        */
    }
    
    /**
      | Returns the local port number to which
      | this server is currently bound.
      | 
      | This is useful if you need to know to which
      | port the OS has actually bound your socket
      | when calling beginWaitingForSocket
      | with a port number of zero.
      | 
      | Returns -1 if the function fails.
      |
      */
    pub fn get_bound_port(&self) -> i32 {
        
        todo!();
        /*
            return (socket == nullptr) ? -1 : socket->getBoundPort();
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            while ((! threadShouldExit()) && socket != nullptr)
        {
            std::unique_ptr<StreamingSocket> clientSocket (socket->waitForNextConnection());

            if (clientSocket != nullptr)
                if (auto* newConnection = createConnectionObject())
                    newConnection->initialiseWithSocket (std::move (clientSocket));
        }
        */
    }
}
