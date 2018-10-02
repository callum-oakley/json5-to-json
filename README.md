Reads [JSON5][] on stdin, writes plain JSON on stdout. A thin wrapper around
[json5-rs][].

    $ cargo install json5-to-json
    $ echo "{ hello: 'world' }" | json5-to-json
    {"hello":"world"}

[JSON5]: https://json5.org/
[json5-rs]: https://github.com/callum-oakley/json5-rs
