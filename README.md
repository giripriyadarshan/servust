# Servust

Servust is a simple CLI tool to create a new project from a template, kind of like `npx create-react-app` but for rust frameworks project.

[![crates.io version]][crates.io link] [![Crates.io Downloads]][crates.io link] [![crates.io license]][crates.io link] [![Github image]][Github status]


### Installation

Haven't published it anywhere yet, so you'll have to install it using `cargo build --release` and then copy the binary to your path.

### Usage

```bash
USAGE:
    servust [OPTIONS] --framework <FRAMEWORK> --orm <ORM> <NAME>

ARGS:
    <NAME>    The name of the server

OPTIONS:
    -d, --database <DATABASE>      database to be used (postgres, mysql, sqlite) default: postgres
    -f, --framework <FRAMEWORK>    library/framework to be used (actix, warp, axum, tonic)
    -h, --help                     Print help information
    -o, --orm <ORM>                ORM to be used (diesel, sea-orm)
    -V, --version                  Print version information
```


### Example
```bash
servust --framework actix --orm diesel my-server
```






[crates.io link]: https://crates.io/crates/servust
[crates.io version]: https://img.shields.io/crates/v/servust
[Crates.io Downloads]: https://img.shields.io/crates/d/servust
[crates.io license]: https://img.shields.io/crates/l/servust
[Github image]: https://github.com/giripriyadarshan/servust/workflows/ci/badge.svg
[Github status]: https://github.com/giripriyadarshan/servust/actions