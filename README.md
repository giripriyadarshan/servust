# Servust

Servust is a simple CLI tool to create a new project from a template, kind of like `npx create-react-app` but for rust frameworks project.

[![crates.io version]][crates.io link] [![Crates.io Downloads]][crates.io link] [![crates.io license]][crates.io link] [![Github image]][Github status]

Support: 

[![Matrix image]][Matrix link]

### [Documentation](https://servust.giripriyadarshan.com)

---

### Installation

Currently only available on crates.io

```bash
cargo install servust
```

### Usage

```bash
Usage: servust [OPTIONS] --framework <FRAMEWORK> --orm <ORM> <NAME>

Arguments:
  <NAME>  The name of the server

Options:
  -f, --framework <FRAMEWORK>  library/framework to be used (actix, warp, axum, tonic)
  -o, --orm <ORM>              ORM to be used (diesel, sea-orm)
  -d, --database <DATABASE>    database to be used (postgres, mysql, sqlite) default: postgres
  -h, --help                   Print help information
  -V, --version                Print version information
```


### Example
```bash
servust --framework actix --orm diesel --database postgres my-server
```

![CLI Run](https://github.com/giripriyadarshan/servust/blob/main/assets/img/servust_cli.gif?raw=true)



### Support

#### Frameworks
- [x] [actix](https://actix.rs)
- [x] [tonic](https://github.com/hyperium/tonic)
- [x] [salvo](https://salvo.rs)
- [x] [axum](https://github.com/tokio-rs/axum)
- [ ] rocket
- [ ] warp

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