# Rust based postgres extension

The main things provided by this crate are some macros that help with writing Postgres extensions in Rust.

The objective (not all these are yet implemented):

- Automatic type conversions, see `PgDatum` and `TryFromPgDatum` to `Into<PgDatum>`
- `pg_magic` macro for declaring libraries as Postgres extensions
- `pg_extern` attribute for wrapping Rust functions in Postgres C style definitions
- panic handlers for conversion into Postgres errors
- allocator that uses Postgres `palloc` allocator and `pfree`
- *tbd* integrate postgres error logs with `log`
- *tbd* support all Datum types
- *tbd* support table like returns and manipulation
- *tbd* generators for the psql scripts to load functions

## Building

First install Postgres. Once installed, this environment variable is required:

`PG_INCLUDE_PATH=/path/to/postgres]/include/server # e.g. /usr/local/pgsql/include/server`

This environment variable is also required for the dynamic libraries to compile:

`RUSTFLAGS="-C link-arg=-undefineddynamic_lookup"`

This informs the linker that some of the symbols for postgres won't be available until runtime on the dynamic library load.

## Running the integration tests

Standard tests can be run with the normal `cargo test`, but the integration tests are a little more involved. They require a connection to an actual Postgres DB. These instructions were performed on macOS. Create a DB in postgres to be use. In this example a DB was created in the `/usr/local/var/posgres` path, with the name `postgres`.

To run the test must know the DB name to use, the DB must be running, and then the tests can be run:

```shell
export RUSTFLAGS="-C link-arg=-undefineddynamic_lookup"
export PG_INCLUDE_PATH=$(pg_config --includedir)/server
export POSTGRES_TEST_DB=postgres

pg_ctl -D /usr/local/var/postgres start
cargo test
```

## Examples

- [adding](https://github.com/bluejekyll/pg-extend-rs/tree/master/examples/adding)
- [panicking](https://github.com/bluejekyll/pg-extend-rs/tree/master/examples/panicking)

## Features

To use the postgres allocator, the feature `pg_allocator` must be defined and enabled in the implementing crate.