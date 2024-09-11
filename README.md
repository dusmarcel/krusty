<div align="center">

  <h1><code>Frust</code></h1>

  <strong>Webapp, die nichts Sinnvolles tut....</strong>
</div>

## About

...aber vielleicht irgendwann mal tun wird.

## Installation

Da Frust vollständig clientseitig ausgeführt wird, sind serverseitig keine besonderen Vorbereitungen erforderlich. Die Installation sollte auf jedem üblichen Webserver möglich sein. Für die Installation wird [Rust](https://www.rust-lang.org/tools/install) benötigt, ferner das Target wasm32-unknown-unknown:

```console
$ rustup target add wasm32-unknown-unknown
```

Trunk muss auch noch installiert werden:

```console
$ cargo install trunk
```

Die benötigten JavaScript- und WASM-Dateien werden erzeugt mit:

```console
$ trunk build --release
```

trunk erzeugt einen Unterordner „dist“. Dessen Inhalt muss jetzt nur noch in das gewünschte Verzeichnis des Websververs verschoben oder kopiert werden.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
