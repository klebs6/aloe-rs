crate::ix!();

/// A trait defining a contract for objects that need to react to changes in network topology.
/// 
/// Implementors of this trait are expected to encapsulate logic that gets triggered whenever
/// there is a modification in the network architecture, such as addition or removal of nodes,
/// changes in connection topology, or alterations in node attributes.
/// 
/// # Examples
///
/// ```
/// struct NetworkMonitor;
///
/// impl TopologyChanged for NetworkMonitor {
///     fn topology_changed(&mut self) {
///         // Implement logic to handle topology change
///     }
/// }
/// ```
///
pub trait TopologyChanged {
    /// Method to be called when the network topology changes.
    ///
    /// This should encapsulate the logic required to adapt to the new topology, which could
    /// involve tasks such as recalculating routing tables, rebalancing load, or issuing
    /// alerts.
    fn topology_changed(&mut self);
}
