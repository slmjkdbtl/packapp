# packapp
pack a binary to MacOS .app bundle

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

### install

use homebrew:

```sh
$ brew install slmjkdbtl/formulae/packapp
```

or use crates.io:

```sh
$ cargo install packapp
```

or clone and build locally:

```sh
$ git clone https://github.com/slmjkdbtl/packapp
$ cd packapp
$ cargo install --force --path .
```

or go to the [release](https://github.com/slmjkdbtl/packapp/releases) tab and download manually

### example
to pack a binary with no settings
```sh
packapp yo
```
with settings
```sh
packapp --identifier com.company.yo --name yo --display-name YO --icon icon.icns --version "1.0.0" yo
```

