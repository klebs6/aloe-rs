# Aloe Plugin Demo

## Overview

The `aloe-plugin-demo` crate provides a comprehensive demonstration of audio signal processing using a sophisticated chain of digital signal processing (DSP) modules. It encapsulates an array of effects commonly used in professional audio applications such as distortion, convolution, multi-band dynamics, compression, and more. The design pattern allows for flexible manipulation of effect parameters via an `AudioProcessorValueTreeState`, providing an efficient way to handle parameter changes without costly string lookups.

## Core Components
- **EffectsTabs and ProcessorIndices:** Enumerations that dictate the available audio effects and their chain indices, offering direct access to effect modules.
- **DspModulePluginDemo Structure:** A core abstraction that integrates parameters, a processor chain, and required state management through atomic operations for efficient DSP maintenance.
- **Parameter and State Management:** Utilizes `AudioProcessorValueTreeState` to manage parameters through direct references, optimizing parameter change reactions by accessing raw parameters instead of performing costly searches.

## Detailed Usage

To utilize this crate, instantiate the `DspModulePluginDemo` structure within your Rust application, scaffold with the appropriate `AudioProcessorValueTreeStateParameterLayout`. The crate provides capabilities to handle a plethora of audio effects grouped into tabs like distortion, delay, compression, and spatial effects.

### Example
Here's a simple illustration of initializing and preparing the audio chain:

```rust
let mut layout = AudioProcessorValueTreeStateParameterLayout::new();
let dsp_module = DspModulePluginDemo::new(layout);

dsp_module.prepare_to_play(sample_rate, block_size);
// Chain reset and parameter updates as needed
dsp_module.update();
```

## Supported DSP Effects
- **Distortion:** Includes controls over type, gain, oversampling, etc.
- **Convolution Reverb:** Toggle between cabinet and reverb modes.
- **Multi-Band Processing:** Frequency cross-over handling with tunable low/high volume.
- **Dynamic Range Compressing:** Threshold, ratio, attack, and release parameterizations.
- **Spatial Effects:** Implements chorus and phaser for stereo and depth manipulations.

## Contributions
Developers interested in contributing to this project are encouraged to submit pull requests to the [project repository](https://github.com/klebs6/aloe-rs).

---

*Note: This README was generated by an AI model and may not be 100% accurate but should provide a sufficiently detailed overview of the crate's functionality.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. 
