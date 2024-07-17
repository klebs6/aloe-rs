crate::ix!();

pub fn can_connect_to_website(url: &Url) -> bool {
    
    todo!();
        /*
            std::unique_ptr<InputStream> in (url.createInputStream (false, nullptr, nullptr, String(), 2000, nullptr));
        return in != nullptr;
        */
}

pub fn are_major_websites_available() -> bool {
    
    todo!();
        /*
            const char* urlsToTry[] = { "http://google.com",  "http://bing.com",  "http://amazon.com",
                                    "https://google.com", "https://bing.com", "https://amazon.com", nullptr};

        for (const char** url = urlsToTry; *url != nullptr; ++url)
            if (canConnectToWebsite (Url (*url)))
                return true;

        return false;
        */
}
