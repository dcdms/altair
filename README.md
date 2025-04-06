# altair

I'm currently dealing with a distributed system and it's such boring to have **8 terminal tabs** running multiple microservices and other applications. 

So I've created this minimal CLI called `altair` that allows us to run multiple commands in a single process with a configuration file.

## Installation

> Altair was deployed to [crates.io](https://crates.io), so make sure you have `cargo` installed on your system before continuing. Follow [Cargo's installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you need to.

First, install `altair`:

```sh
cargo install altairsh
```

Good! Let's say we have two Bun applications in the folders `auth` and `catalog` and we want to run them in the same process. You only need to write the following `altair.yaml` file...

```yaml
commands:
  - name: 'auth.acme.com'
    run: 'bun run --cwd auth dev'

  - name: 'catalog.acme.com'
    run: 'bun run --cwd catalog dev'
```

Then run `altair` in the same working directory and see both applications running! You can terminate them via `ctrl + c`.