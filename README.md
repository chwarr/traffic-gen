# Traffic generators

Super simple pair of programs that read/write random 4K buffers over TCP.

I needed to put some load on a network link during the summer of 2021.
Instead of using something like `nc`, I decided to use this as an
opportunity to explore how to use [Rust](https://www.rust-lang.org/) and
[Tokio](https://tokio.rs/) to do socket I/O.

## License

Copyright 2021, G. Christopher Warrington

This set of tools is free software: you can redistribute it and/or modify it
under the terms of the GNU Affero General Public License Version 3 as
published by the Free Software Foundation.

This set of tools is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public
License for more details.

A copy of the GNU Affero General Public License Version 3 is included in the
file LICENSE at the root of the repository.
