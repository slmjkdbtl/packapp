# packapp
pack a binary to MacOS .app bundle

### usage
```sh
USAGE:
    packapp [OPTIONS] <bin>

Options:
    --res             stuff to copy into the "Resources" folder
    --framework       stuff to copy into the "Frameworks" folder
    --icon            icon file
    --ident           app identifier
    --name            app name
    --display-name    app display name
    --version         app version
    --agent           if app is agent (won't show icon in dock)
    --high-res        if app should render in high resolution
    --filetype        file types that can be opened with this bundle
    --out             output path
    --verbose         verbose output
    --help            display usage information

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

