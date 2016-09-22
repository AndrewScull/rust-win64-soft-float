Simple example showing SSE instructions are generated for a target which
disables MMX and SSE and enables soft-float.

    cargo install xargo
    xargo build --target no-xmm

The generated code for `bar` includes SSE instructions to save and restore the
xmm6-15 registers for the [win64 ABI][1]. These instructions are not generated
when `extern "win64"` is removed. This only happens in this example for debug
builds, however, it occurs in release builds when the function contains cross-crate
function calls.

[1]: https://msdn.microsoft.com/en-us/library/ms235286.aspx
