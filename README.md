## Disclaimer
This is a minimal standard library for rust aimed for making small executable. IT IS NOT A DROP IN REPLACEMENT for the rust standard library! You have been advised.

# Alternative Standard Library
The main objective of this library is to provide a small useful functionalities subset of the default std library that ships with rust. It works in the `#![no_std]` mode for building small sized executables/shared objects.

As of version 0.2.0 it offers the following data structures:
* `Box<T>`
* `Unique<T>`
* `Vector<T>` (growable arrays)
* `HashMap<T>`
* `String`
* `File` (only covers portions of libc)
* `error!` and `errorn!`
* `printf!` and `printfn!`

## License
Copyright (c) 2020, Wael El Oraiby
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of the author(s) nor the
      names of its contributors may be used to endorse or promote products
      derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL WAEL EL ORAIBY OR OTHER AUTHORS BE LIABLE FOR ANY
DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

## Credits
The hashmap implementation is taken from Google's skia and the original copyright is 3 clause BSD license.