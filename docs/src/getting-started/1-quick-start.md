# Quick Start

## Installation

Currently, you can either download packages from the 

[GitHub releases page](https://github.com/giripriyadarshan/servust/releases)

or install from [crates.io](https://crates.io/crates/servust) using 
```bash
cargo install servust
```

## Usage

```properties
servust --framework <framework_name> --orm <ORM_name> --database <database_name> my-server
```

`--framework` [REQUIRED]

`--orm` [REQUIRED]

`--database` [OPTIONAL] - default postgres

check [Supported Frameworks](2-supported-frameworks.md) for more details