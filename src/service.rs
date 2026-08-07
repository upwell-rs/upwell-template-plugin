#[{% if macro_crate %}crate::plugin_component{% else %}upwell::component{% endif %}(by_value)]
#[derive(Clone)]
pub struct GreetingService;

impl GreetingService {
    pub fn greet(&self, name: &str) -> crate::Result<String> {
        if name.trim().is_empty() {
            return Err(crate::Error::EmptyName);
        }

        Ok(format!("Hello, {name}!"))
    }
}
