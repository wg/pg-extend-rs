
[![Build Status](https://travis-ci.org/bluejekyll/pg-extend-rs.svg?branch=master)](https://travis-ci.org/bluejekyll/pg-extend-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](LICENSE-APACHE)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=bluejekyll/pg-extend-rs)](https://dependabot.com)
[![](http://meritbadge.herokuapp.com/pg-extend)](https://crates.io/crates/pg-extend)

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

`PG_INCLUDE_PATH=[/path/to/postgres]/include/server # e.g. /usr/local/pgsql/include/server`

This environment variable is also required for the dynamic libraries to compile:

`RUSTFLAGS="-C link-arg=-undefineddynamic_lookup"`

This informs the linker that some of the symbols for postgres won't be available until runtime on the dynamic library load.

## Examples

- [adding](https://github.com/bluejekyll/pg-extend-rs/tree/master/examples/adding)
- [panicking](https://github.com/bluejekyll/pg-extend-rs/tree/master/examples/panicking)

## Features

To use the postgres allocator, the feature `pg_allocator` must be defined and enabled in the implementing crate.
