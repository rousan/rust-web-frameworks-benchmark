//! A Hello World example application for working with Gotham.
use gotham::state::State;

/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "Hello, World")
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let addr = "127.0.0.1:8081";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}
