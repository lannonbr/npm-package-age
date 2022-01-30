# NPM Package Age

A Rust CLI which if you provide a npm lockfile (package-lock.json to start), it will give you a listing of all of the packages & the last time each was published on npm.

To Run this, run `cargo build` and then run the outputted binary and pass it a filepath to a `package-lock.json` file. You can choose a format by adding the `--format` flag. Add the `--help` flag to list all available options.

Example:

```
./target/release/npm-package-age --file example/package-lock.json
```
