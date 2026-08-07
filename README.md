# {{ project-name }}

A reusable, protocol-neutral Upwell plugin generated with `cargo upwell init` or
`cargo generate`.

`GreetingPlugin` contributes a real DI component. Applications opt in explicitly:

```rust,ignore
upwell::app! {
    pub app Application {
        name: "example",
        protocol: (),
        plugins: [{{ crate_name }}::GreetingPlugin],
    }
}
```

Set `macro_crate = true` when generating to add a companion proc-macro crate and re-exported
`#[plugin_component]` extension point.

## License

Generated projects are configured for `MIT OR Apache-2.0`. Replace that package metadata if your
project uses another license. The template repository itself is available under MIT.
