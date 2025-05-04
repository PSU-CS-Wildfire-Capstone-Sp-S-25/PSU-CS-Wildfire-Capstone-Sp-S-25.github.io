# PSU-CS-Wildfire-Capstone-Sp-S-25.github.io

## Catherine Nemec and Max Rego

This is a webserver written in Rust for hosting the Wildfire Lab's website(s). Currently, it just serves a basic homepage.

For the sake of making a simple website for showing off our ideas to the sponsor, this should suffice.


# Usage
To run the server locally, you need Rust.
Install Rust (guide [here](https://www.rust-lang.org/tools/install)) and make sure `cargo` works. Acquire a local copy of the repository and, from the base of the repository directory, run
```
cargo run
```
to begin serving the website at [localhost:3000](127.0.0.1:3000).

Rust build artifacts can accumulate in your storage - to remove them, periodically run`cargo clean` from the repository directory.

Please report any bugs in the `#website` channel on our Discord.


The website is NOT currently live through GH Pages since it's not ready for the public. If it were, you could access it at [PSU-CS-Wildfire-Capstone-Sp-S-25.github.io](PSU-CS-Wildfire-Capstone-Sp-S-25.github.io)
