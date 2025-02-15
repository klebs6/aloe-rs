crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_WebInputStream.h]

/**
  | Used to receive callbacks for POST data
  | send progress.
  | 
  | Pass one of these into the connect()
  | method and its postDataSendProgress()
  | method will be called periodically
  | with updates on POST data upload progress.
  |
  */
pub trait WebInputStreamListener {

    /**
      | This method will be called periodically
      | with updates on POST data upload progress.
      | 
      | -----------
      | @param request
      | 
      | the original request
      | ----------
      | @param bytesSent
      | 
      | the number of bytes sent so far
      | ----------
      | @param totalBytes
      | 
      | the total number of bytes to send
      | 
      | -----------
      | @return
      | 
      | true to continue or false to cancel the
      | upload
      |
      */
    fn post_data_send_progress(&mut self, 
        request:     &mut WebInputStream,
        bytes_sent:  i32,
        total_bytes: i32) -> bool {
        
        todo!();
        /*
            ignoreUnused (request, bytesSent, totalBytes);
                return true;
        */
    }
}

/**
  | An InputStream which can be used to read
  | from a given Url.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
#[leak_detector]
pub struct WebInputStream<'a> {
    impl_:              Box<WebInputStreamImpl<'a>>,
    has_called_connect: bool, // default = false
}

impl<'a> Read for WebInputStream<'a> {

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!();
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/network/aloe_WebInputStream.cpp]
impl<'a> WebInputStream<'a> {

    /**
      | Creates a new WebInputStream which
      | can be used to read from a Url.
      | 
      | -----------
      | @param url
      | 
      | the Url that should be retrieved. This
      | parameter may also contain POST data
      | and/or parameters.
      | ----------
      | @param addParametersToRequestBody
      | 
      | specifies whether any Url parameters
      | that have been set will be transferred
      | via the request body data or added to
      | the Url address. This will also determine
      | whether a POST or GET command will be
      | used if a custom command is not set.
      |
      */
    pub fn new(
        url:      &Url,
        use_post: bool) -> Self {
    
        todo!();
        /*
        : impl (std::make_unique<Impl> (*this, url, usePost))
        */
    }

    /**
      | Add extra headers to the HTTP request.
      | 
      | Returns a reference to itself so that
      | several methods can be chained.
      | 
      | -----------
      | @param extraHeaders
      | 
      | this string is appended onto the headers
      | that are used for the request. It must
      | therefore be a valid set of HTML header
      | directives, separated by newlines.
      |
      */
    pub fn with_extra_headers(&mut self, extra: &String) -> &mut WebInputStream {
        
        todo!();
        /*
            impl->withExtraHeaders (extra);       return *this;
        */
    }

    /**
      | Override the HTTP command that is sent.
      | 
      | Returns a reference to itself so that
      | several methods can be chained.
      | 
      | Note that this command will not change
      | the way parameters are sent. This must
      | be specified in the constructor.
      | 
      | -----------
      | @param customRequestCommand
      | 
      | this string is the custom HTTP request
      | command such as POST or GET.
      |
      */
    pub fn with_custom_request_command(&mut self, cmd: &String) -> &mut WebInputStream {
        
        todo!();
        /*
            impl->withCustomRequestCommand(cmd);  return *this;
        */
    }

    /**
      | Specify the connection time-out.
      | 
      | Returns a reference to itself so that
      | several methods can be chained.
      | 
      | -----------
      | @param timeoutInMs
      | 
      | the number of milliseconds to wait until
      | the connection request is aborted.
      |
      */
    pub fn with_connection_timeout(&mut self, t: i32) -> &mut WebInputStream {
        
        todo!();
        /*
            impl->withConnectionTimeout (t);      return *this;
        */
    }

    /**
      | Specify the number of redirects to be
      | followed.
      | 
      | Returns a reference to itself so that
      | several methods can be chained.
      | 
      | -----------
      | @param numRedirects
      | 
      | specifies the number of redirects that
      | will be followed before returning a
      | response (ignored for Android which
      | follows up to 5 redirects)
      |
      */
    pub fn with_num_redirects_to_follow(&mut self, num: i32) -> &mut WebInputStream {
        
        todo!();
        /*
            impl->withNumRedirectsToFollow (num); return *this;
        */
    }

    /**
      | Returns a StringPairArray of the request
      | headers.
      |
      */
    pub fn get_request_headers(&self) -> StringPairArray {
        
        todo!();
        /*
            return impl->getRequestHeaders();
        */
    }

    /**
      | Returns a StringPairArray of response
      | headers.
      | 
      | If getResponseHeaders() is called
      | without an established connection,
      | then getResponseHeaders() will call
      | connect internally and block until
      | connect returns - either due to a successful
      | connection or a connection error.
      | 
      | @see connect
      |
      */
    pub fn get_response_headers(&mut self) -> StringPairArray {
        
        todo!();
        /*
            connect (nullptr); return impl->getResponseHeaders();
        */
    }

    /**
      | Returns true if there was an error during
      | the connection attempt.
      |
      */
    pub fn is_error(&self) -> bool {
        
        todo!();
        /*
            return impl->isError();
        */
    }

    /**
      | Will cancel a blocking read and prevent
      | any subsequent connection attempts.
      |
      */
    pub fn cancel(&mut self)  {
        
        todo!();
        /*
            impl->cancel();
        */
    }

    /**
      | Returns true if the stream has no more
      | data to read.
      |
      */
    pub fn is_exhausted(&mut self) -> bool {
        
        todo!();
        /*
            return impl->isExhausted();
        */
    }

    /**
      | Returns the offset of the next byte that
      | will be read from the stream.
      | 
      | @see setPosition
      |
      */
    pub fn get_position(&mut self) -> i64 {
        
        todo!();
        /*
            return impl->getPosition();
        */
    }

    /**
      | Returns the total number of bytes available
      | for reading in this stream.
      | 
      | Note that this is the number of bytes
      | available from the start of the stream,
      | not from the current position.
      | 
      | If getTotalLength() is called without
      | an established connection, then getTotalLength()
      | will call connect internally and block
      | until connect returns - either due to
      | a successful connection or a connection
      | error.
      | 
      | If the size of the stream isn't actually
      | known, this will return -1.
      |
      */
    pub fn get_total_length(&mut self) -> i64 {
        
        todo!();
        /*
            connect (nullptr); return impl->getTotalLength();
        */
    }

    /**
      | Reads some data from the stream into
      | a memory buffer.
      | 
      | This method will block until the maxBytesToRead
      | bytes are available.
      | 
      | This method calls connect() internally
      | if the connection hasn't already been
      | established.
      | 
      | -----------
      | @param destBuffer
      | 
      | the destination buffer for the data.
      | This must not be null.
      | ----------
      | @param maxBytesToRead
      | 
      | the maximum number of bytes to read -
      | make sure the memory block passed in
      | is big enough to contain this many bytes.
      | This value must not be negative.
      | 
      | -----------
      | @return
      | 
      | the actual number of bytes that were
      | read, which may be less than maxBytesToRead
      | if the stream is exhausted before it
      | gets that far
      |
      */
    pub fn read(&mut self, 
        buffer: *mut c_void,
        bytes:  i32) -> i32 {
        
        todo!();
        /*
            connect (nullptr); return impl->read (buffer, bytes);
        */
    }

    /**
      | Tries to move the current read position
      | of the stream.
      | 
      | The position is an absolute number of
      | bytes from the stream's start.
      | 
      | For a WebInputStream, this method will
      | fail if wantedPos is smaller than the
      | current position. If wantedPos is greater
      | than the current position, then calling
      | setPosition() is the same as calling
      | read(), i.e. the skipped data will still
      | be downloaded, although skipped bytes
      | will be discarded immediately.
      | 
      | -----------
      | @return
      | 
      | true if the stream manages to reposition
      | itself correctly @see getPosition
      |
      */
    pub fn set_position(&mut self, pos: i64) -> bool {
        
        todo!();
        /*
            return impl->setPosition (pos);
        */
    }

    /**
      | Returns the status code returned by
      | the HTTP server
      | 
      | If getStatusCode() is called without
      | an established connection, then getStatusCode()
      | will call connect internally and block
      | until connect returns - either due to
      | a successful connection or a connection
      | error.
      | 
      | @see connect
      |
      */
    pub fn get_status_code(&mut self) -> i32 {
        
        todo!();
        /*
            connect (nullptr); return impl->getStatusCode();
        */
    }

    /**
      | Wait until the first byte is ready for
      | reading.
      | 
      | This method will attempt to connect
      | to the Url given in the constructor and
      | block until the status code and all response
      | headers have been received or an error
      | has occurred.
      | 
      | Note that most methods will call connect()
      | internally if they are called without
      | an established connection. Therefore,
      | it is not necessary to explicitly call
      | connect unless you would like to use
      | a custom listener.
      | 
      | After a successful call to connect(),
      | getResponseHeaders(), getTotalLength()
      | and getStatusCode() will all be non-blocking.
      | 
      | -----------
      | @param listener
      | 
      | a listener to receive progress callbacks
      | on the status of a POST data upload.
      | 
      | @see getResponseHeaders, getTotalLength,
      | getStatusCode
      |
      */
    pub fn connect(&mut self, listener: *mut dyn WebInputStreamListener) -> bool {
        
        todo!();
        /*
            if (hasCalledConnect)
            return ! isError();

        hasCalledConnect = true;
        return impl->connect (listener);
        */
    }

    pub fn parse_http_headers(&mut self, header_data: &String) -> StringPairArray {
        
        todo!();
        /*
            StringPairArray headerPairs;
        auto headerLines = StringArray::fromLines (headerData);

        // ignore the first line as this is the status line
        for (int i = 1; i < headerLines.size(); ++i)
        {
            const auto& headersEntry = headerLines[i];

            if (headersEntry.isNotEmpty())
            {
                const auto key = headersEntry.upToFirstOccurrenceOf (": ", false, false);

                auto value = [&headersEntry, &headerPairs, &key]
                {
                    const auto currentValue = headersEntry.fromFirstOccurrenceOf (": ", false, false);
                    const auto previousValue = headerPairs [key];

                    if (previousValue.isNotEmpty())
                        return previousValue + "," + currentValue;

                    return currentValue;
                }();

                headerPairs.set (key, value);
            }
        }

        return headerPairs;
        */
    }

    pub fn create_headers_and_post_data(&mut self, 
        aurl:                   &Url,
        headers:                &mut String,
        data:                   &mut MemoryBlock,
        add_parameters_to_body: bool)  {
        
        todo!();
        /*
            aURL.createHeadersAndPostData (headers, data, addParametersToBody);
        */
    }
}
