The Rust translation of JUCE (aloe) now provides a fully compiling Rust scaffold wrapping the existing JUCE C++ interfaces.

While the actual function implementations currently remain untranslated (commented C++ code), this scaffold creates a unique opportunity for contributors interested in exploring lower-latency Android MIDI integration via the native AMidi NDK APIs.

Our current development focus is on related projects designed to automate the translation and implementation process. See github.com/klebs6/klebs-general—in particular, the workspacer tool and related utilities—for more details.

If you're interested in contributing to improved MIDI latency on Android devices (API ≥29), integrating AMidi into aloe represents an exciting, impactful area for community-driven innovation. Contributions, experiments, and detailed proposals toward AMidi integration are warmly welcomed.

As tools like workspacer mature, we will leverage them to accelerate the completion and refinement of aloe. These tools will continuously enhance aloe’s integration with the broader Rust ecosystem, including robust MIDI handling.

In the meantime, your community-driven exploration here will greatly accelerate aloe’s development and utility.
