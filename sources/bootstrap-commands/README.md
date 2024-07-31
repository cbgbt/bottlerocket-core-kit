# bootstrap-commands

Current version: 0.1.0

## Bootstrap commands

bootstrap-commands ensures that Bootstrap Commands are executed as defined in the system
settings. It is called by bootstrap-commands systemd service which runs prior to the
execution of bootstrap-containers.

Each bootstrap command is a set of commands. The service queries the API for the settings, then
configures the system by going through all the bootstrap commands in lexicographical order and
running all the commands inside it.

## Example:
Given a bootstrap command called `001-test-bootstrap-commands` with the following configuration:

[settings.bootstrap-commands.001-test-bootstrap-commands]
commands = [[ "apiclient", "set", "motd=helloworld"]]
essential = true
mode = "always"

This would set `/etc/motd` to "helloworld".

## Colophon

This text was generated using [cargo-readme](https://crates.io/crates/cargo-readme), and includes the rustdoc from `src/main.rs`.
