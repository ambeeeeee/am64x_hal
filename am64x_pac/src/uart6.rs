#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_mem: [u8; 0x04],
    _reserved_1_mem: [u8; 0x04],
    _reserved_2_mem: [u8; 0x04],
    mem_lcr: MemLcr,
    _reserved_4_mem: [u8; 0x04],
    _reserved_5_mem: [u8; 0x04],
    _reserved_6_mem: [u8; 0x04],
    _reserved_7_mem: [u8; 0x04],
    mem_mdr1: MemMdr1,
    mem_mdr2: MemMdr2,
    _reserved_10_mem: [u8; 0x04],
    _reserved_11_mem: [u8; 0x04],
    _reserved_12_mem: [u8; 0x04],
    _reserved_13_mem: [u8; 0x04],
    _reserved_14_mem: [u8; 0x04],
    mem_acreg: MemAcreg,
    mem_scr: MemScr,
    mem_ssr: MemSsr,
    mem_eblr: MemEblr,
    _reserved19: [u8; 0x04],
    mem_mvr: MemMvr,
    mem_sysc: MemSysc,
    mem_syss: MemSyss,
    mem_wer: MemWer,
    mem_cfps: MemCfps,
    mem_rxfifo_lvl: MemRxfifoLvl,
    mem_txfifo_lvl: MemTxfifoLvl,
    mem_ier2: MemIer2,
    mem_isr2: MemIsr2,
    mem_freq_sel: MemFreqSel,
    mem_abaud_1st_char: MemAbaud1stChar,
    mem_baud_2nd_char: MemBaud2ndChar,
    mem_mdr3: MemMdr3,
    mem_tx_dma_threshold: MemTxDmaThreshold,
    mem_mdr4: MemMdr4,
    mem_efr2: MemEfr2,
    mem_ecr: MemEcr,
    mem_timeguard: MemTimeguard,
    mem_timeoutl: MemTimeoutl,
    mem_timeouth: MemTimeouth,
    mem_sccr: MemSccr,
    _reserved_40_mem: [u8; 0x04],
    mem_mar: MemMar,
    mem_mmr: MemMmr,
    mem_mbr: MemMbr,
}
impl RegisterBlock {
    #[doc = "0x00 - The transmitter section consists of the transmit holding register (THR) and the transmit shift register. The transmit holding register is actually a 64-byte FIFO. The LH writes data to the THR. The data is placed into the transmit shift register where it is shifted out serially on the TX output. If the FIFO is disabled location zero of the FIFO is used to store the data."]
    #[inline(always)]
    pub const fn mem_thr(&self) -> &MemThr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - The receiver section consists of the receiver holding register (RHR) and the receiver shift register. The RHR is actually a 64-byte FIFO. The receiver shift register receives serial data from RX input. The data is converted to parallel data and moved to the RHR. If the FIFO is disabled location zero of the FIFO is used to store the single data character. Note: If an overflow occurs the data in the RHR is not overwritten."]
    #[inline(always)]
    pub const fn mem_rhr(&self) -> &MemRhr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Divisor Latches Low Register"]
    #[inline(always)]
    pub const fn mem_dll(&self) -> &MemDll {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are seven types of interrupt in this mode: receiver error, RHR interrupt, THR interrupt, XOFF received and CTS*/RTS* change of state from low to high. Each interrupt can be enabled/disabled individually. There is also a sleep mode enable bit."]
    #[inline(always)]
    pub const fn mem_ier_uart(&self) -> &MemIerUart {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are 8 types of interrupt in these modes, received EOF, LSR interrupt, TX status, status FIFO interrupt, RX overrun, last byte in RX FIFO, THR interrupt and RHR interrupt and they can be enabled/disabled individually."]
    #[inline(always)]
    pub const fn mem_ier_irda(&self) -> &MemIerIrda {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are 6 types of interrupt in these modes, TX status, status FIFO interrupt, RX overrun, last byte in RX FIFO, THR interrupt and RHR interrupt and they can be enabled/disabled individually."]
    #[inline(always)]
    pub const fn mem_ier_cir(&self) -> &MemIerCir {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Divisor Latches High Register"]
    #[inline(always)]
    pub const fn mem_dlh(&self) -> &MemDlh {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner."]
    #[inline(always)]
    pub const fn mem_iir_uart(&self) -> &MemIirUart {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner."]
    #[inline(always)]
    pub const fn mem_iir_irda(&self) -> &MemIirIrda {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner."]
    #[inline(always)]
    pub const fn mem_iir_cir(&self) -> &MemIirCir {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Notes: Bits 4 and 5 can only be written to when EFR\\[4\\]
= 1 Bits 0 to 3 can be changed only when the baud clock is not running (DLL and DLH set to 0) See Table 31 for FCR\\[5:4\\]
setting restriction when SCR\\[6\\]=1 See Table 32 for FCR\\[7:6\\]
setting restriction when SCR\\[7\\]=1"]
    #[inline(always)]
    pub const fn mem_fcr(&self) -> &MemFcr {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Enhanced Feature Register"]
    #[inline(always)]
    pub const fn mem_efr(&self) -> &MemEfr {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - LCR\\[6:0\\]
define parameters of the transmission and reception. Note: As soon as LCR\\[6\\]
is set to 1, the TX line is forced to 0 and remains in this state as long as LCR\\[6\\]
= 1."]
    #[inline(always)]
    pub const fn mem_lcr(&self) -> &MemLcr {
        &self.mem_lcr
    }
    #[doc = "0x10 - XON1/ADDR1 Register"]
    #[inline(always)]
    pub const fn mem_xon1_addr1(&self) -> &MemXon1Addr1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - MCR\\[3:0\\]
controls the interface with the modem, data set or peripheral device that is emulating the modem."]
    #[inline(always)]
    pub const fn mem_mcr(&self) -> &MemMcr {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - XON2/ADDR2 Register"]
    #[inline(always)]
    pub const fn mem_xon2_addr2(&self) -> &MemXon2Addr2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - MEM_LSR_UART"]
    #[inline(always)]
    pub const fn mem_lsr_uart(&self) -> &MemLsrUart {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - MEM_LSR_IRDA"]
    #[inline(always)]
    pub const fn mem_lsr_irda(&self) -> &MemLsrIrda {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - MEM_LSR_CIR"]
    #[inline(always)]
    pub const fn mem_lsr_cir(&self) -> &MemLsrCir {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - XOFF1 Register"]
    #[inline(always)]
    pub const fn mem_xoff1(&self) -> &MemXoff1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Transmission Control Register"]
    #[inline(always)]
    pub const fn mem_tcr(&self) -> &MemTcr {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - This register provides information about the current state of the control lines from the modem, data set or peripheral device to the LH. It also indicates when a control input from the modem changes state."]
    #[inline(always)]
    pub const fn mem_msr(&self) -> &MemMsr {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - XOFF2 Register"]
    #[inline(always)]
    pub const fn mem_xoff2(&self) -> &MemXoff2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Trigger Level Register"]
    #[inline(always)]
    pub const fn mem_tlr(&self) -> &MemTlr {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - This read/write register does not control the module in anyway. It is intended as a scratchpad register to be used by the programmer to hold temporary data."]
    #[inline(always)]
    pub const fn mem_spr(&self) -> &MemSpr {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - The mode of operation can be programmed by writing to MDR1\\[2:0\\]
and therefore the MDR1 must be programmed on start-up after configuration of the configuration registers (DLL, DLH, LCR). The value of MDR1\\[2:0\\]
must not be changed again during normal operation. Note: If the module is disabled by setting the MODE_SELECT field to"]
    #[inline(always)]
    pub const fn mem_mdr1(&self) -> &MemMdr1 {
        &self.mem_mdr1
    }
    #[doc = "0x24 - IR-IrDA and IR-CIR modes only. MDR2\\[0\\]
describes the status of the interrupt in IIR\\[5\\]. The IRTX_UNDERRUN bit should be read after an IIR\\[5\\]
TX_STATUS_IT interrupt has occurred. The bits \\[2:1\\]
of this register sets the trigger level for the frame status FIFO (8 entries) and must be programmed before the mode is programmed in MDR1\\[2:0\\]. Note: The MDR2\\[6\\]
gives the flexibility to invert the RX pin inside the UART module to ensure that the protocol at the input of the transceiver module has the same polarity at module level. By default, the RX pin is inverted because most of transceiver invert the IR receive pin."]
    #[inline(always)]
    pub const fn mem_mdr2(&self) -> &MemMdr2 {
        &self.mem_mdr2
    }
    #[doc = "0x28 - IrDA modes only. The registers TXFLL and TXFLH hold the 13-bit transmit frame length (expressed in bytes). TXFLL holds the least significant bits and TXFLH holds the most significant bits. The frame length value is used if the frame length method of frame closing is used."]
    #[inline(always)]
    pub const fn mem_txfll(&self) -> &MemTxfll {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - IrDA modes only. Reading this register effectively reads frame status information from the status FIFO (this register doesn't physically exist). Reading this register will increment the status FIFO read pointer (SFREGL and SFREGH must be read first)."]
    #[inline(always)]
    pub const fn mem_sflsr(&self) -> &MemSflsr {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - IrDA modes only. The registers TXFLL and TXFLH hold the 13-bit transmit frame length (expressed in bytes). TXFLL holds the least significant bits and TXFLH holds the most significant bits. The frame length value is used if the frame length method of frame closing is used."]
    #[inline(always)]
    pub const fn mem_txflh(&self) -> &MemTxflh {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - IR-IrDA and IR-CIR modes only. This register is used to clear internal flags, which halt transmission/reception when an underrun/overrun error occurs. Reading this register resumes the halted operation. This register does not physically exist and reads always as 0x00."]
    #[inline(always)]
    pub const fn mem_resume(&self) -> &MemResume {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x30 - IrDA modes only. The frame lengths of received frames are written into the status FIFO. This information can be read by reading the SFREGL and SFREGH registers (i.e. these registers do not physically exist). The least significant bits are read from SFREGL and the most significant bits are read from SFREGH. Reading these registers does not alter the status FIFO read pointer. These registers should be read before the pointer is incremented by reading the SFLSR."]
    #[inline(always)]
    pub const fn mem_sfregl(&self) -> &MemSfregl {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - IrDA modes only. The registers RXFLL and RXFLH hold the 12-bit receive maximum frame length. RXFLL holds the least significant bits and RXFLH holds the most significant bits. If the intended maximum receive frame length is n bytes, then program RXFLL and RXFLH to be n + 3 in SIR or MIR modes and n + 6 in FIR mode (+3 and +6 are due to frame format with CRC and stop flag; there are two bytes associated with the FIR stop flag)."]
    #[inline(always)]
    pub const fn mem_rxfll(&self) -> &MemRxfll {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - IrDA modes only. The frame lengths of received frames are written into the status FIFO. This information can be read by reading the SFREGL and SFREGH registers (i.e. these registers do not physically exist). The least significant bits are read from SFREGL and the most significant bits are read from SFREGH. Reading these registers does not alter the status FIFO read pointer. These registers should be read before the pointer is incremented by reading the SFLSR."]
    #[inline(always)]
    pub const fn mem_sfregh(&self) -> &MemSfregh {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - IrDA modes only. The registers RXFLL and RXFLH hold the 12-bit receive maximum frame length. RXFLL holds the least significant bits and RXFLH holds the most significant bits. If the intended maximum receive frame length is n bytes, then program RXFLL and RXFLH to be n + 3 in SIR or MIR modes and n + 6 in FIR mode (+3 and +6 are due to frame format with CRC and stop flag; there are two bytes associated with the FIR stop flag)."]
    #[inline(always)]
    pub const fn mem_rxflh(&self) -> &MemRxflh {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x38 - UART Autobauding Status Register"]
    #[inline(always)]
    pub const fn mem_uasr(&self) -> &MemUasr {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - IrDA modes only. Note that BLR\\[6\\]
is used to select whether 0xC0 or 0xFF start patterns are to be used, when multiple start flags are required in SIR Mode. If only one start flag is required, this will always be 0xC0. If n start flags are required, then either (n-1) 0xC0 or (n-1) 0xFF flags are sent, followed by a single 0xC0 flag (immediately preceding the first data byte)."]
    #[inline(always)]
    pub const fn mem_blr(&self) -> &MemBlr {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3c - IR-IrDA and IR-CIR modes only."]
    #[inline(always)]
    pub const fn mem_acreg(&self) -> &MemAcreg {
        &self.mem_acreg
    }
    #[doc = "0x40 - Note: Bit 4 enables the wake-up interrupt, but this interrupt is not mapped into the IIR register. Therefore, when an interrupt occurs and there is no interrupt pending in the IIR register, the SSR\\[1\\]
bit must be checked. To clear the wake-up interrupt, bit SCR\\[4\\]
must be reset to 0."]
    #[inline(always)]
    pub const fn mem_scr(&self) -> &MemScr {
        &self.mem_scr
    }
    #[doc = "0x44 - Note: Bit 1 is reset only when SCR\\[4\\]
is reset to 0."]
    #[inline(always)]
    pub const fn mem_ssr(&self) -> &MemSsr {
        &self.mem_ssr
    }
    #[doc = "0x48 - IR-IrDA and IR-CIR modes only. In IR-IrDA SIR operation, this register specifies the number of BOF + xBOFs to transmit. Value set into this register must take into account the BOF character, therefore to only sent one BOF with no XBOF this register must be set to 1. To send one BOF with N XBOF this register must be set to N+1. Furthermore, the value 0 will send 1 BOF plus 255 XBOF. In IR-IrDA MIR mode, this register specifies the number of additional start flags (MIR protocol mandates a minimum of 2 start flags). In IR-CIR mode, this register specifies the number of consecutive zeros to be received before generating the RX_STOP interrupt (IIR\\[2\\]). All the received zeros are stored in the RX FIFO. When the register is set to 0, this feature is de-activated and always in reception state which can be disabled by setting the ACREG\\[5\\]
to"]
    #[inline(always)]
    pub const fn mem_eblr(&self) -> &MemEblr {
        &self.mem_eblr
    }
    #[doc = "0x50 - The reset value is fixed by hardware and corresponds to the RTL revision of this module. A reset has no effect on the value returned Notes: UART / IRDA SIR only module is revision 1.x (WMU_012_1 specification). UART / IRDA with SIR, MIR and FIR support is revision 2.x (WMU_012_2 specification). UART / IRDA with SIR, MIR and FIR / CIR support is revision 3.x (this specification). For example: MVR = 0x30 => Version 3.0 MVR = 0x38 => Version 3.8"]
    #[inline(always)]
    pub const fn mem_mvr(&self) -> &MemMvr {
        &self.mem_mvr
    }
    #[doc = "0x54 - The auto idle bit controls a power saving technique to reduce the logic power consumption of the OCP interface. That is to say when the feature is enabled, the clock will be gated off until an OCP command for this device has been detected. When the software reset bit is set high it causes a full device reset."]
    #[inline(always)]
    pub const fn mem_sysc(&self) -> &MemSysc {
        &self.mem_sysc
    }
    #[doc = "0x58 - MEM_SYSS"]
    #[inline(always)]
    pub const fn mem_syss(&self) -> &MemSyss {
        &self.mem_syss
    }
    #[doc = "0x5c - The UART wakeup enable register is used to mask and unmask a UART event that would subsequently notify the system. The events are any activity in the logic that could cause an interrupt and/ or an activity that would require the system to wakeup. It should be noted that even if the wakeup is disabled for certain events, if these events are also an interrupt to the UART, then the UART will still register the interrupt as such."]
    #[inline(always)]
    pub const fn mem_wer(&self) -> &MemWer {
        &self.mem_wer
    }
    #[doc = "0x60 - Since the Consumer IR works at modulation rates of 30 56.8 KHz, the 48 MHz clock must be pre scaled before the clock can drive the IR logic. This register sets the divisor rate to give a range to accommodate the remote control requirements in BAUD multiples of 12x. The value of the CFPS at reset is 0105 decimal which equates to a 38.1 KHz output from starting conditions. The 48 MHz carrier is prescaled by the CFPS which is then divided by the 12x BAUD multiple."]
    #[inline(always)]
    pub const fn mem_cfps(&self) -> &MemCfps {
        &self.mem_cfps
    }
    #[doc = "0x64 - Level of the RX FIFO"]
    #[inline(always)]
    pub const fn mem_rxfifo_lvl(&self) -> &MemRxfifoLvl {
        &self.mem_rxfifo_lvl
    }
    #[doc = "0x68 - Level of the TX FIFO"]
    #[inline(always)]
    pub const fn mem_txfifo_lvl(&self) -> &MemTxfifoLvl {
        &self.mem_txfifo_lvl
    }
    #[doc = "0x6c - Enables RX/TX FIFOs empty corresponding interrupts."]
    #[inline(always)]
    pub const fn mem_ier2(&self) -> &MemIer2 {
        &self.mem_ier2
    }
    #[doc = "0x70 - Status of RX/TX FIFOs empty corresponding interrupts."]
    #[inline(always)]
    pub const fn mem_isr2(&self) -> &MemIsr2 {
        &self.mem_isr2
    }
    #[doc = "0x74 - Sample per bit value selector"]
    #[inline(always)]
    pub const fn mem_freq_sel(&self) -> &MemFreqSel {
        &self.mem_freq_sel
    }
    #[doc = "0x78 - Unused"]
    #[inline(always)]
    pub const fn mem_abaud_1st_char(&self) -> &MemAbaud1stChar {
        &self.mem_abaud_1st_char
    }
    #[doc = "0x7c - Unused"]
    #[inline(always)]
    pub const fn mem_baud_2nd_char(&self) -> &MemBaud2ndChar {
        &self.mem_baud_2nd_char
    }
    #[doc = "0x80 - Mode definition register 3."]
    #[inline(always)]
    pub const fn mem_mdr3(&self) -> &MemMdr3 {
        &self.mem_mdr3
    }
    #[doc = "0x84 - Use to manually set the TX DMA threshold level. MDR3\\[2\\]
SET_TX_DMA_THRESHOLD must be one and must be value + tx_trigger_level &lt;= 64 (TX FIFO size). If not, 64-tx_trigger_level will be used w/o modifying the value of this register."]
    #[inline(always)]
    pub const fn mem_tx_dma_threshold(&self) -> &MemTxDmaThreshold {
        &self.mem_tx_dma_threshold
    }
    #[doc = "0x88 - Mode definition register 4"]
    #[inline(always)]
    pub const fn mem_mdr4(&self) -> &MemMdr4 {
        &self.mem_mdr4
    }
    #[doc = "0x8c - Enhanced Features Register 2"]
    #[inline(always)]
    pub const fn mem_efr2(&self) -> &MemEfr2 {
        &self.mem_efr2
    }
    #[doc = "0x90 - Enhanced Control register"]
    #[inline(always)]
    pub const fn mem_ecr(&self) -> &MemEcr {
        &self.mem_ecr
    }
    #[doc = "0x94 - Timeguard"]
    #[inline(always)]
    pub const fn mem_timeguard(&self) -> &MemTimeguard {
        &self.mem_timeguard
    }
    #[doc = "0x98 - Timeout lower byte"]
    #[inline(always)]
    pub const fn mem_timeoutl(&self) -> &MemTimeoutl {
        &self.mem_timeoutl
    }
    #[doc = "0x9c - Timeout higher byte"]
    #[inline(always)]
    pub const fn mem_timeouth(&self) -> &MemTimeouth {
        &self.mem_timeouth
    }
    #[doc = "0xa0 - Smartcard (ISO7816) mode Control Register"]
    #[inline(always)]
    pub const fn mem_sccr(&self) -> &MemSccr {
        &self.mem_sccr
    }
    #[doc = "0xa4 - Extended Transmit Holding Register"]
    #[inline(always)]
    pub const fn mem_ethr(&self) -> &MemEthr {
        unsafe { &*(self as *const Self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa4 - Extended Receive Holding Register"]
    #[inline(always)]
    pub const fn mem_erhr(&self) -> &MemErhr {
        unsafe { &*(self as *const Self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa8 - Multidrop Address Register"]
    #[inline(always)]
    pub const fn mem_mar(&self) -> &MemMar {
        &self.mem_mar
    }
    #[doc = "0xac - Multidrop Mask Register"]
    #[inline(always)]
    pub const fn mem_mmr(&self) -> &MemMmr {
        &self.mem_mmr
    }
    #[doc = "0xb0 - Multidrop Broadcast Address Register"]
    #[inline(always)]
    pub const fn mem_mbr(&self) -> &MemMbr {
        &self.mem_mbr
    }
}
#[doc = "MEM_DLL (rw) register accessor: Divisor Latches Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_dll`]
module"]
#[doc(alias = "MEM_DLL")]
pub type MemDll = crate::Reg<mem_dll::MemDllSpec>;
#[doc = "Divisor Latches Low Register"]
pub mod mem_dll;
#[doc = "MEM_RHR (rw) register accessor: The receiver section consists of the receiver holding register (RHR) and the receiver shift register. The RHR is actually a 64-byte FIFO. The receiver shift register receives serial data from RX input. The data is converted to parallel data and moved to the RHR. If the FIFO is disabled location zero of the FIFO is used to store the single data character. Note: If an overflow occurs the data in the RHR is not overwritten.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_rhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rhr`]
module"]
#[doc(alias = "MEM_RHR")]
pub type MemRhr = crate::Reg<mem_rhr::MemRhrSpec>;
#[doc = "The receiver section consists of the receiver holding register (RHR) and the receiver shift register. The RHR is actually a 64-byte FIFO. The receiver shift register receives serial data from RX input. The data is converted to parallel data and moved to the RHR. If the FIFO is disabled location zero of the FIFO is used to store the single data character. Note: If an overflow occurs the data in the RHR is not overwritten."]
pub mod mem_rhr;
#[doc = "MEM_THR (rw) register accessor: The transmitter section consists of the transmit holding register (THR) and the transmit shift register. The transmit holding register is actually a 64-byte FIFO. The LH writes data to the THR. The data is placed into the transmit shift register where it is shifted out serially on the TX output. If the FIFO is disabled location zero of the FIFO is used to store the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_thr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_thr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_thr`]
module"]
#[doc(alias = "MEM_THR")]
pub type MemThr = crate::Reg<mem_thr::MemThrSpec>;
#[doc = "The transmitter section consists of the transmit holding register (THR) and the transmit shift register. The transmit holding register is actually a 64-byte FIFO. The LH writes data to the THR. The data is placed into the transmit shift register where it is shifted out serially on the TX output. If the FIFO is disabled location zero of the FIFO is used to store the data."]
pub mod mem_thr;
#[doc = "MEM_DLH (rw) register accessor: Divisor Latches High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_dlh`]
module"]
#[doc(alias = "MEM_DLH")]
pub type MemDlh = crate::Reg<mem_dlh::MemDlhSpec>;
#[doc = "Divisor Latches High Register"]
pub mod mem_dlh;
#[doc = "MEM_IER_CIR (rw) register accessor: The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are 6 types of interrupt in these modes, TX status, status FIFO interrupt, RX overrun, last byte in RX FIFO, THR interrupt and RHR interrupt and they can be enabled/disabled individually.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ier_cir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ier_cir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ier_cir`]
module"]
#[doc(alias = "MEM_IER_CIR")]
pub type MemIerCir = crate::Reg<mem_ier_cir::MemIerCirSpec>;
#[doc = "The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are 6 types of interrupt in these modes, TX status, status FIFO interrupt, RX overrun, last byte in RX FIFO, THR interrupt and RHR interrupt and they can be enabled/disabled individually."]
pub mod mem_ier_cir;
#[doc = "MEM_IER_IRDA (rw) register accessor: The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are 8 types of interrupt in these modes, received EOF, LSR interrupt, TX status, status FIFO interrupt, RX overrun, last byte in RX FIFO, THR interrupt and RHR interrupt and they can be enabled/disabled individually.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ier_irda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ier_irda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ier_irda`]
module"]
#[doc(alias = "MEM_IER_IRDA")]
pub type MemIerIrda = crate::Reg<mem_ier_irda::MemIerIrdaSpec>;
#[doc = "The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are 8 types of interrupt in these modes, received EOF, LSR interrupt, TX status, status FIFO interrupt, RX overrun, last byte in RX FIFO, THR interrupt and RHR interrupt and they can be enabled/disabled individually."]
pub mod mem_ier_irda;
#[doc = "MEM_IER_UART (rw) register accessor: The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are seven types of interrupt in this mode: receiver error, RHR interrupt, THR interrupt, XOFF received and CTS*/RTS* change of state from low to high. Each interrupt can be enabled/disabled individually. There is also a sleep mode enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ier_uart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ier_uart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ier_uart`]
module"]
#[doc(alias = "MEM_IER_UART")]
pub type MemIerUart = crate::Reg<mem_ier_uart::MemIerUartSpec>;
#[doc = "The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are seven types of interrupt in this mode: receiver error, RHR interrupt, THR interrupt, XOFF received and CTS*/RTS* change of state from low to high. Each interrupt can be enabled/disabled individually. There is also a sleep mode enable bit."]
pub mod mem_ier_uart;
#[doc = "MEM_EFR (rw) register accessor: Enhanced Feature Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_efr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_efr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_efr`]
module"]
#[doc(alias = "MEM_EFR")]
pub type MemEfr = crate::Reg<mem_efr::MemEfrSpec>;
#[doc = "Enhanced Feature Register"]
pub mod mem_efr;
#[doc = "MEM_FCR (rw) register accessor: Notes: Bits 4 and 5 can only be written to when EFR\\[4\\]
= 1 Bits 0 to 3 can be changed only when the baud clock is not running (DLL and DLH set to 0) See Table 31 for FCR\\[5:4\\]
setting restriction when SCR\\[6\\]=1 See Table 32 for FCR\\[7:6\\]
setting restriction when SCR\\[7\\]=1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_fcr`]
module"]
#[doc(alias = "MEM_FCR")]
pub type MemFcr = crate::Reg<mem_fcr::MemFcrSpec>;
#[doc = "Notes: Bits 4 and 5 can only be written to when EFR\\[4\\]
= 1 Bits 0 to 3 can be changed only when the baud clock is not running (DLL and DLH set to 0) See Table 31 for FCR\\[5:4\\]
setting restriction when SCR\\[6\\]=1 See Table 32 for FCR\\[7:6\\]
setting restriction when SCR\\[7\\]=1"]
pub mod mem_fcr;
#[doc = "MEM_IIR_CIR (rw) register accessor: The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_iir_cir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_iir_cir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_iir_cir`]
module"]
#[doc(alias = "MEM_IIR_CIR")]
pub type MemIirCir = crate::Reg<mem_iir_cir::MemIirCirSpec>;
#[doc = "The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner."]
pub mod mem_iir_cir;
#[doc = "MEM_IIR_IRDA (rw) register accessor: The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_iir_irda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_iir_irda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_iir_irda`]
module"]
#[doc(alias = "MEM_IIR_IRDA")]
pub type MemIirIrda = crate::Reg<mem_iir_irda::MemIirIrdaSpec>;
#[doc = "The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner."]
pub mod mem_iir_irda;
#[doc = "MEM_IIR_UART (rw) register accessor: The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_iir_uart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_iir_uart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_iir_uart`]
module"]
#[doc(alias = "MEM_IIR_UART")]
pub type MemIirUart = crate::Reg<mem_iir_uart::MemIirUartSpec>;
#[doc = "The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner."]
pub mod mem_iir_uart;
#[doc = "MEM_LCR (rw) register accessor: LCR\\[6:0\\]
define parameters of the transmission and reception. Note: As soon as LCR\\[6\\]
is set to 1, the TX line is forced to 0 and remains in this state as long as LCR\\[6\\]
= 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_lcr`]
module"]
#[doc(alias = "MEM_LCR")]
pub type MemLcr = crate::Reg<mem_lcr::MemLcrSpec>;
#[doc = "LCR\\[6:0\\]
define parameters of the transmission and reception. Note: As soon as LCR\\[6\\]
is set to 1, the TX line is forced to 0 and remains in this state as long as LCR\\[6\\]
= 1."]
pub mod mem_lcr;
#[doc = "MEM_MCR (rw) register accessor: MCR\\[3:0\\]
controls the interface with the modem, data set or peripheral device that is emulating the modem.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mcr`]
module"]
#[doc(alias = "MEM_MCR")]
pub type MemMcr = crate::Reg<mem_mcr::MemMcrSpec>;
#[doc = "MCR\\[3:0\\]
controls the interface with the modem, data set or peripheral device that is emulating the modem."]
pub mod mem_mcr;
#[doc = "MEM_XON1_ADDR1 (rw) register accessor: XON1/ADDR1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_xon1_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_xon1_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xon1_addr1`]
module"]
#[doc(alias = "MEM_XON1_ADDR1")]
pub type MemXon1Addr1 = crate::Reg<mem_xon1_addr1::MemXon1Addr1Spec>;
#[doc = "XON1/ADDR1 Register"]
pub mod mem_xon1_addr1;
#[doc = "MEM_LSR_CIR (rw) register accessor: MEM_LSR_CIR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_lsr_cir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_lsr_cir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_lsr_cir`]
module"]
#[doc(alias = "MEM_LSR_CIR")]
pub type MemLsrCir = crate::Reg<mem_lsr_cir::MemLsrCirSpec>;
#[doc = "MEM_LSR_CIR"]
pub mod mem_lsr_cir;
#[doc = "MEM_LSR_IRDA (rw) register accessor: MEM_LSR_IRDA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_lsr_irda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_lsr_irda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_lsr_irda`]
module"]
#[doc(alias = "MEM_LSR_IRDA")]
pub type MemLsrIrda = crate::Reg<mem_lsr_irda::MemLsrIrdaSpec>;
#[doc = "MEM_LSR_IRDA"]
pub mod mem_lsr_irda;
#[doc = "MEM_LSR_UART (rw) register accessor: MEM_LSR_UART\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_lsr_uart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_lsr_uart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_lsr_uart`]
module"]
#[doc(alias = "MEM_LSR_UART")]
pub type MemLsrUart = crate::Reg<mem_lsr_uart::MemLsrUartSpec>;
#[doc = "MEM_LSR_UART"]
pub mod mem_lsr_uart;
#[doc = "MEM_XON2_ADDR2 (rw) register accessor: XON2/ADDR2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_xon2_addr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_xon2_addr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xon2_addr2`]
module"]
#[doc(alias = "MEM_XON2_ADDR2")]
pub type MemXon2Addr2 = crate::Reg<mem_xon2_addr2::MemXon2Addr2Spec>;
#[doc = "XON2/ADDR2 Register"]
pub mod mem_xon2_addr2;
#[doc = "MEM_MSR (rw) register accessor: This register provides information about the current state of the control lines from the modem, data set or peripheral device to the LH. It also indicates when a control input from the modem changes state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_msr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_msr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_msr`]
module"]
#[doc(alias = "MEM_MSR")]
pub type MemMsr = crate::Reg<mem_msr::MemMsrSpec>;
#[doc = "This register provides information about the current state of the control lines from the modem, data set or peripheral device to the LH. It also indicates when a control input from the modem changes state."]
pub mod mem_msr;
#[doc = "MEM_TCR (rw) register accessor: Transmission Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_tcr`]
module"]
#[doc(alias = "MEM_TCR")]
pub type MemTcr = crate::Reg<mem_tcr::MemTcrSpec>;
#[doc = "Transmission Control Register"]
pub mod mem_tcr;
#[doc = "MEM_XOFF1 (rw) register accessor: XOFF1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_xoff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_xoff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xoff1`]
module"]
#[doc(alias = "MEM_XOFF1")]
pub type MemXoff1 = crate::Reg<mem_xoff1::MemXoff1Spec>;
#[doc = "XOFF1 Register"]
pub mod mem_xoff1;
#[doc = "MEM_SPR (rw) register accessor: This read/write register does not control the module in anyway. It is intended as a scratchpad register to be used by the programmer to hold temporary data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_spr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_spr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_spr`]
module"]
#[doc(alias = "MEM_SPR")]
pub type MemSpr = crate::Reg<mem_spr::MemSprSpec>;
#[doc = "This read/write register does not control the module in anyway. It is intended as a scratchpad register to be used by the programmer to hold temporary data."]
pub mod mem_spr;
#[doc = "MEM_TLR (rw) register accessor: Trigger Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_tlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_tlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_tlr`]
module"]
#[doc(alias = "MEM_TLR")]
pub type MemTlr = crate::Reg<mem_tlr::MemTlrSpec>;
#[doc = "Trigger Level Register"]
pub mod mem_tlr;
#[doc = "MEM_XOFF2 (rw) register accessor: XOFF2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_xoff2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_xoff2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xoff2`]
module"]
#[doc(alias = "MEM_XOFF2")]
pub type MemXoff2 = crate::Reg<mem_xoff2::MemXoff2Spec>;
#[doc = "XOFF2 Register"]
pub mod mem_xoff2;
#[doc = "MEM_MDR1 (rw) register accessor: The mode of operation can be programmed by writing to MDR1\\[2:0\\]
and therefore the MDR1 must be programmed on start-up after configuration of the configuration registers (DLL, DLH, LCR). The value of MDR1\\[2:0\\]
must not be changed again during normal operation. Note: If the module is disabled by setting the MODE_SELECT field to\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mdr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mdr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mdr1`]
module"]
#[doc(alias = "MEM_MDR1")]
pub type MemMdr1 = crate::Reg<mem_mdr1::MemMdr1Spec>;
#[doc = "The mode of operation can be programmed by writing to MDR1\\[2:0\\]
and therefore the MDR1 must be programmed on start-up after configuration of the configuration registers (DLL, DLH, LCR). The value of MDR1\\[2:0\\]
must not be changed again during normal operation. Note: If the module is disabled by setting the MODE_SELECT field to"]
pub mod mem_mdr1;
#[doc = "MEM_MDR2 (rw) register accessor: IR-IrDA and IR-CIR modes only. MDR2\\[0\\]
describes the status of the interrupt in IIR\\[5\\]. The IRTX_UNDERRUN bit should be read after an IIR\\[5\\]
TX_STATUS_IT interrupt has occurred. The bits \\[2:1\\]
of this register sets the trigger level for the frame status FIFO (8 entries) and must be programmed before the mode is programmed in MDR1\\[2:0\\]. Note: The MDR2\\[6\\]
gives the flexibility to invert the RX pin inside the UART module to ensure that the protocol at the input of the transceiver module has the same polarity at module level. By default, the RX pin is inverted because most of transceiver invert the IR receive pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mdr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mdr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mdr2`]
module"]
#[doc(alias = "MEM_MDR2")]
pub type MemMdr2 = crate::Reg<mem_mdr2::MemMdr2Spec>;
#[doc = "IR-IrDA and IR-CIR modes only. MDR2\\[0\\]
describes the status of the interrupt in IIR\\[5\\]. The IRTX_UNDERRUN bit should be read after an IIR\\[5\\]
TX_STATUS_IT interrupt has occurred. The bits \\[2:1\\]
of this register sets the trigger level for the frame status FIFO (8 entries) and must be programmed before the mode is programmed in MDR1\\[2:0\\]. Note: The MDR2\\[6\\]
gives the flexibility to invert the RX pin inside the UART module to ensure that the protocol at the input of the transceiver module has the same polarity at module level. By default, the RX pin is inverted because most of transceiver invert the IR receive pin."]
pub mod mem_mdr2;
#[doc = "MEM_SFLSR (rw) register accessor: IrDA modes only. Reading this register effectively reads frame status information from the status FIFO (this register doesn't physically exist). Reading this register will increment the status FIFO read pointer (SFREGL and SFREGH must be read first).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sflsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sflsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_sflsr`]
module"]
#[doc(alias = "MEM_SFLSR")]
pub type MemSflsr = crate::Reg<mem_sflsr::MemSflsrSpec>;
#[doc = "IrDA modes only. Reading this register effectively reads frame status information from the status FIFO (this register doesn't physically exist). Reading this register will increment the status FIFO read pointer (SFREGL and SFREGH must be read first)."]
pub mod mem_sflsr;
#[doc = "MEM_TXFLL (rw) register accessor: IrDA modes only. The registers TXFLL and TXFLH hold the 13-bit transmit frame length (expressed in bytes). TXFLL holds the least significant bits and TXFLH holds the most significant bits. The frame length value is used if the frame length method of frame closing is used.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_txfll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_txfll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_txfll`]
module"]
#[doc(alias = "MEM_TXFLL")]
pub type MemTxfll = crate::Reg<mem_txfll::MemTxfllSpec>;
#[doc = "IrDA modes only. The registers TXFLL and TXFLH hold the 13-bit transmit frame length (expressed in bytes). TXFLL holds the least significant bits and TXFLH holds the most significant bits. The frame length value is used if the frame length method of frame closing is used."]
pub mod mem_txfll;
#[doc = "MEM_RESUME (rw) register accessor: IR-IrDA and IR-CIR modes only. This register is used to clear internal flags, which halt transmission/reception when an underrun/overrun error occurs. Reading this register resumes the halted operation. This register does not physically exist and reads always as 0x00.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_resume::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_resume::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_resume`]
module"]
#[doc(alias = "MEM_RESUME")]
pub type MemResume = crate::Reg<mem_resume::MemResumeSpec>;
#[doc = "IR-IrDA and IR-CIR modes only. This register is used to clear internal flags, which halt transmission/reception when an underrun/overrun error occurs. Reading this register resumes the halted operation. This register does not physically exist and reads always as 0x00."]
pub mod mem_resume;
#[doc = "MEM_TXFLH (rw) register accessor: IrDA modes only. The registers TXFLL and TXFLH hold the 13-bit transmit frame length (expressed in bytes). TXFLL holds the least significant bits and TXFLH holds the most significant bits. The frame length value is used if the frame length method of frame closing is used.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_txflh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_txflh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_txflh`]
module"]
#[doc(alias = "MEM_TXFLH")]
pub type MemTxflh = crate::Reg<mem_txflh::MemTxflhSpec>;
#[doc = "IrDA modes only. The registers TXFLL and TXFLH hold the 13-bit transmit frame length (expressed in bytes). TXFLL holds the least significant bits and TXFLH holds the most significant bits. The frame length value is used if the frame length method of frame closing is used."]
pub mod mem_txflh;
#[doc = "MEM_RXFLL (rw) register accessor: IrDA modes only. The registers RXFLL and RXFLH hold the 12-bit receive maximum frame length. RXFLL holds the least significant bits and RXFLH holds the most significant bits. If the intended maximum receive frame length is n bytes, then program RXFLL and RXFLH to be n + 3 in SIR or MIR modes and n + 6 in FIR mode (+3 and +6 are due to frame format with CRC and stop flag; there are two bytes associated with the FIR stop flag).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rxfll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_rxfll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rxfll`]
module"]
#[doc(alias = "MEM_RXFLL")]
pub type MemRxfll = crate::Reg<mem_rxfll::MemRxfllSpec>;
#[doc = "IrDA modes only. The registers RXFLL and RXFLH hold the 12-bit receive maximum frame length. RXFLL holds the least significant bits and RXFLH holds the most significant bits. If the intended maximum receive frame length is n bytes, then program RXFLL and RXFLH to be n + 3 in SIR or MIR modes and n + 6 in FIR mode (+3 and +6 are due to frame format with CRC and stop flag; there are two bytes associated with the FIR stop flag)."]
pub mod mem_rxfll;
#[doc = "MEM_SFREGL (rw) register accessor: IrDA modes only. The frame lengths of received frames are written into the status FIFO. This information can be read by reading the SFREGL and SFREGH registers (i.e. these registers do not physically exist). The least significant bits are read from SFREGL and the most significant bits are read from SFREGH. Reading these registers does not alter the status FIFO read pointer. These registers should be read before the pointer is incremented by reading the SFLSR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sfregl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sfregl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_sfregl`]
module"]
#[doc(alias = "MEM_SFREGL")]
pub type MemSfregl = crate::Reg<mem_sfregl::MemSfreglSpec>;
#[doc = "IrDA modes only. The frame lengths of received frames are written into the status FIFO. This information can be read by reading the SFREGL and SFREGH registers (i.e. these registers do not physically exist). The least significant bits are read from SFREGL and the most significant bits are read from SFREGH. Reading these registers does not alter the status FIFO read pointer. These registers should be read before the pointer is incremented by reading the SFLSR."]
pub mod mem_sfregl;
#[doc = "MEM_RXFLH (rw) register accessor: IrDA modes only. The registers RXFLL and RXFLH hold the 12-bit receive maximum frame length. RXFLL holds the least significant bits and RXFLH holds the most significant bits. If the intended maximum receive frame length is n bytes, then program RXFLL and RXFLH to be n + 3 in SIR or MIR modes and n + 6 in FIR mode (+3 and +6 are due to frame format with CRC and stop flag; there are two bytes associated with the FIR stop flag).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rxflh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_rxflh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rxflh`]
module"]
#[doc(alias = "MEM_RXFLH")]
pub type MemRxflh = crate::Reg<mem_rxflh::MemRxflhSpec>;
#[doc = "IrDA modes only. The registers RXFLL and RXFLH hold the 12-bit receive maximum frame length. RXFLL holds the least significant bits and RXFLH holds the most significant bits. If the intended maximum receive frame length is n bytes, then program RXFLL and RXFLH to be n + 3 in SIR or MIR modes and n + 6 in FIR mode (+3 and +6 are due to frame format with CRC and stop flag; there are two bytes associated with the FIR stop flag)."]
pub mod mem_rxflh;
#[doc = "MEM_SFREGH (rw) register accessor: IrDA modes only. The frame lengths of received frames are written into the status FIFO. This information can be read by reading the SFREGL and SFREGH registers (i.e. these registers do not physically exist). The least significant bits are read from SFREGL and the most significant bits are read from SFREGH. Reading these registers does not alter the status FIFO read pointer. These registers should be read before the pointer is incremented by reading the SFLSR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sfregh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sfregh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_sfregh`]
module"]
#[doc(alias = "MEM_SFREGH")]
pub type MemSfregh = crate::Reg<mem_sfregh::MemSfreghSpec>;
#[doc = "IrDA modes only. The frame lengths of received frames are written into the status FIFO. This information can be read by reading the SFREGL and SFREGH registers (i.e. these registers do not physically exist). The least significant bits are read from SFREGL and the most significant bits are read from SFREGH. Reading these registers does not alter the status FIFO read pointer. These registers should be read before the pointer is incremented by reading the SFLSR."]
pub mod mem_sfregh;
#[doc = "MEM_BLR (rw) register accessor: IrDA modes only. Note that BLR\\[6\\]
is used to select whether 0xC0 or 0xFF start patterns are to be used, when multiple start flags are required in SIR Mode. If only one start flag is required, this will always be 0xC0. If n start flags are required, then either (n-1) 0xC0 or (n-1) 0xFF flags are sent, followed by a single 0xC0 flag (immediately preceding the first data byte).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_blr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_blr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_blr`]
module"]
#[doc(alias = "MEM_BLR")]
pub type MemBlr = crate::Reg<mem_blr::MemBlrSpec>;
#[doc = "IrDA modes only. Note that BLR\\[6\\]
is used to select whether 0xC0 or 0xFF start patterns are to be used, when multiple start flags are required in SIR Mode. If only one start flag is required, this will always be 0xC0. If n start flags are required, then either (n-1) 0xC0 or (n-1) 0xFF flags are sent, followed by a single 0xC0 flag (immediately preceding the first data byte)."]
pub mod mem_blr;
#[doc = "MEM_UASR (rw) register accessor: UART Autobauding Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_uasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_uasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_uasr`]
module"]
#[doc(alias = "MEM_UASR")]
pub type MemUasr = crate::Reg<mem_uasr::MemUasrSpec>;
#[doc = "UART Autobauding Status Register"]
pub mod mem_uasr;
#[doc = "MEM_ACREG (rw) register accessor: IR-IrDA and IR-CIR modes only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_acreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_acreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_acreg`]
module"]
#[doc(alias = "MEM_ACREG")]
pub type MemAcreg = crate::Reg<mem_acreg::MemAcregSpec>;
#[doc = "IR-IrDA and IR-CIR modes only."]
pub mod mem_acreg;
#[doc = "MEM_SCR (rw) register accessor: Note: Bit 4 enables the wake-up interrupt, but this interrupt is not mapped into the IIR register. Therefore, when an interrupt occurs and there is no interrupt pending in the IIR register, the SSR\\[1\\]
bit must be checked. To clear the wake-up interrupt, bit SCR\\[4\\]
must be reset to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_scr`]
module"]
#[doc(alias = "MEM_SCR")]
pub type MemScr = crate::Reg<mem_scr::MemScrSpec>;
#[doc = "Note: Bit 4 enables the wake-up interrupt, but this interrupt is not mapped into the IIR register. Therefore, when an interrupt occurs and there is no interrupt pending in the IIR register, the SSR\\[1\\]
bit must be checked. To clear the wake-up interrupt, bit SCR\\[4\\]
must be reset to 0."]
pub mod mem_scr;
#[doc = "MEM_SSR (rw) register accessor: Note: Bit 1 is reset only when SCR\\[4\\]
is reset to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ssr`]
module"]
#[doc(alias = "MEM_SSR")]
pub type MemSsr = crate::Reg<mem_ssr::MemSsrSpec>;
#[doc = "Note: Bit 1 is reset only when SCR\\[4\\]
is reset to 0."]
pub mod mem_ssr;
#[doc = "MEM_EBLR (rw) register accessor: IR-IrDA and IR-CIR modes only. In IR-IrDA SIR operation, this register specifies the number of BOF + xBOFs to transmit. Value set into this register must take into account the BOF character, therefore to only sent one BOF with no XBOF this register must be set to 1. To send one BOF with N XBOF this register must be set to N+1. Furthermore, the value 0 will send 1 BOF plus 255 XBOF. In IR-IrDA MIR mode, this register specifies the number of additional start flags (MIR protocol mandates a minimum of 2 start flags). In IR-CIR mode, this register specifies the number of consecutive zeros to be received before generating the RX_STOP interrupt (IIR\\[2\\]). All the received zeros are stored in the RX FIFO. When the register is set to 0, this feature is de-activated and always in reception state which can be disabled by setting the ACREG\\[5\\]
to\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_eblr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_eblr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_eblr`]
module"]
#[doc(alias = "MEM_EBLR")]
pub type MemEblr = crate::Reg<mem_eblr::MemEblrSpec>;
#[doc = "IR-IrDA and IR-CIR modes only. In IR-IrDA SIR operation, this register specifies the number of BOF + xBOFs to transmit. Value set into this register must take into account the BOF character, therefore to only sent one BOF with no XBOF this register must be set to 1. To send one BOF with N XBOF this register must be set to N+1. Furthermore, the value 0 will send 1 BOF plus 255 XBOF. In IR-IrDA MIR mode, this register specifies the number of additional start flags (MIR protocol mandates a minimum of 2 start flags). In IR-CIR mode, this register specifies the number of consecutive zeros to be received before generating the RX_STOP interrupt (IIR\\[2\\]). All the received zeros are stored in the RX FIFO. When the register is set to 0, this feature is de-activated and always in reception state which can be disabled by setting the ACREG\\[5\\]
to"]
pub mod mem_eblr;
#[doc = "MEM_MVR (rw) register accessor: The reset value is fixed by hardware and corresponds to the RTL revision of this module. A reset has no effect on the value returned Notes: UART / IRDA SIR only module is revision 1.x (WMU_012_1 specification). UART / IRDA with SIR, MIR and FIR support is revision 2.x (WMU_012_2 specification). UART / IRDA with SIR, MIR and FIR / CIR support is revision 3.x (this specification). For example: MVR = 0x30 => Version 3.0 MVR = 0x38 => Version 3.8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mvr`]
module"]
#[doc(alias = "MEM_MVR")]
pub type MemMvr = crate::Reg<mem_mvr::MemMvrSpec>;
#[doc = "The reset value is fixed by hardware and corresponds to the RTL revision of this module. A reset has no effect on the value returned Notes: UART / IRDA SIR only module is revision 1.x (WMU_012_1 specification). UART / IRDA with SIR, MIR and FIR support is revision 2.x (WMU_012_2 specification). UART / IRDA with SIR, MIR and FIR / CIR support is revision 3.x (this specification). For example: MVR = 0x30 => Version 3.0 MVR = 0x38 => Version 3.8"]
pub mod mem_mvr;
#[doc = "MEM_SYSC (rw) register accessor: The auto idle bit controls a power saving technique to reduce the logic power consumption of the OCP interface. That is to say when the feature is enabled, the clock will be gated off until an OCP command for this device has been detected. When the software reset bit is set high it causes a full device reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sysc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sysc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_sysc`]
module"]
#[doc(alias = "MEM_SYSC")]
pub type MemSysc = crate::Reg<mem_sysc::MemSyscSpec>;
#[doc = "The auto idle bit controls a power saving technique to reduce the logic power consumption of the OCP interface. That is to say when the feature is enabled, the clock will be gated off until an OCP command for this device has been detected. When the software reset bit is set high it causes a full device reset."]
pub mod mem_sysc;
#[doc = "MEM_SYSS (rw) register accessor: MEM_SYSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_syss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_syss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_syss`]
module"]
#[doc(alias = "MEM_SYSS")]
pub type MemSyss = crate::Reg<mem_syss::MemSyssSpec>;
#[doc = "MEM_SYSS"]
pub mod mem_syss;
#[doc = "MEM_WER (rw) register accessor: The UART wakeup enable register is used to mask and unmask a UART event that would subsequently notify the system. The events are any activity in the logic that could cause an interrupt and/ or an activity that would require the system to wakeup. It should be noted that even if the wakeup is disabled for certain events, if these events are also an interrupt to the UART, then the UART will still register the interrupt as such.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_wer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_wer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_wer`]
module"]
#[doc(alias = "MEM_WER")]
pub type MemWer = crate::Reg<mem_wer::MemWerSpec>;
#[doc = "The UART wakeup enable register is used to mask and unmask a UART event that would subsequently notify the system. The events are any activity in the logic that could cause an interrupt and/ or an activity that would require the system to wakeup. It should be noted that even if the wakeup is disabled for certain events, if these events are also an interrupt to the UART, then the UART will still register the interrupt as such."]
pub mod mem_wer;
#[doc = "MEM_CFPS (rw) register accessor: Since the Consumer IR works at modulation rates of 30 56.8 KHz, the 48 MHz clock must be pre scaled before the clock can drive the IR logic. This register sets the divisor rate to give a range to accommodate the remote control requirements in BAUD multiples of 12x. The value of the CFPS at reset is 0105 decimal which equates to a 38.1 KHz output from starting conditions. The 48 MHz carrier is prescaled by the CFPS which is then divided by the 12x BAUD multiple.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_cfps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_cfps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_cfps`]
module"]
#[doc(alias = "MEM_CFPS")]
pub type MemCfps = crate::Reg<mem_cfps::MemCfpsSpec>;
#[doc = "Since the Consumer IR works at modulation rates of 30 56.8 KHz, the 48 MHz clock must be pre scaled before the clock can drive the IR logic. This register sets the divisor rate to give a range to accommodate the remote control requirements in BAUD multiples of 12x. The value of the CFPS at reset is 0105 decimal which equates to a 38.1 KHz output from starting conditions. The 48 MHz carrier is prescaled by the CFPS which is then divided by the 12x BAUD multiple."]
pub mod mem_cfps;
#[doc = "MEM_RXFIFO_LVL (rw) register accessor: Level of the RX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rxfifo_lvl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_rxfifo_lvl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rxfifo_lvl`]
module"]
#[doc(alias = "MEM_RXFIFO_LVL")]
pub type MemRxfifoLvl = crate::Reg<mem_rxfifo_lvl::MemRxfifoLvlSpec>;
#[doc = "Level of the RX FIFO"]
pub mod mem_rxfifo_lvl;
#[doc = "MEM_TXFIFO_LVL (rw) register accessor: Level of the TX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_txfifo_lvl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_txfifo_lvl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_txfifo_lvl`]
module"]
#[doc(alias = "MEM_TXFIFO_LVL")]
pub type MemTxfifoLvl = crate::Reg<mem_txfifo_lvl::MemTxfifoLvlSpec>;
#[doc = "Level of the TX FIFO"]
pub mod mem_txfifo_lvl;
#[doc = "MEM_IER2 (rw) register accessor: Enables RX/TX FIFOs empty corresponding interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ier2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ier2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ier2`]
module"]
#[doc(alias = "MEM_IER2")]
pub type MemIer2 = crate::Reg<mem_ier2::MemIer2Spec>;
#[doc = "Enables RX/TX FIFOs empty corresponding interrupts."]
pub mod mem_ier2;
#[doc = "MEM_ISR2 (rw) register accessor: Status of RX/TX FIFOs empty corresponding interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_isr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_isr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_isr2`]
module"]
#[doc(alias = "MEM_ISR2")]
pub type MemIsr2 = crate::Reg<mem_isr2::MemIsr2Spec>;
#[doc = "Status of RX/TX FIFOs empty corresponding interrupts."]
pub mod mem_isr2;
#[doc = "MEM_FREQ_SEL (rw) register accessor: Sample per bit value selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_freq_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_freq_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_freq_sel`]
module"]
#[doc(alias = "MEM_FREQ_SEL")]
pub type MemFreqSel = crate::Reg<mem_freq_sel::MemFreqSelSpec>;
#[doc = "Sample per bit value selector"]
pub mod mem_freq_sel;
#[doc = "MEM_ABAUD_1ST_CHAR (rw) register accessor: Unused\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_abaud_1st_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_abaud_1st_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_abaud_1st_char`]
module"]
#[doc(alias = "MEM_ABAUD_1ST_CHAR")]
pub type MemAbaud1stChar = crate::Reg<mem_abaud_1st_char::MemAbaud1stCharSpec>;
#[doc = "Unused"]
pub mod mem_abaud_1st_char;
#[doc = "MEM_BAUD_2ND_CHAR (rw) register accessor: Unused\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_baud_2nd_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_baud_2nd_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_baud_2nd_char`]
module"]
#[doc(alias = "MEM_BAUD_2ND_CHAR")]
pub type MemBaud2ndChar = crate::Reg<mem_baud_2nd_char::MemBaud2ndCharSpec>;
#[doc = "Unused"]
pub mod mem_baud_2nd_char;
#[doc = "MEM_MDR3 (rw) register accessor: Mode definition register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mdr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mdr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mdr3`]
module"]
#[doc(alias = "MEM_MDR3")]
pub type MemMdr3 = crate::Reg<mem_mdr3::MemMdr3Spec>;
#[doc = "Mode definition register 3."]
pub mod mem_mdr3;
#[doc = "MEM_TX_DMA_THRESHOLD (rw) register accessor: Use to manually set the TX DMA threshold level. MDR3\\[2\\]
SET_TX_DMA_THRESHOLD must be one and must be value + tx_trigger_level &lt;= 64 (TX FIFO size). If not, 64-tx_trigger_level will be used w/o modifying the value of this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_tx_dma_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_tx_dma_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_tx_dma_threshold`]
module"]
#[doc(alias = "MEM_TX_DMA_THRESHOLD")]
pub type MemTxDmaThreshold = crate::Reg<mem_tx_dma_threshold::MemTxDmaThresholdSpec>;
#[doc = "Use to manually set the TX DMA threshold level. MDR3\\[2\\]
SET_TX_DMA_THRESHOLD must be one and must be value + tx_trigger_level &lt;= 64 (TX FIFO size). If not, 64-tx_trigger_level will be used w/o modifying the value of this register."]
pub mod mem_tx_dma_threshold;
#[doc = "MEM_MDR4 (rw) register accessor: Mode definition register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mdr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mdr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mdr4`]
module"]
#[doc(alias = "MEM_MDR4")]
pub type MemMdr4 = crate::Reg<mem_mdr4::MemMdr4Spec>;
#[doc = "Mode definition register 4"]
pub mod mem_mdr4;
#[doc = "MEM_EFR2 (rw) register accessor: Enhanced Features Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_efr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_efr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_efr2`]
module"]
#[doc(alias = "MEM_EFR2")]
pub type MemEfr2 = crate::Reg<mem_efr2::MemEfr2Spec>;
#[doc = "Enhanced Features Register 2"]
pub mod mem_efr2;
#[doc = "MEM_ECR (rw) register accessor: Enhanced Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ecr`]
module"]
#[doc(alias = "MEM_ECR")]
pub type MemEcr = crate::Reg<mem_ecr::MemEcrSpec>;
#[doc = "Enhanced Control register"]
pub mod mem_ecr;
#[doc = "MEM_TIMEGUARD (rw) register accessor: Timeguard\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_timeguard::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_timeguard::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_timeguard`]
module"]
#[doc(alias = "MEM_TIMEGUARD")]
pub type MemTimeguard = crate::Reg<mem_timeguard::MemTimeguardSpec>;
#[doc = "Timeguard"]
pub mod mem_timeguard;
#[doc = "MEM_TIMEOUTL (rw) register accessor: Timeout lower byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_timeoutl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_timeoutl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_timeoutl`]
module"]
#[doc(alias = "MEM_TIMEOUTL")]
pub type MemTimeoutl = crate::Reg<mem_timeoutl::MemTimeoutlSpec>;
#[doc = "Timeout lower byte"]
pub mod mem_timeoutl;
#[doc = "MEM_TIMEOUTH (rw) register accessor: Timeout higher byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_timeouth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_timeouth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_timeouth`]
module"]
#[doc(alias = "MEM_TIMEOUTH")]
pub type MemTimeouth = crate::Reg<mem_timeouth::MemTimeouthSpec>;
#[doc = "Timeout higher byte"]
pub mod mem_timeouth;
#[doc = "MEM_SCCR (rw) register accessor: Smartcard (ISO7816) mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_sccr`]
module"]
#[doc(alias = "MEM_SCCR")]
pub type MemSccr = crate::Reg<mem_sccr::MemSccrSpec>;
#[doc = "Smartcard (ISO7816) mode Control Register"]
pub mod mem_sccr;
#[doc = "MEM_ERHR (rw) register accessor: Extended Receive Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_erhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_erhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_erhr`]
module"]
#[doc(alias = "MEM_ERHR")]
pub type MemErhr = crate::Reg<mem_erhr::MemErhrSpec>;
#[doc = "Extended Receive Holding Register"]
pub mod mem_erhr;
#[doc = "MEM_ETHR (rw) register accessor: Extended Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ethr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ethr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ethr`]
module"]
#[doc(alias = "MEM_ETHR")]
pub type MemEthr = crate::Reg<mem_ethr::MemEthrSpec>;
#[doc = "Extended Transmit Holding Register"]
pub mod mem_ethr;
#[doc = "MEM_MAR (rw) register accessor: Multidrop Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mar`]
module"]
#[doc(alias = "MEM_MAR")]
pub type MemMar = crate::Reg<mem_mar::MemMarSpec>;
#[doc = "Multidrop Address Register"]
pub mod mem_mar;
#[doc = "MEM_MMR (rw) register accessor: Multidrop Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mmr`]
module"]
#[doc(alias = "MEM_MMR")]
pub type MemMmr = crate::Reg<mem_mmr::MemMmrSpec>;
#[doc = "Multidrop Mask Register"]
pub mod mem_mmr;
#[doc = "MEM_MBR (rw) register accessor: Multidrop Broadcast Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mbr`]
module"]
#[doc(alias = "MEM_MBR")]
pub type MemMbr = crate::Reg<mem_mbr::MemMbrSpec>;
#[doc = "Multidrop Broadcast Address Register"]
pub mod mem_mbr;
