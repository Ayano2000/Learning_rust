This is my repository for learning the rust language.

This will contain very basic programmes while I familiarize myself with the languages nuances
before hopefully moving on to some bigger projects.

Cannot get the Cargo.toml to build or run any projects at this moment so those will be excluded for now.

Fixed cargo.toml not allowing compilation when you try and build or run with it. you need the command "CARGO_INCREMENTAL=0"

can be built with : "CARGO_INCREMENTAL=0 cargo build"
can be run with : "CARGO_INCREMENTAL=0 cargo run"