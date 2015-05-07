OpenShift Rust Cartridge
========================

Runs [Rust](http://www.rust-lang.org) on [OpenShift](https://openshift.redhat.com/app/login) using downloadable cartridge support.  To install to OpenShift from the CLI (you'll need version 1.9 or later of rhc), run:

    rhc create-app myrust https://cartreflect-claytondev.rhcloud.com/reflect?github=FlaPer87/openshift-rust-cart

How it Works
------------

When you push code to your repo, a Git postreceive hook runs and invokes the bin/compile script.  This attempts to download a Rust environment using [rustup](https://github.com/rust-lang/rustup) for you into $OPENSHIFT_RUST_DIR/cache (A symlink is created in $OPENSHIFT_RUST_DIR/current).  Once the environment is setup, the cart runs

    cargo build --verbose --release

on a working copy of your source.  The main file that you run will have access to two environment variables, $HOST and $PORT, which contain the internal address you must listen on to receive HTTP requests to your application.

The application is then executed using:

    cargo run

Credits
-------

This cartridge is a fork of the [openshift-go-cart](https://github.com/smarterclayton/openshift-go-cart). This means you'll see many similarities with just some words replaced. :D

ToDo
----

- Add support for custom build dirs
- Support build/execution options
