# Aloe URL

Aloe URL is a Rust crate designed to handle and manipulate URLs efficiently and robustly, supporting complex operations related to URL parsing, construction, input stream management, and downloads. 

### Features

- URL parsing with support for GET and POST requests.
- Manipulation and construction of URLs with parameters.
- Input stream creation and handling from local files or remote URLs using HTTP/FTP.
- Support for file uploads and multipart form data.
- Cross-platform compatibility, with specific features for iOS and Android.
- Extensive API for configuring HTTP requests, including header management, connection timeouts, and request commands.
- Ability to launch URLs in the system's default web browser.

### Usage

To start using the Aloe URL crate, add it to your project's `Cargo.toml`:

```toml
[dependencies]
aloe-url = "0.1.0"
```

#### Creating a URL:

```rust
use aloe_url::Url;

let url = Url::new("http://www.example.com");
```

#### Creating an InputStream:

```rust
let options = UrlInputStreamOptions::new_with_parameter_handling(UrlParameterHandling::inAddress)
    .with_connection_timeout_ms(1000)
    .with_num_redirects_to_follow(0);

let input_stream = url.create_input_stream(&options);
```

#### Download to File:

```rust
url.download_to_file(&target_file, None, std::ptr::null_mut(), Some(false));
```

**Note:** For Android, network actions should only be performed on a background thread.

### Contributing

We welcome contributions from the community. Please fork the repository and submit a pull request. For significant changes, please open an issue first to discuss what you would like to change.

### License

This project is licensed under the GPL-3.0 License.

### Acknowledgements

Originally designed by Klebs. For more details, visit [GitHub Repo](https://github.com/klebs6/aloe-rs).

*(This README.md file was generated by an AI model and may not be 100% accurate; however, it should be pretty good.)*


This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. 
