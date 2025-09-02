```
cargo build
./target/debug/kura install ./crates/demo-lib
./target/debug/kura install ./crates/otherlib
./target/debug/kura run test.koto

or if you set the output of kura build in your PATH:
koto test.koto
```