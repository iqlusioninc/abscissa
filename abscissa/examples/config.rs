// Example which loads `GlobalConfig` without using Abscissa's `Application`
// framework trait.

// This feature requires `lazy_static`
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate abscissa;
#[macro_use]
extern crate serde_derive;

use abscissa::GlobalConfig;

/// Configuration data to parse from TOML
// TODO: `derive(GlobalConfig)`
#[derive(Clone, Deserialize, Debug)]
pub struct MyConfig {
    /// Some value
    pub foo: String,

    /// Another value
    pub bar: String,

    /// Still another value!
    pub baz: String,
}

// Impl the `abscissa::GlobalConfig` trait on `MyConfig`, storing the
// configuration in the `MY_GLOBAL_CONFIG` static value.
// TODO: this should be a proc macro instead of macro_rules! i.e. `derive(GlobalConfig)`
impl_global_config!(MyConfig, MY_GLOBAL_CONFIG);

fn main() {
    // Load `MyConfig` from the given TOML file or exit
    MyConfig::set_from_toml_file_or_exit("example_config.toml");

    // Prints the `foo` record of `example_config.toml`
    print_foo_from_global_config();

    // Prints the `bar` record of `example_config.toml`
    print_bar_from_global_config();

    // We can update the global config!
    MyConfig::set_global(MyConfig {
        foo: "foo2".to_owned(),
        bar: "bar2".to_owned(),
        baz: "baz2".to_owned(),
    });

    // Prints `foo2` since we updated the global config
    print_foo_from_global_config();
}

/// Print the "foo" member of the global configuration
fn print_foo_from_global_config() {
    // This acquires a `RwLock` on the config and uses an RAII guard
    let config = MyConfig::get_global();
    println!("config.foo: {}", config.foo);
}

/// Print the "bar" member of the global configuration
fn print_bar_from_global_config() {
    let config = MyConfig::get_global();
    println!("config.bar: {}", config.foo);
}
