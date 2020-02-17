# `f411`
Support crate for the [STM32F411E-DISCO](https://www.st.com/en/evaluation-tools/32f411ediscovery.html).

[Reference Manual (EN)](https://www.st.com/content/ccc/resource/technical/document/user_manual/e9/d2/00/5e/15/46/44/0e/DM00148985.pdf/files/DM00148985.pdf/jcr:content/translations/en.DM00148985.pdf)

Based heavily on [japaric/f3](https://github.com/japaric/f3)

## Board Features
- `STM32F411VET6` microcontroller (Cortex-M4F) featuring 512 KiB of Flash memory
- `LQFP100` 128 KiB of RAM
- `L3GD20`: ST MEMS motion sensor 3-axis digital output gyroscope
- `LSM303DLHC`: ST MEMS system-in-package featuring a 3D digital linear acceleration sensor and a 3D digital magnetic sensor
- `MP45DT02`: ST MEMS audio sensor, omnidirectional digital microphone
- `CS43L22`: audio DAC with integrated class D speaker driver
- 8 LEDs:
    - `LD1` red/green - USB communication
    - `LD2` red - 3.3V power on
    - 4 user LEDs
        - `LD3:PD13` Orange / North
        - `LD5:PD14` Red / East
        - `LD6:PD15` Blue / South
        - `LD4:PD12` Green / West
    - 2 USB OTG LEDs
        - `LD7:PA9` green - VBus
        - `LD8:PD5` red - over-current
- 2 buttons (user and reset)
- USB OTG with micro-AB connector

- ST-LINK/V2
- Board power supply: through USB bus or from an external 5V supply voltage
- External application power supply: 3V and 5V

<!-- ## [Documentation](https://docs.rs/f4) -->

## [Change log](CHANGELOG.md)

## License
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
