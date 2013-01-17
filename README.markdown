RMCR
====

A simple Monte Carlo -style pathtracer written in Rust. Doesn't do any
nice things like MLT, bidirectional or even russian roulette. It is threaded,
though.

Usage
-----

Tested on
<pre>
rustc 0.6 (09bb07b 2012-12-24 18:29:02 -0800)
host: x86_64-unknown-linux-gnu
</pre>

Please do
<pre>
$ rustc --opt-level=3 main.rs
$ ./main
</pre>

Note: this will cause an assertion failure and crash on exit presumably due to some Rust bug.
(The code doesn't contain any `unsafe` blocks.) It will produce a file named framethreaded.ppm
containing the image.
