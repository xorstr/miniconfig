# miniconfig

A minimalistic config file crate written in Rust.

## Overview

- **Lua configs** (requires `"lua"` feature).

    Main format for human-readable config files with nested array/table support.

    Piggybacks on the Lua interpreter both as a parser and as runtime data representation.

    May be used directly as a Lua representation within an `rlua` `Context`, or be serialized for dynamic or read-only use to decouple itself from the Lua state.

    **Data**: text file representing a valid Lua script, declaring a root config table with string keys as a collection of named global variables, including nested config arrays/tables represented by Lua tables. Only a subset of Lua types / features are supported.

    **Runtime**: internally represented by a root Lua table reference. Provides a mutable config interface. Can add/modify/remove values.

    **Serialization**: to string Lua script, to binary config (requires `"bin"` feature).

    **Example**:

    ``` lua
    array_value = { 54, 12, 78.9 } -- array_value
    bool_value = true
    float_value = 3.14
    int_value = 7
    string_value = "foo"
    table_value = {
        bar = 2020,
        baz = "hello",
        foo = false,
    } -- table_value
    ```

- **Dynamic configs** (requires `"dyn"` feature).

    Main format for runtime representation of dynamic configs, or an intermediate representation for Lua configs (after deserialization) / binary configs (before serialization).

    **Data**: if `"ini"` feature is enabled - a text file representing a valid `.ini` config, declaring a root config table with string keys and a number of sections - nested tables. Unquoted values, if allowed, are parsed as booleans / integers / floats / strings, in order. Quoted values, if enbaled, are always treated as strings.

    **Runtime**: internally represented by a root hashmap with string keys. Provides a mutable config interface. Can add/modify/remove values.

    **Serialization**: to string Lua script (requires `"lua"` feature), to binary config (requires "bin" feature).

- **Binary configs** (requires `"bin"` feature).

    Main format for runtime representation of read-only configs with nested array/table support.

    **Data**: raw byte blob. Generated by serializing a Lua config (requires `"lua"` feature), dynamic config (requires `"dyn"` feature), or by using the provided writer API.

    **Runtime**: wrapper over the raw byte blob. Provides a read-only config interface. Cannot add/modify/remove values.

    **Serialization**: to string Lua script (requires `"lua"` feature).

## Example

See `example.rs`.

## Building

Requires some path dependencies in the parent directory - see `Dependencies` section.

## Features

- `"lua"` (enabled by default) - adds support for Lua configs.
- `"dyn"` (enabled by default) - adds support for dynamic configs.
- `"bin"` (enabled by default) - adds support for binary configs / serialization.
- `"ini"` (enabled by default) - adds support for parsing `.ini` config strings to dynamic configs.

## Dependencies

- If "lua" feature is enabled (it is by default), [`rlua`](https://crates.io/crates/rlua) and [`rlua_ext`](https://github.com/alex05447/rlua_ext) as a path dependency (TODO - github dependency?).

## Problems / missing features

Despite the fact that all configs implement a common interface, it is currently impossible to implement a Rust trait to encapsulate that
due to Rust not having GAT's (generic associated types) at the moment of writing.

As a result, some code is duplicated internally in the crate, and the crate users will not be able to write code generic over config implementation.
