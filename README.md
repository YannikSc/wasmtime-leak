# wasmtime_wasi memory leak

This projects shows a (presumably) memory leak in wasmtime_wasi when inheriting the env variables

## Try it out

I hope I don't have to show you how to clone the repository/get the code on your local machine, so I skip this part.

### Without memory leak

The wasi context is build with all the `inherit_*()` functions (namely: `inherit_stdout`, `inherit_stderr`, `inherit_stdin`, `inherit_args`) , except the `inherit_env()`

```
# Also happens with --debug
cargo run --release > /dev/null # the > /dev/null is used, as the example library is printing to the stdout
```

### With memory leak (yeah, its a feature, not a bug!)

In addition to the non-leak version, the `inherit_env()` is called on the builder.

```
# Also happens with --debug
cargo run --release --features leak > /dev/null # the > /dev/null is used, as the example library is printing to the stdout
```

