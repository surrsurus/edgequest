# TODO

- Replace `String` with `&'static str`
- `pub fn load(path: &str) -> Config` should probably be something like `pub fn load(path: &str) -> Result<Config, WhateverThisIs>`
- Actually add `Default` impls