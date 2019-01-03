# packapp
pack a binary to MacOS .app bundle

## install
Recommended to install with `cargo install`:
```sh
cargo install packapp
```
to update to the latest version:
```sh
cargo install packapp --force
```


## usage
```sh
USAGE:
    packapp [OPTIONS] <BIN>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
        --display-name <DNAME>
        --icon <ICON>
        --identifier <IDENT>
        --name <NAME>
        --version <VERSION>

ARGS:
    <BIN>    the binary to pack
```

## example
to pack a binary with no settings
```sh
packapp yo
```
with settings
```sh
packapp --identifier com.company.yo --name yo --display-name YO --icon icon.icns --version "1.0.0" yo
```

