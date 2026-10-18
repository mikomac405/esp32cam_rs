# esp32cam-rs
<a href="https://github.com/Kezii/esp32cam_rs/actions"><img alt="actions" src="https://github.com/Kezii/esp32cam_rs/actions/workflows/rust.yml/badge.svg"></a>

Rust esp32-cam examples

### Usage

After cloning the repo, download the esp32-camera component

```bash
git submodule update --init
```

The first build creates `src/wifi_config.rs` from `src/wifi_config.rs.example`; fill in your credentials there. The file is git-ignored, so they never get committed.

### Object detection model (Edge Impulse)

The can detector runs an Edge Impulse FOMO model. The Edge Impulse SDK is a git submodule (`components/object-detection/edge-impulse-sdk`), but the model itself is **not in this repository**: Edge Impulse deployments are licensed under the Edge Impulse Terms of Service and may not be redistributed. Export it yourself:

1. In Edge Impulse Studio, open the project → **Deployment** → **C++ library**, **Quantized (int8)**, EON Compiler on → **Build**.
2. Unzip and copy **only** `model-parameters/` and `tflite-model/` into `components/object-detection/` (keep this repo's `CMakeLists.txt`; the export's is for Zephyr).
3. Fix the export's timestamps (they can lie in the future and make ninja loop with `build.ninja still dirty`):

   ```bash
   command find components/object-detection -newermt "$(date -Iseconds)" -exec touch -c {} +
   ```

4. Make sure the SDK submodule matches the SDK version the export was built with, then rebuild ESP-IDF (cargo doesn't track C component changes):

   ```bash
   cargo clean -p esp-idf-sys && cargo build
   ```

Model settings the firmware expects: 128×128 input, grayscale, resize mode "Fit shortest axis".

## Telegram bot

```bash
cargo run --example telegram_bot

```

Insert the correct token and owner id, then use the /photo command to take a picture

<img width="480" alt="image" src="https://github.com/Kezii/esp32cam_rs/assets/3357750/5a61974f-a0dc-4bdd-94ad-81225c53ba59">

## Webserver

```bash
cargo run --example webserver
```

Connect to the ip in the log output, then access the /camera.jpg path to take a picture and have it delivered to your browser

## IDotMatrix

```bash
cargo run --example idotmatrix
```

If you have an idotmatrix display, the esp32-cam will deliver an image to it every few seconds

<img width="480" alt="image" src="https://github.com/Kezii/esp32cam_rs/assets/3357750/148e0a0e-3c06-47f0-9916-6f1ec76d67e5">


## credits:
https://github.com/esp-rs/std-training

https://github.com/jlocash/esp-camera-rs
