# Servust

Servust is a simple CLI tool to create a new project from a template, kind of like `npx create-react-app` but for rust frameworks project.

[![crates.io version]][crates.io link] [![Crates.io Downloads]][crates.io link] [![crates.io license]][crates.io link] [![Github image]][Github status]

Support: 

[![Matrix image]][Matrix link]


### Installation

Currently only available on crates.io

```bash
cargo install servust
```

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
servust --framework actix --orm diesel --database postgres my-server
```

![CLI Run](https://github.com/giripriyadarshan/servust/blob/beta/assets/img/servust_cli_running.png?raw=true)



### Support

#### Frameworks
- [x] [actix](https://actix.rs)
- [x] [tonic](https://github.com/hyperium/tonic)
- [x] [salvo](https://salvo.rs)
- [ ] rocket
- [ ] warp
- [ ] axum

#### ORMs
- [x] Diesel.rs
- [x] Sea-ORM

#### Databases
- [x] Postgres
- [x] MySQL
- [x] SQLite

#### Types
- [x] With ORM + Database
- [ ] Without ORM / Database
- [ ] With Database only






[crates.io link]: https://crates.io/crates/servust
[crates.io version]: https://img.shields.io/crates/v/servust
[Crates.io Downloads]: https://img.shields.io/crates/d/servust
[crates.io license]: https://img.shields.io/crates/l/servust
[Github image]: https://github.com/giripriyadarshan/servust/workflows/ci/badge.svg
[Github status]: https://github.com/giripriyadarshan/servust/actions
[Matrix image]: https://img.shields.io/matrix/giripriyadarshan-servust:matrix.org
[Matrix link]: https://matrix.to/#/#giripriyadarshan-servust:matrix.org