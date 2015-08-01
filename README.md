# NCL

[![Build Status](https://travis-ci.org/nukomeet/ncl.svg?branch=master)](https://travis-ci.org/nukomeet/ncl)

Simple configuration format based on UCL and HCL (just rewritten to Rust).

## Syntax

```
num = 42
str = "str"
bool = true
object = {
  num = 666
}
```

## TODO

- [ ] numerics (change to floats, currently `u64`)
- [x] booleans
- [x] strings
- [x] objects
- [x] comments
- [ ] subkey notation (`obj "key" {}`)
- [ ] add Mustache support for external variables/functions
- [ ] string escapes characters
- [ ] arrays
- [ ] auto-arrays

## Licence

See [`LICENCE`](LICENCE).
