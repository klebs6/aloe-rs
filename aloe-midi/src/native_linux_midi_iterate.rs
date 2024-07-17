crate::ix!();

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub fn iterate_midi_client(
    client:                    &AlsaClientPtr,
    client_info:               *mut SndSeqClientInfo,
    for_input:                 bool,
    devices:                   &mut Vec<MidiDeviceInfo>,
    device_identifier_to_open: &String

) -> *mut AlsaClientPort {

    todo!();
    /*
        AlsaClient::AlsaClientPort* port = nullptr;

        auto seqHandle = client->get();
        snd_seq_port_info_t* portInfo = nullptr;

        snd_seq_port_info_alloca (&portInfo);
        jassert (portInfo != nullptr);
        auto numPorts = snd_seq_client_info_get_num_ports (clientInfo);
        auto sourceClient = snd_seq_client_info_get_client (clientInfo);

        snd_seq_port_info_set_client (portInfo, sourceClient);
        snd_seq_port_info_set_port (portInfo, -1);

        while (--numPorts >= 0)
        {
            if (snd_seq_query_next_port (seqHandle, portInfo) == 0
                && (snd_seq_port_info_get_capability (portInfo)
                    & (forInput ? SND_SEQ_PORT_CAP_SUBS_READ : SND_SEQ_PORT_CAP_SUBS_WRITE)) != 0)
            {
                String portName (snd_seq_port_info_get_name (portInfo));
                auto portID = snd_seq_port_info_get_port (portInfo);

                MidiDeviceInfo device (portName, getFormattedPortIdentifier (sourceClient, portID));
                devices.add (device);

                if (deviceIdentifierToOpen.isNotEmpty() && deviceIdentifierToOpen == device.identifier)
                {
                    if (portID != -1)
                    {
                        port = client->createPort (portName, forInput, false);
                        jassert (port->isValid());
                        port->connectWith (sourceClient, portID);
                        break;
                    }
                }
            }
        }

        return port;
    */
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub fn iterate_midi_devices(
    for_input:                 bool,
    devices:                   &mut Vec<MidiDeviceInfo>,
    device_identifier_to_open: &String

) -> *mut AlsaClientPort {

    todo!();
    /*
        AlsaClient::AlsaClientPort* port = nullptr;
        auto client = AlsaClient::getInstance();

        if (auto seqHandle = client->get())
        {
            snd_seq_system_info_t* systemInfo = nullptr;
            snd_seq_client_info_t* clientInfo = nullptr;

            snd_seq_system_info_alloca (&systemInfo);
            jassert (systemInfo != nullptr);

            if (snd_seq_system_info (seqHandle, systemInfo) == 0)
            {
                snd_seq_client_info_alloca (&clientInfo);
                jassert (clientInfo != nullptr);

                auto numClients = snd_seq_system_info_get_cur_clients (systemInfo);

                while (--numClients >= 0)
                {
                    if (snd_seq_query_next_client (seqHandle, clientInfo) == 0)
                    {
                        port = iterateMidiClient (client, clientInfo, forInput,
                                                  devices, deviceIdentifierToOpen);

                        if (port != nullptr)
                            break;
                    }
                }
            }
        }

        return port;
    */
}
