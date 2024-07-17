/*!
  | Contains classes that implement a simple
  | protocol for broadcasting the availability
  | and location of a discoverable service
  | on the local network, and for maintaining
  | a list of known services.
  | 
  | @tags{Events}
  |
  */

crate::ix!();

pub fn sort_service_list(services: &mut Vec<NetworkServiceDiscoveryService>)  {
    
    todo!();
    /*
        auto compareServices = [] (const NetworkServiceDiscovery::Service& s1,
                                   const NetworkServiceDiscovery::Service& s2)
        {
            return s1.instanceID < s2.instanceID;
        };

        std::sort (services.begin(), services.end(), compareServices);
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_events/interprocess/aloe_NetworkServiceDiscovery.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_events/interprocess/aloe_NetworkServiceDiscovery.cpp]

/**
  | An object which runs a thread to repeatedly
  | broadcast the existence of a discoverable
  | service.
  | 
  | To use, simply create an instance of
  | an Advertiser and it'll broadcast until
  | you delete it.
  | 
  | @tags{Events}
  |
  */
pub struct NetworkServiceDiscoveryAdvertiser {
    base:           Thread,
    message:        XmlElement,
    broadcast_port: i32,
    min_interval:   RelativeTime,
    socket:         DatagramSocket, // default = true 
}

impl Drop for NetworkServiceDiscoveryAdvertiser {

    fn drop(&mut self) {
        todo!();
        /* 
        stopThread (2000);
        socket.shutdown();
         */
    }
}

impl NetworkServiceDiscoveryAdvertiser {

    /**
      | Creates and starts an Advertiser thread,
      | broadcasting with the given properties.
      | 
      | -----------
      | @param serviceTypeUID
      | 
      | A user-supplied string to define the
      | type of service this represents
      | ----------
      | @param serviceDescription
      | 
      | A description string that will appear
      | in the Service::description field
      | for clients
      | ----------
      | @param broadcastPort
      | 
      | The port number on which to broadcast
      | the service discovery packets
      | ----------
      | @param connectionPort
      | 
      | The port number that will be sent to appear
      | in the Service::port field
      | ----------
      | @param minTimeBetweenBroadcasts
      | 
      | The interval to wait between sending
      | broadcast messages
      |
      */
    pub fn new(
        service_typeuid:             &String,
        service_description:         &String,
        broadcast_port_to_use:       i32,
        connection_port:             i32,
        min_time_between_broadcasts: Option<RelativeTime>

    ) -> Self {

        let min_time_between_broadcasts =
            min_time_between_broadcasts.unwrap_or(RelativeTime::seconds(1.5));
    
        todo!();
        /*


            : Thread ("Discovery_broadcast"),
          message (serviceTypeUID), broadcastPort (broadcastPortToUse),
          minInterval (minTimeBetweenBroadcasts)

        message.setAttribute ("id", Uuid().toString());
        message.setAttribute ("name", serviceDescription);
        message.setAttribute ("address", String());
        message.setAttribute ("port", connectionPort);

        startThread (2);
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            if (! socket.bindToPort (0))
        {
            jassertfalse;
            return;
        }

        while (! threadShouldExit())
        {
            sendBroadcast();
            wait ((int) minInterval.inMilliseconds());
        }
        */
    }
    
    pub fn send_broadcast(&mut self)  {
        
        todo!();
        /*
            static IPAddress local = IPAddress::local();

        for (auto& address : IPAddress::getAllAddresses())
        {
            if (address == local)
                continue;

            message.setAttribute ("address", address.toString());

            auto broadcastAddress = IPAddress::getInterfaceBroadcastAddress (address);
            auto data = message.toString (XmlElement::TextFormat().singleLine().withoutHeader());

            socket.write (broadcastAddress.toString(), broadcastPort, data.toRawUTF8(), (int) data.getNumBytesAsUTF8());
        }
        */
    }
}

/**
  | Contains information about a service
  | that has been found on the network.
  | 
  | @see AvailableServiceList, Advertiser
  | 
  | @tags{Events}
  |
  */
pub struct NetworkServiceDiscoveryService
{
    /**
      | A UUID that identifies the particular
      | instance of the Advertiser class.
      |
      */
    instanceid:  String,

    /**
      | The service description as sent by the
      | Advertiser
      |
      */
    description: String,

    /**
      | The IP address of the advertiser
      |
      */
    address:     IPAddress,

    /**
      | The port number of the advertiser
      |
      */
    port:        i32,

    /**
      | The time of the last ping received from
      | the advertiser
      |
      */
    last_seen:   Time,
}

/**
  | Watches the network for broadcasts
  | from Advertiser objects, and keeps
  | a list of all the currently active instances.
  | 
  | Just create an instance of AvailableServiceList
  | and it will start listening - you can
  | register a callback with its onChange
  | member to find out when services appear/disappear,
  | and you can call getServices() to find
  | out the current list.
  | 
  | @see Service, Advertiser
  | 
  | @tags{Events}
  |
  */
#[no_copy]
#[leak_detector]
pub struct NetworkServiceDiscoveryAvailableServiceList<'a> {
    base:            Thread,
    base2:           AsyncUpdater<'a>,

    /**
      | A lambda that can be set to receive
      | a callback when the list changes
      |
      */
    on_change:       fn() -> (),
    socket:          DatagramSocket, // default = true 
    service_typeuid: String,
    list_lock:       CriticalSection,
    services:        Vec<NetworkServiceDiscoveryService>,
}

impl<'a> Drop for NetworkServiceDiscoveryAvailableServiceList<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
        socket.shutdown();
        stopThread (2000);

        #if ALOE_ANDROID
         releaseMulticastLock();
        #endif
 */
    }
}

impl<'a> NetworkServiceDiscoveryAvailableServiceList<'a> {

    /**
      | Creates an AvailableServiceList that
      | will bind to the given port number and
      | watch the network for Advertisers broadcasting
      | the given service type.
      | 
      | This will only detect broadcasts from
      | an Advertiser object with a matching
      | serviceTypeUID value, and where the
      | broadcastPort matches.
      |
      */
    pub fn new(
        service_type:   &String,
        broadcast_port: i32) -> Self {
    
        todo!();
        /*


            : Thread ("Discovery_listen"), serviceTypeUID (serviceType)

       #if ALOE_ANDROID
        acquireMulticastLock();
       #endif

        socket.bindToPort (broadcastPort);
        startThread (2);
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            while (! threadShouldExit())
        {
            if (socket.waitUntilReady (true, 200) == 1)
            {
                char buffer[1024];
                auto bytesRead = socket.read (buffer, sizeof (buffer) - 1, false);

                if (bytesRead > 10)
                    if (auto xml = parseXML (String (CharPointer_UTF8 (buffer),
                                                     CharPointer_UTF8 (buffer + bytesRead))))
                        if (xml->hasTagName (serviceTypeUID))
                            handleMessage (*xml);
            }

            removeTimedOutServices();
        }
        */
    }
    
    /**
      | Returns a list of the currently known
      | services.
      |
      */
    pub fn get_services(&self) -> Vec<NetworkServiceDiscoveryService> {
        
        todo!();
        /*
            const ScopedLock sl (listLock);
        auto listCopy = services;
        return listCopy;
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            if (onChange != nullptr)
            onChange();
        */
    }
    
    pub fn handle_message_with_xml_event(&mut self, xml: &XmlElement)  {
        
        todo!();
        /*
            Service service;
        service.instanceID = xml.getStringAttribute ("id");

        if (service.instanceID.trim().isNotEmpty())
        {
            service.description = xml.getStringAttribute ("name");
            service.address = IPAddress (xml.getStringAttribute ("address"));
            service.port = xml.getIntAttribute ("port");
            service.lastSeen = Time::getCurrentTime();

            handleMessage (service);
        }
        */
    }
    
    pub fn handle_message(&mut self, service: &NetworkServiceDiscoveryService)  {
        
        todo!();
        /*
            const ScopedLock sl (listLock);

        for (auto& s : services)
        {
            if (s.instanceID == service.instanceID)
            {
                if (s.description != service.description
                     || s.address != service.address
                     || s.port != service.port)
                {
                    s = service;
                    triggerAsyncUpdate();
                }

                s.lastSeen = service.lastSeen;
                return;
            }
        }

        services.push_back (service);
        sortServiceList (services);
        triggerAsyncUpdate();
        */
    }
    
    pub fn remove_timed_out_services(&mut self)  {
        
        todo!();
        /*
            const double timeoutSeconds = 5.0;
        auto oldestAllowedTime = Time::getCurrentTime() - RelativeTime::seconds (timeoutSeconds);

        const ScopedLock sl (listLock);

        auto oldEnd = std::end (services);
        auto newEnd = std::remove_if (std::begin (services), oldEnd,
                                      [=] (const Service& s) { return s.lastSeen < oldestAllowedTime; });

        if (newEnd != oldEnd)
        {
            services.erase (newEnd, oldEnd);
            triggerAsyncUpdate();
        }
        */
    }
}
