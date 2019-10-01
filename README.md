# packapp
pack a binary to MacOS .app bundle

### install
Recommended to install with `cargo install`:
```sh
cargo install packapp
```
to update to the latest version:
```sh
cargo install packapp --force
```

### usage
```sh
USAGE:
    packapp [OPTIONS] <bin>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -d, --display-name <display-name>
    -f, --frameworks <frameworks>...
    -c, --icon <icon>
    -i, --identifier <identifier>
    -n, --name <name>
    -r, --resources <resources>...
    -v, --version <version>

ARGS:
    <bin>
```

### example
to pack a binary with no settings
```sh
packapp yo
```
with settings
```sh
packapp --identifier com.company.yo --name yo --display-name YO --icon icon.icns --version "1.0.0" yo
```

