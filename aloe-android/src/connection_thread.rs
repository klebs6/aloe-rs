crate::ix!();

#[cfg(target_os="android")]
pub struct ConnectionThread<'a> {
    base:             Thread,
    owner:            &'a mut WebBrowserComponentPimpl<'a>,
    web_input_stream: Box<WebInputStream<'a>>,
    result:           Result<(),()>,
}

pub struct ConnectionThreadResult
{
    url:         String,
    status_code: i32, // default = 0
    description: String,
    data:        String,
}

#[cfg(target_os="android")]
impl<'a> Drop for ConnectionThread<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            webInputStream->cancel();
                signalThreadShouldExit();
                waitForThreadToExit (10000);

                webInputStream = nullptr;
        */
    }
}

#[cfg(target_os="android")]
impl<'a> ConnectionThread<'a> {

    pub fn new(
        owner_to_use: &mut WebBrowserComponentPimpl,
        url:          &mut Url,
        headers:      &Vec<String>) -> Self {
    
        todo!();
        /*


            : Thread ("WebBrowserComponent::WebBrowserComponentPimpl::ConnectionThread"),
                  owner (ownerToUse),
                  webInputStream (new WebInputStream (url, true))

                webInputStream->withExtraHeaders (headers.joinIntoString ("\n"));
                webInputStream->withConnectionTimeout (10000);

                result.url = url.toString (true);

                startThread();
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            if (! webInputStream->connect (nullptr))
                {
                    result.description = "Could not establish connection";
                    owner.triggerAsyncUpdate();
                    return;
                }

                result.statusCode = webInputStream->getStatusCode();
                result.description = "Status code: " + String (result.statusCode);
                readFromInputStream();
                owner.triggerAsyncUpdate();
        */
    }
    
    pub fn get_result(&mut self) -> &Result<(),()> {
        
        todo!();
        /*
            return result;
        */
    }
    
    pub fn read_from_input_stream(&mut self)  {
        
        todo!();
        /*
            MemoryOutputStream ostream;

                for (;;)
                {
                    if (threadShouldExit())
                        return;

                    char buffer [8192];
                    auto num = webInputStream->read (buffer, sizeof (buffer));

                    if (num <= 0)
                        break;

                    ostream.write (buffer, (size_t) num);
                }

                result.data = ostream.toUTF8();
        */
    }
}
