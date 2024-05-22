# A Rust Hardware Abstraction Library (HAL) for the Texas Instruments [AM64x series processors](https://www.ti.com/product/AM6442)

🚧 **WORK IN PROGRESS** 🚧

I'm implementing this HAL for a project I want to develop. I'm likely only targeting a partial implementation long-term. Please reach out if you need support for additional peripherals or would like
to contribute.

# SVD File

The SVD file the PAC is generated from is wonky and I had to edit the generated PAC to get things to work.

# License

This software is licensed under the terms of the MIT or APACHE-2.0 license.

# Peripheral Support

## MAIN Domain Peripherals
- [ ] Timers
  - 🚧 **Work in Progress**
- [ ] GPIO
  - 🚧 **Work in Progress**
- [ ] Data Movement Subsystem (DMSS)
- [ ] Mailbox (MAILBOX)
- [ ] Spinlock (SPINLOCK)
- [ ] ADC
- [ ] I2C
- [ ] SPI
- [ ] UART
  - Pins and pin functions implemented
- [ ] PCIE
- [ ] Region-based Address Translation Module (RAT)
- [ ] SERDES
- [ ] USB 3.0 Subsystem (USBSS)
- [ ] General Purpose Memory Controller (GPMC)
- [ ] Error Location Module (ELM)
- [ ] Flash Subsystem (FSS)
- [ ] Octal Serial Peripheral Interface (OSPI)
- [ ] Multi-Media Card / Secure Digital Interface (MMCSD)
- [ ] Controller Area Network (MCAN)
- [ ] 3-port Gigabit Ethernet Switch (CPSW3G)
  - Unplanned
- [ ] Enhanced Capture Module (ECAP)
  - Unplanned
- [ ] Enhanced Pulse-Width Modulation Module
  - Unplanned
- [ ] Enhanced Quadrature Encoder Pulse Module (EQEP)
  - Unplanned
- [ ] Fast Serial Interface Receiver (FSI_RX)
  - Unplanned
- [ ] Faster Serial Interface Transmitter (FSI_TX)
  - Unplanned

## MCU Domain Peripherals (Unsupported)