//! Reusable plugin composition and public extension points.

mod error;
mod plugin;
mod service;

pub use error::{Error, Result};
pub use plugin::GreetingPlugin;
pub use service::GreetingService;
{% if macro_crate -%}
pub use {{ crate_name }}_macros::plugin_component;
{% endif -%}

pub mod prelude {
    //! Common imports for plugin consumers.

    pub use crate::{Error, GreetingPlugin, GreetingService, Result};
{% if macro_crate -%}
    pub use crate::plugin_component;
{% endif -%}
}
