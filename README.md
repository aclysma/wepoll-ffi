# wepoll-rs

Bindings for wepoll (epoll on windows)

This crate provides unsafe bindings. Please refer to https://github.com/piscisaureus/wepoll for more details.

## Performance

The C++ code that is wrapped by these bindings is built at the same optimization level used for building the 
`wepoll-rs` crate. Most users of these bindings will benefit from building this crate with optimizations
enabled, even during development. To do that, place this in your Cargo.toml file.

```
[profile.dev.package."wepoll-rs"]
opt-level = 3
```

Enable optimizations for ALL upstream crates works too:

```
[profile.dev.package."*"]
opt-level = 3
```

## License

The bindings are licensed under either of

* BSD 2-clause ([LICENSE-BSD-2-CLAUSE](LICENSE-BSD-2-CLAUSE) or https://opensource.org/licenses/BSD-2-Clause)
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The BSD 2-clause license is also used by the wrapped library.

### Upstream Dependencies

Some dependencies may be licensed under other terms. These licenses include "ISC", "CC0-1.0", "BSD-2-Clause",
"BSD-3-Clause", and "Zlib". This is validated on a best-effort basis in every CI run using cargo-deny.

`wepoll` is licensed under BSD-2-clause:

wepoll - epoll for Windows
https://github.com/piscisaureus/wepoll

Copyright 2012-2020, Bert Belder <bertbelder@gmail.com>
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

* Redistributions of source code must retain the above copyright
  notice, this list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright
  notice, this list of conditions and the following disclaimer in the
  documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be licensed as above, without any additional terms or
conditions.

See [LICENSE-BSD-2-CLAUSE](LICENSE-BSD-2-CLAUSE) [LICENSE-APACHE](LICENSE-APACHE), and [LICENSE-MIT](LICENSE-MIT).
