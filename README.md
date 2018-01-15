# mapbox-expressions-to-sql

[![Build Status](https://travis-ci.org/stepankuzmin/rust-mapbox-expressions-to-sql.svg?branch=master)](https://travis-ci.org/stepankuzmin/rust-mapbox-expressions-to-sql)

Transform Mapbox GL style specification [decision expressions](https://www.mapbox.com/mapbox-gl-js/style-spec/#expressions-decision) to SQL `WHERE` clause conditions.

## Documentation
Documentation is [here](https://docs.rs/mapbox_expressions_to_sql)

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mapbox_expressions_to_sql = "0.1"
```

and this to your crate root:

```rust
extern crate mapbox_expressions_to_sql;
```

### Example

```rust
extern crate mapbox_expressions_to_sql;
use mapbox_expressions_to_sql::parse;

assert_eq!(parse(r#"["has", "key"]"#).unwrap(), "key IS NOT NULL");

assert_eq!(parse(r#"["==", "key", 42]"#).unwrap(), "key = 42");

assert_eq!(parse(r#"["in", "key", "v0", "v1", "v2"]"#).unwrap(), "key IN ('v0', 'v1', 'v2')");

assert_eq!(parse(r#"["all", ["==", "key0", "value0"], ["==", "key1", "value1"]]"#).unwrap(), "key0 = 'value0' AND key1 = 'value1'");
```

See [tests](https://github.com/stepankuzmin/rust-mapbox-expressions-to-sql/blob/master/src/lib.rs) for more examples.