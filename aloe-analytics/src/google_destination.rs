crate::ix!();

pub struct GoogleAnalyticsDestination<'a> {
    base:                ThreadedAnalyticsDestination<'a>,
    initial_period_ms:   i32, // default = 1000
    period_ms:           i32, // default = initialPeriodMs
    web_stream_creation: CriticalSection,
    should_exit:         bool, // default = false
    web_stream:          Box<WebInputStream<'a>>,
    api_key:             String,
    saved_events_file:   File,
}

impl<'a> Default for GoogleAnalyticsDestination<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            : ThreadedAnalyticsDestination ("GoogleAnalyticsThread")

            {
                // Choose where to save any unsent events.

                auto appDataDir = File::getSpecialLocation (File::userApplicationDataDirectory)
                                       .getChildFile (ALOEApplication::getInstance()->getApplicationName());

                if (! appDataDir.exists())
                    appDataDir.createDirectory();

                savedEventsFile = appDataDir.getChildFile ("analytics_events.xml");
            }

            {
                // It's often a good idea to construct any analytics service API keys
                // at runtime, so they're not searchable in the binary distribution of
                // your application (but we've not done this here). You should replace
                // the following key with your own to get this example application
                // fully working.

                apiKey = "UA-XXXXXXXXX-1";
            }

            startAnalyticsThread (initialPeriodMs)
        */
    }
}

impl<'a> Drop for GoogleAnalyticsDestination<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            // Here we sleep so that our background thread has a chance to send the
            // last lot of batched events. Be careful - if your app takes too long to
            // shut down then some operating systems will kill it forcibly!
            Thread::sleep (initialPeriodMs);

            stopAnalyticsThread (1000);
         */
    }
}

impl<'a> GoogleAnalyticsDestination<'a> {

    pub fn get_maximum_batch_size(&mut self) -> i32 {
        
        todo!();
        /*
            return 20;
        */
    }
    
    pub fn log_batched_events(&mut self, events: &[AnalyticsEvent]) -> bool {
        
        todo!();
        /*
            // Send events to Google Analytics.

            String appData ("v=1&aip=1&tid=" + apiKey);

            StringArray postData;

            for (auto& event : events)
            {
                StringPairArray data;

                switch (event.eventType)
                {
                    case (DemoAnalyticsEventTypes::event):
                    {
                        data.set ("t", "event");

                        if (event.name == "startup")
                        {
                            data.set ("ec",  "info");
                            data.set ("ea",  "appStarted");
                        }
                        else if (event.name == "shutdown")
                        {
                            data.set ("ec",  "info");
                            data.set ("ea",  "appStopped");
                        }
                        else if (event.name == "button_press")
                        {
                            data.set ("ec",  "button_press");
                            data.set ("ea",  event.parameters["id"]);
                        }
                        else if (event.name == "crash")
                        {
                            data.set ("ec",  "crash");
                            data.set ("ea",  "crash");
                        }
                        else
                        {
                            jassertfalse;
                            continue;
                        }

                        break;
                    }

                    default:
                    {
                        // Unknown event type! In this demo app we're just using a
                        // single event type, but in a real app you probably want to
                        // handle multiple ones.
                        jassertfalse;
                        break;
                    }
                }

                data.set ("cid", event.userID);

                StringArray eventData;

                for (auto& key : data.getAllKeys())
                    eventData.add (key + "=" + Url::addEscapeChars (data[key], true));

                postData.add (appData + "&" + eventData.joinIntoString ("&"));
            }

            auto url = Url ("https://www.google-analytics.com/batch")
                           .withPOSTData (postData.joinIntoString ("\n"));

            {
                const ScopedLock lock (webStreamCreation);

                if (shouldExit)
                    return false;

                webStream.reset (new WebInputStream (url, true));
            }

            auto success = webStream->connect (nullptr);

            // Do an exponential backoff if we failed to connect.
            if (success)
                periodMs = initialPeriodMs;
            else
                periodMs *= 2;

            setBatchPeriod (periodMs);

            return success;
        */
    }
    
    pub fn stop_logging_events(&mut self)  {
        
        todo!();
        /*
            const ScopedLock lock (webStreamCreation);

            shouldExit = true;

            if (webStream.get() != nullptr)
                webStream->cancel();
        */
    }
    
    pub fn save_unlogged_events(&mut self, events_to_save: &VecDeque<AnalyticsEvent>)  {
        
        todo!();
        /*
            // Save unsent events to disk. Here we use XML as a serialisation format, but
            // you can use anything else as long as the restoreUnloggedEvents method can
            // restore events from disk. If you're saving very large numbers of events then
            // a binary format may be more suitable if it is faster - remember that this
            // method is called on app shutdown so it needs to complete quickly!

            auto xml = parseXMLIfTagMatches (savedEventsFile, "events");

            if (xml == nullptr)
                xml = std::make_unique<XmlElement> ("events");

            for (auto& event : eventsToSave)
            {
                auto* xmlEvent = new XmlElement ("google_analytics_event");
                xmlEvent->setAttribute ("name", event.name);
                xmlEvent->setAttribute ("type", event.eventType);
                xmlEvent->setAttribute ("timestamp", (int) event.timestamp);
                xmlEvent->setAttribute ("user_id", event.userID);

                auto* parameters = new XmlElement ("parameters");

                for (auto& key : event.parameters.getAllKeys())
                    parameters->setAttribute (key, event.parameters[key]);

                xmlEvent->addChildElement (parameters);

                auto* userProperties = new XmlElement ("user_properties");

                for (auto& key : event.userProperties.getAllKeys())
                    userProperties->setAttribute (key, event.userProperties[key]);

                xmlEvent->addChildElement (userProperties);

                xml->addChildElement (xmlEvent);
            }

            xml->writeTo (savedEventsFile, {});
        */
    }
    
    pub fn restore_unlogged_events(&mut self, restored_event_queue: &mut VecDeque<AnalyticsEvent>)  {
        
        todo!();
        /*
            if (auto xml = parseXMLIfTagMatches (savedEventsFile, "events"))
            {
                auto numEvents = xml->getNumChildElements();

                for (auto iEvent = 0; iEvent < numEvents; ++iEvent)
                {
                    auto* xmlEvent = xml->getChildElement (iEvent);

                    StringPairArray parameters;
                    auto* xmlParameters = xmlEvent->getChildByName ("parameters");
                    auto numParameters = xmlParameters->getNumAttributes();

                    for (auto iParam = 0; iParam < numParameters; ++iParam)
                        parameters.set (xmlParameters->getAttributeName (iParam),
                                        xmlParameters->getAttributeValue (iParam));

                    StringPairArray userProperties;
                    auto* xmlUserProperties = xmlEvent->getChildByName ("user_properties");
                    auto numUserProperties = xmlUserProperties->getNumAttributes();

                    for (auto iProp = 0; iProp < numUserProperties; ++iProp)
                        userProperties.set (xmlUserProperties->getAttributeName (iProp),
                                            xmlUserProperties->getAttributeValue (iProp));

                    restoredEventQueue.push_back ({
                        xmlEvent->getStringAttribute ("name"),
                        xmlEvent->getIntAttribute ("type"),
                        static_cast<uint32> (xmlEvent->getIntAttribute ("timestamp")),
                        parameters,
                        xmlEvent->getStringAttribute ("user_id"),
                        userProperties
                    });
                }

                savedEventsFile.deleteFile();
            }
        */
    }
}
