OpenShift Rust Cartridge
========================

Runs [Rust](http://www.rust-lang.org) on [OpenShift](https://openshift.redhat.com/app/login) using downloadable cartridge support. 

Once the app is created, you'll have a ".rustdir" file in the root of your repo.

When you push code to the repo, the cart will compile your package using <code>cargo build --release</code>.

If you want to serve web requests (vs. running in the background), you'll need to listen on the ip address and port that OpenShift allocates - those are available as HOST and PORT in the environment.

This default "main.rs" file is a simple "hello, world" web service that uses nickle.rs.

Any log output will be generated to <code>$OPENSHIFT_RUST_LOG_DIR</code> on your OpenShift gear


Build
-----

When you push code to your repo, a Git postreceive hook runs and invokes the bin/compile script.  This attempts to download a Rust environment using [rustup](https://github.com/rust-lang/rustup) for you into $OPENSHIFT_RUST_DIR/cache (A symlink is created in $OPENSHIFT_RUST_DIR/current).  Once the environment is setup, the cart runs

    cargo build --verbose --release

on a working copy of your source.  The main file that you run will have access to two environment variables, $HOST and $PORT, which contain the internal address you must listen on to receive HTTP requests to your application.

The application is then executed using:

cargo run
