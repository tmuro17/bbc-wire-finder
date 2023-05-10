_default:
    @just --list

target := 'thumbv7em-none-eabihf'
binary_name := 'bbc-wire-finder'

build *flags:
    cargo build --target {{target}} {{flags}}

clippy:
    cargo clippy --target {{target}}

check:
    cargo readobj --target {{target}} --bin {{binary_name}} -- --file-headers

flash *flags:
    cargo embed --target {{target}} {{flags}}

debug:
    arm-none-eabi-gdb target/{{target}}/debug/{{binary_name}}

size *flags:
    cargo size --target {{target}} {{flags}} -- -A
