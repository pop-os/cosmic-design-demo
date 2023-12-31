import 'just/rust.just'
import 'just/packaging.just'

rootdir := ''
prefix := '/usr'

# Name of the application's binary
bin-name := 'cosmic-design-demo'

# The AppID of the application
app-id := 'com.system76.CosmicDesignDemo'

# Application binary executable source and install destination
bin-src := 'target' / 'release' / bin-name
bin-dst := clean(rootdir / prefix) / 'bin' / bin-name

# Desktop file source and install destination
desktop-file := app-id + '.desktop'
desktop-src := 'resources' / desktop-file
desktop-dst := clean(rootdir / prefix) / 'share' / 'applications' / desktop-file

# Recipe for compiling the application
build *args:
    cargo build {{args}}

run *args:
    cargo run --release {{args}}

install: (install-bin bin-src bin-dst) (install-file desktop-src desktop-dst)