# Ticket Adder
A simple prepare-commit-msg git hook which checks the branch name you're in, attempts to extract a ticket number from it
and prepends it to the commit message, unless the message already contains it. This is more of an experiment than
anything else, so use at your own risk.

## How to use
Since the regex pattern is hardcoded, you'll have to build it yourself. Install Rust, clone/download the repo, change
the regex pattern to fit your needs (`TICKET_PATTERN` global constant), run `cargo build --release`, and throw the
resulting binary file found in `target/release` into your projects `.git/hooks` folder. Enjoy never having to remember
to add the ticket to a commit ever again.

This is my first time coding in Rust, so improvements are very welcome.

## TODO

*   Methods can be broken up further to improve readability
*   ~~Pass regex pattern from outside to avoid jumping through the build it yourself hoop~~

## Acknowledgements

The majority of the code is coming from Justin Worthe, who was generous to write a tutorial on how to write git hooks in
Rust, and already had code done to add the branch name to a commit message. My changes are fairly minimal.

https://code.worthe-it.co.za/rust-git-hooks.git/tree/src/bin/prepare-commit-msg.rs

https://www.worthe-it.co.za/blog/2017-08-29-writing-git-hooks-using-rust.html
