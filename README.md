# SnipStream

SnipStream is an experimental screen and camera recorder built with the [Iced GUI framework](https://github.com/iced-rs/iced). The current implementation includes placeholder logic for advanced editing features such as auto-cropping, background removal and automatic captions.

## Building

The project requires system libraries for the `scrap` and `iced` dependencies. On Debian based systems you may need to install

```
apt-get install libxcb-shape0-dev libxcb-xfixes0-dev libxcb-randr0-dev libxcb-shm0-dev
```

Screenshots are saved using the `image` crate, which requires no additional system
packages. Timestamped filenames are generated with the `chrono` crate, which is
pure Rust and needs no extra libraries.

Then you can build with Cargo:

```
cargo build
```

Running the binary will open a modern dark-themed window with **Record/Stop** and **Screenshot** buttons centered on the screen. Recordings are still stubs, but screenshots are saved automatically to the `screenshots/` directory with timestamped names.
