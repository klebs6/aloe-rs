# Aloe Analytics

Aloe Analytics is a comprehensive Rust-based analytics logging framework designed to manage and dispatch analytics events with maximal precision and efficiency. This crate provides flexible interfaces to define custom analytics destinations for data collection and analysis.

## Features
- **Event Logging:** Utilize `AnalyticsDestinationInterface` to log events, whether atomic, batched, or restored.
- **Threaded Analytics:** Implement `ThreadedAnalyticsDestinationInterface` to handle concurrent event processing, ideal for integrations with web servers.
- **Dynamic Event Handling:** Use `ButtonTracker` to associate UI interactions with analytics events seamlessly.
- **Google Analytics Integration:** Extend `GoogleAnalyticsDestination` for detailed telemetry on user interactions.
- **Session Management:** Capture session lifecycle events with ease, like session start and end, screen views, and exceptions.

## Technical Overview
Aloe Analytics is inspired by thread-safe logging patterns, simulating batch operations and resiliency patterns to ensure no information is dropped during network failures. Events are queued and dispatched according to batch policies, optimizing throughput and latency. The crate leverages Rust's concurrency primitives to facilitate non-blocking operations, safeguarding UI performance.

### Key Structures
- `Analytics`: The central manager for dispatching events to destinations.
- `EventDispatcher`: Manages queued events and batch submissions on separate threads.
- `GoogleAnalyticsDestination`: Extends analytics capabilities specific to Google Analytics, providing serialization and persistence for unlogged events during downtime.

## Usage
The crate encourages developers to subclass and tailor analytics destinations using provided interfaces, affording control over how events are processed, serialized, and transmitted.

```rust
fn main() {
    let mut analytics = Analytics::default();
    let destination = GoogleAnalyticsDestination::default();

    analytics.add_destination(Box::new(destination));
    analytics.set_user_id("unique_user_123");
    analytics.log_event(&"app_launch".to_owned(), &vec![], None);
}
```

## License
Licensed under GPL-3.0.

---

*This README.md file was generated by an AI model and may not be 100% accurate; however, it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
