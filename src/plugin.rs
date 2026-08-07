#[derive(Default)]
pub struct GreetingPlugin;

impl upwell::Plugin for GreetingPlugin {
    const ID: upwell::PluginId =
        upwell::namespaced_id!(upwell::PluginId, "{{ crate_name }}/plugin");

    fn contribute(self, contributions: &mut upwell::PluginContributions) {
        contributions.component::<crate::GreetingService>(upwell::namespaced_id!(
            upwell::ContributionId,
            "{{ crate_name }}/greeting-service"
        ));
    }
}

#[cfg(test)]
mod tests;
