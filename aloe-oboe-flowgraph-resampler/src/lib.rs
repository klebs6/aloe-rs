#[macro_use] mod imports; use imports::*;

x!{flowgraph_resampler_kaiserwindow}
x!{flowgraph_resampler_multichannelresampler}
x!{flowgraph_ramplinear}
x!{flowgraph_resampler_sincresampler}
x!{flowgraph_resampler_polyphaseresamplermono}
x!{flowgraph_resampler_linearresampler}
x!{flowgraph_resampler_hyperboliccosinewindow}
x!{flowgraph_resampler_multichannelresampler_builder}
x!{flowgraph_resampler_polyphaseresamplerstereo}
x!{flowgraph_resampler_sincresamplerstereo}
x!{flowgraph_resampler_integerratio}
x!{flowgraph_resampler_polyphaseresampler}

pub struct Missing {}
pub type MultiChannelResamplerBuilder = Missing;
