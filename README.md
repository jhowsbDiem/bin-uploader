# bin-uploader — ESP32 binaries ready to flash ⚡

[![Target](https://img.shields.io/badge/Target-ESP32-blue.svg)](https://www.espressif.com/en/products/socs/esp32)
[![Flasher](https://img.shields.io/badge/Flasher-esptool-orange.svg)](https://github.com/espressif/esptool)
[![Version](https://img.shields.io/badge/Version-0.1.0-brightgreen.svg)](https://github.com/jhowsbDiem/bin-uploader)

bin-uploader is a terminal tool, for Windows and Linux, that finds the latest
ESP32 build of an Arduino sketch, compiled by Arduino IDE 2.x or arduino-cli,
and leaves its binaries ready to flash with a single command — no hunting for
the build folder, renaming files or editing a flash script by hand.

---

## 🤝 Contributing

Workflow, commit and comment conventions are in [`CONTRIBUTING.md`](CONTRIBUTING.md).

---

## 📊 Project Status

### ⏳ In Progress
* **Design:** how the tool finds a build, names the binaries and flashes them.

### 🗺️ Planned Next Steps
* **Build detection:** the latest build of a sketch, from Arduino IDE 2.x or arduino-cli.
* **Ready-to-flash binaries:** bootloader, partition table and application collected from the same build, under fixed names.
* **One-command flashing:** the board is flashed from the terminal, on Windows and Linux.

---

## ✒️ Authors and License

* **Developer:** Jonathan dos Santos Bezerra.
* **License:** MIT — see [LICENSE](LICENSE).
