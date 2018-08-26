/// Implement the `Application` trait for the given type
// TODO: less hax way of doing this (move into `derive(Application)`?)
#[macro_export]
macro_rules! impl_application {
    ($app:tt) => {
        impl Application for $command {
            /// Parse command-line arguments from a string iterator
            pub fn from_args<A: IntoIterator<Item = String>>(into_args: A) -> Self {
                let args: Vec<_> = into_args.into_iter().collect();

                if args.len() == 1 {
                    match args[0].as_ref() {
                        "-h" | "--help" => {
                            print_usage!(Self);
                        }
                        "-V" | "--version" => {
                            print_package_version!();
                            ::std::process::exit(0);
                        }
                        _ => (),
                    }
                }

                Self::parse_args_default(args.as_slice()).unwrap_or_else(|e| {
                    match e.to_string().as_ref() {
                        // Show usage if no command name is given or if "help" is given
                        // TODO: don't gate on a string, handle the error properly
                        "missing command name" => {
                            print_usage!(Self);
                        }
                        string => eprintln!("{}: {}", args[0], string),
                    }

                    ::std::process::exit(2);
                })
            }

            /// Parse command-line arguments from the environment
            pub fn from_env_args() -> Self {
                let mut args = ::std::env::args();
                assert!(args.next().is_some(), "expected one argument but got zero");
                Self::from_args(args)
            }
        }
    };
}
