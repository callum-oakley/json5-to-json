# j5j

Reads [JSON5] from one or more files and prints it as plain old JSON.

Based on `json5-to-json by @callum-oakley.

    $ cargo install json5-to-json
    $ echo "{ hello: 'world' }" | j5j
    {"hello":"world"}

[JSON5]: https://json5.org/
[json5-rs]: https://github.com/callum-oakley/json5-rs
[Serde JSON]: https://github.com/serde-rs/json
