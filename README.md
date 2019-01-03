# packapp
pack a binary to MacOS .app bundle

## install
```bash
cargo install packapp
```

## usage
```bash
# pack a binary named 'yo'
packapp yo
# pack a binary with settings
packapp --identifier com.company.yo --name yo --display-name YO --icon icon.icns --version "1.0.0" yo
```

