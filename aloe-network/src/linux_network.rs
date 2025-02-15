crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/native/aloe_linux_Network.cpp]

pub trait OpenEmailWithAttachments {
    
    fn open_email_with_attachments(&mut self, 
        target_email_address: &String,
        email_subject:        &String,
        body_text:            &String,
        files_to_attach:      &Vec<String>) -> bool;
}

impl OpenEmailWithAttachments for Process {
    
    fn open_email_with_attachments(&mut self, 
        target_email_address: &String,
        email_subject:        &String,
        body_text:            &String,
        files_to_attach:      &Vec<String>) -> bool {
        
        todo!();
        /*
            jassertfalse;    // xxx todo
        return false;
        */
    }
}

#[cfg(not(ALOE_USE_CURL))]
#[no_copy]
#[leak_detector]
pub(crate) struct WebInputStreamImpl<'a> {
    status_code:                    i32, // default = 0
    owner:                          &'a mut WebInputStream<'a>,
    url:                            Url,
    socket_handle:                  i32, // default = -1
    levels_of_redirection:          i32, // default = 0
    header_lines:                   Vec<String>,
    address:                        String,
    headers:                        String,
    post_data:                      MemoryBlock,
    content_length:                 i64, // default = -1
    position:                       i64, // default = 0
    finished:                       bool, // default = false
    add_parameters_to_request_body: bool,
    has_body_data_to_send:          bool,
    time_out_ms:                    i32, // default = 0
    num_redirects_to_follow:        i32, // default = 5
    http_request_cmd:               String,
    chunk_end:                      i64, // default = 0
    is_chunked:                     bool, // default = false
    reading_chunk:                  bool, // default = false
    close_socket_lock:              CriticalSection,
    create_socket_lock:             CriticalSection,
    has_been_cancelled:             bool, // default = false
}

#[cfg(not(ALOE_USE_CURL))]
impl<'a> Drop for WebInputStreamImpl<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                closeSocket();
             */
    }
}

#[cfg(not(ALOE_USE_CURL))]
impl<'a> WebInputStreamImpl<'a> {

    pub fn new(
        pimpl_owner:            &mut WebInputStream,
        url_to_copy:            &Url,
        add_parameters_to_body: bool) -> Self {
    
        todo!();
        /*


            : owner (pimplOwner),
                  url (urlToCopy),
                  addParametersToRequestBody (addParametersToBody),
                  hasBodyDataToSend (addParametersToRequestBody || url.hasBodyDataToSend()),
                  httpRequestCmd (hasBodyDataToSend ? "POST" : "GET")
        */
    }

    /**
       WebInputStream methods
      */
    pub fn with_extra_headers(&mut self, extra_headers: &String)  {
        
        todo!();
        /*
            if (! headers.endsWithChar ('\n') && headers.isNotEmpty())
                    headers << "\r\n";

                headers << extraHeaders;

                if (! headers.endsWithChar ('\n') && headers.isNotEmpty())
                    headers << "\r\n";
        */
    }
    
    pub fn with_custom_request_command(&mut self, custom_request_command: &String)  {
        
        todo!();
        /*
            httpRequestCmd = customRequestCommand;
        */
    }
    
    pub fn with_connection_timeout(&mut self, timeout_in_ms: i32)  {
        
        todo!();
        /*
            timeOutMs = timeoutInMs;
        */
    }
    
    pub fn with_num_redirects_to_follow(&mut self, max_redirects_to_follow: i32)  {
        
        todo!();
        /*
            numRedirectsToFollow = maxRedirectsToFollow;
        */
    }
    
    pub fn get_status_code(&self) -> i32 {
        
        todo!();
        /*
            return statusCode;
        */
    }
    
    pub fn get_request_headers(&self) -> StringPairArray {
        
        todo!();
        /*
            return WebInputStream::parseHttpHeaders (headers);
        */
    }
    
    pub fn get_response_headers(&self) -> StringPairArray {
        
        todo!();
        /*
            StringPairArray responseHeaders;

                if (! isError())
                {
                    for (int i = 0; i < headerLines.size(); ++i)
                    {
                        auto& headersEntry = headerLines[i];
                        auto key   = headersEntry.upToFirstOccurrenceOf (": ", false, false);
                        auto value = headersEntry.fromFirstOccurrenceOf (": ", false, false);
                        auto previousValue = responseHeaders[key];
                        responseHeaders.set (key, previousValue.isEmpty() ? value : (previousValue + "," + value));
                    }
                }

                return responseHeaders;
        */
    }
    
    pub fn connect(&mut self, listener: *mut dyn WebInputStreamListener) -> bool {
        
        todo!();
        /*
            {
                    const ScopedLock lock (createSocketLock);

                    if (hasBeenCancelled)
                        return false;
                }

                address = url.toString (! addParametersToRequestBody);
                statusCode = createConnection (listener, numRedirectsToFollow);

                return statusCode != 0;
        */
    }
    
    pub fn cancel(&mut self)  {
        
        todo!();
        /*
            const ScopedLock lock (createSocketLock);

                hasBeenCancelled = true;
                statusCode = -1;
                finished = true;

                closeSocket();
        */
    }
    
    pub fn is_error(&self) -> bool {
        
        todo!();
        /*
            return socketHandle < 0;
        */
    }
    
    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return finished;
        */
    }
    
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return position;
        */
    }
    
    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            return contentLength;
        */
    }
    
    pub fn read(&mut self, 
        buffer:        *mut c_void,
        bytes_to_read: i32) -> i32 {
        
        todo!();
        /*
            if (finished || isError())
                    return 0;

                if (isChunked && ! readingChunk)
                {
                    if (position >= chunkEnd)
                    {
                        const ScopedValueSetter<bool> setter (readingChunk, true, false);
                        MemoryOutputStream chunkLengthBuffer;
                        char c = 0;

                        if (chunkEnd > 0)
                        {
                            if (read (&c, 1) != 1 || c != '\r'
                                 || read (&c, 1) != 1 || c != '\n')
                            {
                                finished = true;
                                return 0;
                            }
                        }

                        while (chunkLengthBuffer.getDataSize() < 512 && ! (finished || isError()))
                        {
                            if (read (&c, 1) != 1)
                            {
                                finished = true;
                                return 0;
                            }

                            if (c == '\r')
                                continue;

                            if (c == '\n')
                                break;

                            chunkLengthBuffer.writeByte (c);
                        }

                        auto chunkSize = chunkLengthBuffer.toString().trimStart().getHexValue64();

                        if (chunkSize == 0)
                        {
                            finished = true;
                            return 0;
                        }

                        chunkEnd += chunkSize;
                    }

                    if (bytesToRead > chunkEnd - position)
                        bytesToRead = static_cast<int> (chunkEnd - position);
                }

                pollfd pfd { socketHandle, POLLIN, 0 };

                if (poll (&pfd, 1, timeOutMs) <= 0)
                    return 0; // (timeout)

                auto bytesRead = jmax (0, (int) recv (socketHandle, buffer, (size_t) bytesToRead, MSG_WAITALL));

                if (bytesRead == 0)
                    finished = true;

                if (! readingChunk)
                    position += bytesRead;

                return bytesRead;
        */
    }
    
    pub fn set_position(&mut self, wanted_pos: i64) -> bool {
        
        todo!();
        /*
            if (isError())
                    return false;

                if (wantedPos != position)
                {
                    finished = false;

                    if (wantedPos < position)
                        return false;

                    auto numBytesToSkip = wantedPos - position;
                    auto skipBufferSize = (int) jmin (numBytesToSkip, (int64) 16384);
                    HeapBlock<char> temp (skipBufferSize);

                    while (numBytesToSkip > 0 && ! isExhausted())
                        numBytesToSkip -= read (temp, (int) jmin (numBytesToSkip, (int64) skipBufferSize));
                }

                return true;
        */
    }
    
    pub fn close_socket(&mut self, reset_levels_of_redirection: Option<bool>) {

        let reset_levels_of_redirection: bool = reset_levels_of_redirection.unwrap_or(true);

        todo!();
        /*
            const ScopedLock lock (closeSocketLock);

                if (socketHandle >= 0)
                {
                    ::shutdown (socketHandle, SHUT_RDWR);
                    ::close (socketHandle);
                }

                socketHandle = -1;

                if (resetLevelsOfRedirection)
                    levelsOfRedirection = 0;
        */
    }
    
    pub fn create_connection(
        &mut self, 
        listener:      *mut dyn WebInputStreamListener,
        num_redirects: i32

    ) -> i32 {
        
        todo!();
        /*
            closeSocket (false);

                if (hasBodyDataToSend)
                    WebInputStream::createHeadersAndPostData (url,
                                                              headers,
                                                              postData,
                                                              addParametersToRequestBody);

                auto timeOutTime = Time::getMillisecondCounter();

                if (timeOutMs == 0)
                    timeOutMs = 30000;

                if (timeOutMs < 0)
                    timeOutTime = 0xffffffff;
                else
                    timeOutTime += (uint32) timeOutMs;

                String hostName, hostPath;
                int hostPort;

                if (! decomposeURL (address, hostName, hostPath, hostPort))
                    return 0;

                String serverName, proxyName, proxyPath;
                int proxyPort = 0;
                int port = 0;

                auto proxyURL = String::fromUTF8 (getenv ("http_proxy"));

                if (proxyURL.startsWithIgnoreCase ("http://"))
                {
                    if (! decomposeURL (proxyURL, proxyName, proxyPath, proxyPort))
                        return 0;

                    serverName = proxyName;
                    port = proxyPort;
                }
                else
                {
                    serverName = hostName;
                    port = hostPort;
                }

                struct addrinfo hints;
                zerostruct (hints);

                hints.ai_family = AF_UNSPEC;
                hints.ai_socktype = SOCK_STREAM;
                hints.ai_flags = AI_NUMERICSERV;

                struct addrinfo* result = nullptr;

                if (getaddrinfo (serverName.toUTF8(), String (port).toUTF8(), &hints, &result) != 0 || result == nullptr)
                    return 0;

                {
                    const ScopedLock lock (createSocketLock);

                    socketHandle = hasBeenCancelled ? -1
                                                    : socket (result->ai_family, result->ai_socktype, 0);
                }

                if (socketHandle == -1)
                {
                    freeaddrinfo (result);
                    return 0;
                }

                int receiveBufferSize = 16384;
                setsockopt (socketHandle, SOL_SOCKET, SO_RCVBUF, (char*) &receiveBufferSize, sizeof (receiveBufferSize));
                setsockopt (socketHandle, SOL_SOCKET, SO_KEEPALIVE, nullptr, 0);

              #if ALOE_MAC
                setsockopt (socketHandle, SOL_SOCKET, SO_NOSIGPIPE, 0, 0);
              #endif

                if (::connect (socketHandle, result->ai_addr, result->ai_addrlen) == -1)
                {
                    closeSocket();
                    freeaddrinfo (result);
                    return 0;
                }

                freeaddrinfo (result);

                {
                    const MemoryBlock requestHeader (createRequestHeader (hostName, hostPort, proxyName, proxyPort, hostPath, address,
                                                                          headers, postData, httpRequestCmd));

                    if (! sendHeader (socketHandle, requestHeader, timeOutTime, owner, listener))
                    {
                        closeSocket();
                        return 0;
                    }
                }

                auto responseHeader = readResponse (timeOutTime);
                position = 0;

                if (responseHeader.isNotEmpty())
                {
                    headerLines = Vec<String>::fromLines (responseHeader);

                    auto status = responseHeader.fromFirstOccurrenceOf (" ", false, false)
                                                .substring (0, 3).getIntValue();

                    auto location = findHeaderItem (headerLines, "Location:");

                    if (++levelsOfRedirection <= numRedirects
                         && status >= 300 && status < 400
                         && location.isNotEmpty() && location != address)
                    {
                        if (! (location.startsWithIgnoreCase ("http://")
                                || location.startsWithIgnoreCase ("https://")
                                || location.startsWithIgnoreCase ("ftp://")))
                        {
                            // The following is a bit dodgy. Ideally, we should do a proper transform of the relative URI to a target URI
                            if (location.startsWithChar ('/'))
                                location = Url (address).withNewSubPath (location).toString (true);
                            else
                                location = address + "/" + location;
                        }

                        address = location;
                        return createConnection (listener, numRedirects);
                    }

                    auto contentLengthString = findHeaderItem (headerLines, "Content-Length:");

                    if (contentLengthString.isNotEmpty())
                        contentLength = contentLengthString.getLargeIntValue();

                    isChunked = (findHeaderItem (headerLines, "Transfer-Encoding:") == "chunked");

                    return status;
                }

                closeSocket();
                return 0;
        */
    }
    
    pub fn read_response(&mut self, time_out_time: u32) -> String {
        
        todo!();
        /*
            int numConsecutiveLFs  = 0;
                MemoryOutputStream buffer;

                while (numConsecutiveLFs < 2
                        && buffer.getDataSize() < 32768
                        && Time::getMillisecondCounter() <= timeOutTime
                        && ! (finished || isError()))
                {
                    char c = 0;

                    if (read (&c, 1) != 1)
                        return {};

                    buffer.writeByte (c);

                    if (c == '\n')
                        ++numConsecutiveLFs;
                    else if (c != '\r')
                        numConsecutiveLFs = 0;
                }

                auto header = buffer.toString().trimEnd();

                if (header.startsWithIgnoreCase ("HTTP/"))
                    return header;

                return {};
        */
    }
    
    pub fn write_value_if_not_present(
        dest:    &mut MemoryOutputStream,
        headers: &String,
        key:     &String,
        value:   &String)  {
        
        todo!();
        /*
            if (! headers.containsIgnoreCase (key))
                    dest << "\r\n" << key << ' ' << value;
        */
    }
    
    pub fn write_host(
        dest:             &mut MemoryOutputStream,
        http_request_cmd: &String,
        path:             &String,
        host:             &String,
        port:             i32)  {
        
        todo!();
        /*
            dest << httpRequestCmd << ' ' << path << " HTTP/1.1\r\nHost: " << host;

                /* HTTP spec 14.23 says that the port number must be included in the header if it is not 80 */
                if (port != 80)
                    dest << ':' << port;
        */
    }
    
    pub fn create_request_header(
        host_name:        &String,
        host_port:        i32,
        proxy_name:       &String,
        proxy_port:       i32,
        host_path:        &String,
        originalurl:      &String,
        user_headers:     &String,
        post_data:        &MemoryBlock,
        http_request_cmd: &String) -> MemoryBlock {
        
        todo!();
        /*
            MemoryOutputStream header;

                if (proxyName.isEmpty())
                    writeHost (header, httpRequestCmd, hostPath, hostName, hostPort);
                else
                    writeHost (header, httpRequestCmd, originalURL, proxyName, proxyPort);

                writeValueIfNotPresent (header, userHeaders, "User-Agent:", "Aloe/" ALOE_STRINGIFY(ALOE_MAJOR_VERSION)
                                                                                "." ALOE_STRINGIFY(ALOE_MINOR_VERSION)
                                                                                "." ALOE_STRINGIFY(ALOE_BUILDNUMBER));
                writeValueIfNotPresent (header, userHeaders, "Connection:", "close");

                const auto postDataSize = postData.getSize();
                const auto hasPostData = postDataSize > 0;

                if (hasPostData)
                    writeValueIfNotPresent (header, userHeaders, "Content-Length:", String ((int) postDataSize));

                if (userHeaders.isNotEmpty())
                    header << "\r\n" << userHeaders;

                header << "\r\n\r\n";

                if (hasPostData)
                    header << postData;

                return header.getMemoryBlock();
        */
    }
    
    pub fn send_header(
        socket_handle:  i32,
        request_header: &MemoryBlock,
        time_out_time:  u32,
        pimpl_owner:    &mut WebInputStream,
        listener:       *mut dyn WebInputStreamListener) -> bool {
        
        todo!();
        /*
            size_t totalHeaderSent = 0;

                while (totalHeaderSent < requestHeader.getSize())
                {
                    if (Time::getMillisecondCounter() > timeOutTime)
                        return false;

                    auto numToSend = jmin (1024, (int) (requestHeader.getSize() - totalHeaderSent));

                    if (send (socketHandle, static_cast<const char*> (requestHeader.getData()) + totalHeaderSent, (size_t) numToSend, 0) != numToSend)
                        return false;

                    totalHeaderSent += (size_t) numToSend;

                    if (listener != nullptr && ! listener->postDataSendProgress (pimplOwner, (int) totalHeaderSent, (int) requestHeader.getSize()))
                        return false;
                }

                return true;
        */
    }
    
    pub fn decomposeurl(
        url:  &String,
        host: &mut String,
        path: &mut String,
        port: &mut i32) -> bool {
        
        todo!();
        /*
            if (! url.startsWithIgnoreCase ("http://"))
                    return false;

                auto nextSlash = url.indexOfChar (7, '/');
                auto nextColon = url.indexOfChar (7, ':');

                if (nextColon > nextSlash && nextSlash > 0)
                    nextColon = -1;

                if (nextColon >= 0)
                {
                    host = url.substring (7, nextColon);

                    if (nextSlash >= 0)
                        port = url.substring (nextColon + 1, nextSlash).getIntValue();
                    else
                        port = url.substring (nextColon + 1).getIntValue();
                }
                else
                {
                    port = 80;

                    if (nextSlash >= 0)
                        host = url.substring (7, nextSlash);
                    else
                        host = url.substring (7);
                }

                if (nextSlash >= 0)
                    path = url.substring (nextSlash);
                else
                    path = "/";

                return true;
        */
    }
    
    pub fn find_header_item(
        lines:     &Vec<String>,
        item_name: &String) -> String {
        
        todo!();
        /*
            for (int i = 0; i < lines.size(); ++i)
                    if (lines[i].startsWithIgnoreCase (itemName))
                        return lines[i].substring (itemName.length()).trim();

                return {};
        */
    }
}
