> Note: This project started as a partial implementation of the API and features provided by [mbtileserver](https://github.com/consbio/mbtileserver) written in Go by [Brendan Ward](https://github.com/brendan-ward). It might diverge from that project in the future.

# mbtileserver

[![Crates.io](https://img.shields.io/crates/v/mbtileserver.svg)](https://crates.io/crates/mbtileserver)
[![Coverage Status](https://coveralls.io/repos/github/maplibre/mbtileserver-rs/badge.svg)](https://coveralls.io/github/maplibre/mbtileserver-rs)

_Tested with rust 1.60_

A simple Rust-based server for map tiles stored in mbtiles format.


## Installation
```
apt install -y build-essential libsqlite3-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cd /var/www/intern/rust-mbtileserver
cargo build --release
mkdir -p /var/www/tiles
/var/www/intern/rust-mbtileserver/target/release/mbtileserver --allowed-hosts "*" --directory "/var/www/tiles" --port 3636 &
```

### Getting Involved

Join the #maplibre slack channel at OSMUS: get an invite at https://osmus-slack.herokuapp.com/

## Usage

Run `mbtileserver --help` for a list and description of the available flags:

```
mbtileserver 0.1.7
A simple mbtiles server

USAGE:
    mbtileserver [OPTIONS]

OPTIONS:
        --allow-reload-api
            Allow reloading tilesets with /reload endpoint

        --allow-reload-signal
            Allow reloading timesets with a SIGHUP

        --allowed-hosts <ALLOWED_HOSTS>
            "*" matches all domains and ".<domain>" matches all subdomains for the given domain
            [default: localhost,127.0.0.1,[::1]]

    -d, --directory <DIRECTORY>
            Tiles directory [default: ./tiles]

        --disable-preview
            Disable preview map

        --disable-watcher
            Disable fs watcher for automatic tileset reloading

    -h, --help
            Print help information

    -H, --header <HEADER>
            Add custom header. Can be used multiple times.

    -p, --port <PORT>
            Server port [default: 3000]

        --reload-command <RELOAD_COMMAND>
            Command to run on tileset reload

        --reload-interval <RELOAD_INTERVAL>
            An interval at which tilesets get reloaded

    -V, --version
            Print version information
```

Run `mbtileserver` to start serving the mbtiles in a given folder. The default folder is `./tiles` and you can change it with `-d` flag.
The server starts on port 3000 by default. You can use a different port via `-p` flag.

You can adjust the log level by setting `RUST_LOG` environment variable. Possible values are `trace`, `debug`, `info`, `warn`, `error`.

### Endpoints

| Endpoint                                                     | Description                                                                    |
|--------------------------------------------------------------|--------------------------------------------------------------------------------|
| /reload                                                      | reloads tilesets from directory (if enabled with `--allow-reload`)             |
| /services                                                    | lists all discovered and valid mbtiles in the tiles directory                  |
| /services/\<path-to-tileset>                                 | shows tileset metadata                                                         |
| /services/\<path-to-tileset>/map                             | tileset preview                                                                |
| /services/\<path-to-tileset>/tiles/{z}/{x}/{y}.<tile-format> | returns tileset tile at the given x, y, and z                                  |
| /services/\<path-to-tileset>/tiles/{z}/{x}/{y}.json          | returns UTFGrid data at the given x, y, and z (only for tilesets with UTFGrid) |

## Docker

You can test this project by running `docker-compose up`. It starts a server on port 3000 and serves the tilesets in `./tiles` directory.
