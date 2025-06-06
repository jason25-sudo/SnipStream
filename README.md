# SnipStream

SnipStream is an experimental screen and camera recorder built with the [Iced GUI framework](https://github.com/iced-rs/iced). The current implementation targets **Windows only** and includes placeholder logic for advanced editing features such as auto-cropping, background removal and automatic captions.

## Building

Building on Windows requires the standard MSVC toolchain and the `win32` dependencies that ship with it. Screenshots are saved using the `image` crate, which requires no additional system packages. Timestamped filenames are generated with the `chrono` crate.

Then you can build with Cargo:

```
cargo build
```

Running the binary will open a modern dark-themed window with **Record/Stop**, **Pause/Resume** and **Screenshot** buttons centered on the screen. Recordings and editing actions are still stubs, but screenshots are saved automatically to the `screenshots/` directory with timestamped names.
