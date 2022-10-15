# Servust

Generate quick templates of rust backend servers 

[![crates.io version]][crates.io link] [![Crates.io Downloads]][crates.io link] [![crates.io license]][crates.io link] [![Github image]][Github status]

# Introduction

Servust is a simple CLI tool to create a new project from a template, kind of like `npx create-react-app` but for rust frameworks project.

* Multi framework/library support
* Bare minimum setup
* Lightweight and fast CLI (~2mb binary size)
* Takes few seconds to generate a project

### Example
 <!-- Had to make the servust binary standout as a different color-->
```properties
servust --framework actix --orm diesel --database postgres my-server
```

![CLI Run](https://github.com/giripriyadarshan/servust/blob/main/assets/img/servust_cli.gif?raw=true)



# Project Status

- Heavily under development.
- Supports only handful of frameworks and libraries (check [Supported Frameworks/Libraries](getting-started/2-supported-frameworks.md)).
- Currently no option to create server without ORM/Databases.










[crates.io link]: https://crates.io/crates/servust
[crates.io version]: https://img.shields.io/crates/v/servust
[Crates.io Downloads]: https://img.shields.io/crates/d/servust
[crates.io license]: https://img.shields.io/crates/l/servust
[Github image]: https://github.com/giripriyadarshan/servust/workflows/ci/badge.svg
[Github status]: https://github.com/giripriyadarshan/servust/actions
[Matrix image]: https://img.shields.io/matrix/giripriyadarshan-servust:matrix.org
[Matrix link]: https://matrix.to/#/#giripriyadarshan-servust:matrix.org


