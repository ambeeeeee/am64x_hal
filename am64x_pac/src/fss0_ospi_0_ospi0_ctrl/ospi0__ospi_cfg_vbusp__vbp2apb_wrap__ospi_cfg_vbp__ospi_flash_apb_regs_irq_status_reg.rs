#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_status_reg` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_status_reg` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>;
#[doc = "Field `MODE_M_FAIL_FLD` reader - 0:0\\]
Mode M Failure: Mode M failure indicates the voltage on pin n_ss_in is inconsistent with the SPI mode. Set =1 if n_ss_in is low in master mode \\[multi-master contention\\]. These conditions will clear the spi_enable bit and disable the SPI. This bit is reset only by a system reset and cleared only when this register is read. 0 : no mode fault has been detected 1 : a mode fault has occurred"]
pub type ModeMFailFldR = crate::BitReader;
#[doc = "Field `MODE_M_FAIL_FLD` writer - 0:0\\]
Mode M Failure: Mode M failure indicates the voltage on pin n_ss_in is inconsistent with the SPI mode. Set =1 if n_ss_in is low in master mode \\[multi-master contention\\]. These conditions will clear the spi_enable bit and disable the SPI. This bit is reset only by a system reset and cleared only when this register is read. 0 : no mode fault has been detected 1 : a mode fault has occurred"]
pub type ModeMFailFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW_DET_FLD` reader - 1:1\\]
Underflow Detected: 0 : no underflow has been detected 1 : underflow is detected and an attempt to transfer data is made when the small TX FIFO is empty. This may occur when AHB write data is being supplied too slowly to keep up with the requested write operation This bit is reset only by a system reset and cleared only when the register is read."]
pub type UnderflowDetFldR = crate::BitReader;
#[doc = "Field `UNDERFLOW_DET_FLD` writer - 1:1\\]
Underflow Detected: 0 : no underflow has been detected 1 : underflow is detected and an attempt to transfer data is made when the small TX FIFO is empty. This may occur when AHB write data is being supplied too slowly to keep up with the requested write operation This bit is reset only by a system reset and cleared only when the register is read."]
pub type UnderflowDetFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECT_OP_DONE_FLD` reader - 2:2\\]
Indirect Operation Complete: Controller has completed last triggered indirect operation"]
pub type IndirectOpDoneFldR = crate::BitReader;
#[doc = "Field `INDIRECT_OP_DONE_FLD` writer - 2:2\\]
Indirect Operation Complete: Controller has completed last triggered indirect operation"]
pub type IndirectOpDoneFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECT_READ_REJECT_FLD` reader - 3:3\\]
Indirect operation was requested but could not be accepted. Two indirect operations already in storage."]
pub type IndirectReadRejectFldR = crate::BitReader;
#[doc = "Field `INDIRECT_READ_REJECT_FLD` writer - 3:3\\]
Indirect operation was requested but could not be accepted. Two indirect operations already in storage."]
pub type IndirectReadRejectFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROT_WR_ATTEMPT_FLD` reader - 4:4\\]
Write to protected area was attempted and rejected."]
pub type ProtWrAttemptFldR = crate::BitReader;
#[doc = "Field `PROT_WR_ATTEMPT_FLD` writer - 4:4\\]
Write to protected area was attempted and rejected."]
pub type ProtWrAttemptFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLEGAL_ACCESS_DET_FLD` reader - 5:5\\]
Illegal AHB access has been detected. AHB wrapping bursts and the use of SPLIT/RETRY accesses will cause this error interrupt to trigger."]
pub type IllegalAccessDetFldR = crate::BitReader;
#[doc = "Field `ILLEGAL_ACCESS_DET_FLD` writer - 5:5\\]
Illegal AHB access has been detected. AHB wrapping bursts and the use of SPLIT/RETRY accesses will cause this error interrupt to trigger."]
pub type IllegalAccessDetFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECT_XFER_LEVEL_BREACH_FLD` reader - 6:6\\]
Indirect Transfer Watermark Level Breached"]
pub type IndirectXferLevelBreachFldR = crate::BitReader;
#[doc = "Field `INDIRECT_XFER_LEVEL_BREACH_FLD` writer - 6:6\\]
Indirect Transfer Watermark Level Breached"]
pub type IndirectXferLevelBreachFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECV_OVERFLOW_FLD` reader - 7:7\\]
Receive Overflow: This should only occur in Legacy SPI mode. Set if an attempt is made to push the RX FIFO when it is full. This bit is reset only by a system reset and cleared only when this register is read. If a new push to the RX FIFO occurs coincident with a register read this flag will remain set. 0 : no overflow has been detected. 1 : an overflow has occurred."]
pub type RecvOverflowFldR = crate::BitReader;
#[doc = "Field `RECV_OVERFLOW_FLD` writer - 7:7\\]
Receive Overflow: This should only occur in Legacy SPI mode. Set if an attempt is made to push the RX FIFO when it is full. This bit is reset only by a system reset and cleared only when this register is read. If a new push to the RX FIFO occurs coincident with a register read this flag will remain set. 0 : no overflow has been detected. 1 : an overflow has occurred."]
pub type RecvOverflowFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_NOT_FULL_FLD` reader - 8:8\\]
Small TX FIFO not full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO has >= THRESHOLD entries, 1 : FIFO has less than THRESHOLD entries"]
pub type TxFifoNotFullFldR = crate::BitReader;
#[doc = "Field `TX_FIFO_NOT_FULL_FLD` writer - 8:8\\]
Small TX FIFO not full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO has >= THRESHOLD entries, 1 : FIFO has less than THRESHOLD entries"]
pub type TxFifoNotFullFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_FULL_FLD` reader - 9:9\\]
Small TX FIFO full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO is not full, 1 : FIFO is full"]
pub type TxFifoFullFldR = crate::BitReader;
#[doc = "Field `TX_FIFO_FULL_FLD` writer - 9:9\\]
Small TX FIFO full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO is not full, 1 : FIFO is full"]
pub type TxFifoFullFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_NOT_EMPTY_FLD` reader - 10:10\\]
Small RX FIFO not empty: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO has less than RX THRESHOLD entries, 1 : FIFO has >= THRESHOLD entries"]
pub type RxFifoNotEmptyFldR = crate::BitReader;
#[doc = "Field `RX_FIFO_NOT_EMPTY_FLD` writer - 10:10\\]
Small RX FIFO not empty: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO has less than RX THRESHOLD entries, 1 : FIFO has >= THRESHOLD entries"]
pub type RxFifoNotEmptyFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_FULL_FLD` reader - 11:11\\]
Small RX FIFO full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO is not full 1 : FIFO is full"]
pub type RxFifoFullFldR = crate::BitReader;
#[doc = "Field `RX_FIFO_FULL_FLD` writer - 11:11\\]
Small RX FIFO full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO is not full 1 : FIFO is full"]
pub type RxFifoFullFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDRD_SRAM_FULL_FLD` reader - 12:12\\]
Indirect Read Partition overflow: Indirect Read Partition of SRAM is full and unable to immediately complete indirect operation"]
pub type IndrdSramFullFldR = crate::BitReader;
#[doc = "Field `INDRD_SRAM_FULL_FLD` writer - 12:12\\]
Indirect Read Partition overflow: Indirect Read Partition of SRAM is full and unable to immediately complete indirect operation"]
pub type IndrdSramFullFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLL_EXP_INT_FLD` reader - 13:13\\]
The maximum number of programmed polls cycles is expired"]
pub type PollExpIntFldR = crate::BitReader;
#[doc = "Field `POLL_EXP_INT_FLD` writer - 13:13\\]
The maximum number of programmed polls cycles is expired"]
pub type PollExpIntFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIG_REQ_INT_FLD` reader - 14:14\\]
The controller is ready for getting another STIG request."]
pub type StigReqIntFldR = crate::BitReader;
#[doc = "Field `STIG_REQ_INT_FLD` writer - 14:14\\]
The controller is ready for getting another STIG request."]
pub type StigReqIntFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CRC_DATA_ERR_FLD` reader - 16:16\\]
RX CRC data error CRC data from Flash Device does not correspond to the one dynamically calculated by the controller."]
pub type RxCrcDataErrFldR = crate::BitReader;
#[doc = "Field `RX_CRC_DATA_ERR_FLD` writer - 16:16\\]
RX CRC data error CRC data from Flash Device does not correspond to the one dynamically calculated by the controller."]
pub type RxCrcDataErrFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CRC_DATA_VAL_FLD` reader - 17:17\\]
RX CRC data valid New RX CRC data was captured from Flash Device"]
pub type RxCrcDataValFldR = crate::BitReader;
#[doc = "Field `RX_CRC_DATA_VAL_FLD` writer - 17:17\\]
RX CRC data valid New RX CRC data was captured from Flash Device"]
pub type RxCrcDataValFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CRC_CHUNK_BRK_FLD` reader - 18:18\\]
TX CRC chunk was broken This interrupt informs the system that program page SPI transfer was discontinued somewhere inside the chunk."]
pub type TxCrcChunkBrkFldR = crate::BitReader;
#[doc = "Field `TX_CRC_CHUNK_BRK_FLD` writer - 18:18\\]
TX CRC chunk was broken This interrupt informs the system that program page SPI transfer was discontinued somewhere inside the chunk."]
pub type TxCrcChunkBrkFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_FAIL_FLD` reader - 19:19\\]
ECC failure This interrupt informs the system that Flash Device reported ECC error."]
pub type EccFailFldR = crate::BitReader;
#[doc = "Field `ECC_FAIL_FLD` writer - 19:19\\]
ECC failure This interrupt informs the system that Flash Device reported ECC error."]
pub type EccFailFldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Mode M Failure: Mode M failure indicates the voltage on pin n_ss_in is inconsistent with the SPI mode. Set =1 if n_ss_in is low in master mode \\[multi-master contention\\]. These conditions will clear the spi_enable bit and disable the SPI. This bit is reset only by a system reset and cleared only when this register is read. 0 : no mode fault has been detected 1 : a mode fault has occurred"]
    #[inline(always)]
    pub fn mode_m_fail_fld(&self) -> ModeMFailFldR {
        ModeMFailFldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Underflow Detected: 0 : no underflow has been detected 1 : underflow is detected and an attempt to transfer data is made when the small TX FIFO is empty. This may occur when AHB write data is being supplied too slowly to keep up with the requested write operation This bit is reset only by a system reset and cleared only when the register is read."]
    #[inline(always)]
    pub fn underflow_det_fld(&self) -> UnderflowDetFldR {
        UnderflowDetFldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indirect Operation Complete: Controller has completed last triggered indirect operation"]
    #[inline(always)]
    pub fn indirect_op_done_fld(&self) -> IndirectOpDoneFldR {
        IndirectOpDoneFldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indirect operation was requested but could not be accepted. Two indirect operations already in storage."]
    #[inline(always)]
    pub fn indirect_read_reject_fld(&self) -> IndirectReadRejectFldR {
        IndirectReadRejectFldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Write to protected area was attempted and rejected."]
    #[inline(always)]
    pub fn prot_wr_attempt_fld(&self) -> ProtWrAttemptFldR {
        ProtWrAttemptFldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Illegal AHB access has been detected. AHB wrapping bursts and the use of SPLIT/RETRY accesses will cause this error interrupt to trigger."]
    #[inline(always)]
    pub fn illegal_access_det_fld(&self) -> IllegalAccessDetFldR {
        IllegalAccessDetFldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indirect Transfer Watermark Level Breached"]
    #[inline(always)]
    pub fn indirect_xfer_level_breach_fld(&self) -> IndirectXferLevelBreachFldR {
        IndirectXferLevelBreachFldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Overflow: This should only occur in Legacy SPI mode. Set if an attempt is made to push the RX FIFO when it is full. This bit is reset only by a system reset and cleared only when this register is read. If a new push to the RX FIFO occurs coincident with a register read this flag will remain set. 0 : no overflow has been detected. 1 : an overflow has occurred."]
    #[inline(always)]
    pub fn recv_overflow_fld(&self) -> RecvOverflowFldR {
        RecvOverflowFldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Small TX FIFO not full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO has >= THRESHOLD entries, 1 : FIFO has less than THRESHOLD entries"]
    #[inline(always)]
    pub fn tx_fifo_not_full_fld(&self) -> TxFifoNotFullFldR {
        TxFifoNotFullFldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Small TX FIFO full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO is not full, 1 : FIFO is full"]
    #[inline(always)]
    pub fn tx_fifo_full_fld(&self) -> TxFifoFullFldR {
        TxFifoFullFldR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Small RX FIFO not empty: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO has less than RX THRESHOLD entries, 1 : FIFO has >= THRESHOLD entries"]
    #[inline(always)]
    pub fn rx_fifo_not_empty_fld(&self) -> RxFifoNotEmptyFldR {
        RxFifoNotEmptyFldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Small RX FIFO full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO is not full 1 : FIFO is full"]
    #[inline(always)]
    pub fn rx_fifo_full_fld(&self) -> RxFifoFullFldR {
        RxFifoFullFldR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Indirect Read Partition overflow: Indirect Read Partition of SRAM is full and unable to immediately complete indirect operation"]
    #[inline(always)]
    pub fn indrd_sram_full_fld(&self) -> IndrdSramFullFldR {
        IndrdSramFullFldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
The maximum number of programmed polls cycles is expired"]
    #[inline(always)]
    pub fn poll_exp_int_fld(&self) -> PollExpIntFldR {
        PollExpIntFldR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
The controller is ready for getting another STIG request."]
    #[inline(always)]
    pub fn stig_req_int_fld(&self) -> StigReqIntFldR {
        StigReqIntFldR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
RX CRC data error CRC data from Flash Device does not correspond to the one dynamically calculated by the controller."]
    #[inline(always)]
    pub fn rx_crc_data_err_fld(&self) -> RxCrcDataErrFldR {
        RxCrcDataErrFldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
RX CRC data valid New RX CRC data was captured from Flash Device"]
    #[inline(always)]
    pub fn rx_crc_data_val_fld(&self) -> RxCrcDataValFldR {
        RxCrcDataValFldR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
TX CRC chunk was broken This interrupt informs the system that program page SPI transfer was discontinued somewhere inside the chunk."]
    #[inline(always)]
    pub fn tx_crc_chunk_brk_fld(&self) -> TxCrcChunkBrkFldR {
        TxCrcChunkBrkFldR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
ECC failure This interrupt informs the system that Flash Device reported ECC error."]
    #[inline(always)]
    pub fn ecc_fail_fld(&self) -> EccFailFldR {
        EccFailFldR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Mode M Failure: Mode M failure indicates the voltage on pin n_ss_in is inconsistent with the SPI mode. Set =1 if n_ss_in is low in master mode \\[multi-master contention\\]. These conditions will clear the spi_enable bit and disable the SPI. This bit is reset only by a system reset and cleared only when this register is read. 0 : no mode fault has been detected 1 : a mode fault has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn mode_m_fail_fld(
        &mut self,
    ) -> ModeMFailFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        ModeMFailFldW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Underflow Detected: 0 : no underflow has been detected 1 : underflow is detected and an attempt to transfer data is made when the small TX FIFO is empty. This may occur when AHB write data is being supplied too slowly to keep up with the requested write operation This bit is reset only by a system reset and cleared only when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn underflow_det_fld(
        &mut self,
    ) -> UnderflowDetFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        UnderflowDetFldW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indirect Operation Complete: Controller has completed last triggered indirect operation"]
    #[inline(always)]
    #[must_use]
    pub fn indirect_op_done_fld(
        &mut self,
    ) -> IndirectOpDoneFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec,
    > {
        IndirectOpDoneFldW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indirect operation was requested but could not be accepted. Two indirect operations already in storage."]
    #[inline(always)]
    #[must_use]
    pub fn indirect_read_reject_fld(
        &mut self,
    ) -> IndirectReadRejectFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec,
    > {
        IndirectReadRejectFldW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Write to protected area was attempted and rejected."]
    #[inline(always)]
    #[must_use]
    pub fn prot_wr_attempt_fld(
        &mut self,
    ) -> ProtWrAttemptFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        ProtWrAttemptFldW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Illegal AHB access has been detected. AHB wrapping bursts and the use of SPLIT/RETRY accesses will cause this error interrupt to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn illegal_access_det_fld(
        &mut self,
    ) -> IllegalAccessDetFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec,
    > {
        IllegalAccessDetFldW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Indirect Transfer Watermark Level Breached"]
    #[inline(always)]
    #[must_use]
    pub fn indirect_xfer_level_breach_fld(
        &mut self,
    ) -> IndirectXferLevelBreachFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec,
    > {
        IndirectXferLevelBreachFldW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Overflow: This should only occur in Legacy SPI mode. Set if an attempt is made to push the RX FIFO when it is full. This bit is reset only by a system reset and cleared only when this register is read. If a new push to the RX FIFO occurs coincident with a register read this flag will remain set. 0 : no overflow has been detected. 1 : an overflow has occurred."]
    #[inline(always)]
    #[must_use]
    pub fn recv_overflow_fld(
        &mut self,
    ) -> RecvOverflowFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        RecvOverflowFldW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Small TX FIFO not full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO has >= THRESHOLD entries, 1 : FIFO has less than THRESHOLD entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_not_full_fld(
        &mut self,
    ) -> TxFifoNotFullFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        TxFifoNotFullFldW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Small TX FIFO full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO is not full, 1 : FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_full_fld(
        &mut self,
    ) -> TxFifoFullFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        TxFifoFullFldW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Small RX FIFO not empty: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO has less than RX THRESHOLD entries, 1 : FIFO has >= THRESHOLD entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_not_empty_fld(
        &mut self,
    ) -> RxFifoNotEmptyFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec,
    > {
        RxFifoNotEmptyFldW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Small RX FIFO full: Current FIFO status can be ignored in non-SPI legacy mode 0 : FIFO is not full 1 : FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_full_fld(
        &mut self,
    ) -> RxFifoFullFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        RxFifoFullFldW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Indirect Read Partition overflow: Indirect Read Partition of SRAM is full and unable to immediately complete indirect operation"]
    #[inline(always)]
    #[must_use]
    pub fn indrd_sram_full_fld(
        &mut self,
    ) -> IndrdSramFullFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        IndrdSramFullFldW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
The maximum number of programmed polls cycles is expired"]
    #[inline(always)]
    #[must_use]
    pub fn poll_exp_int_fld(
        &mut self,
    ) -> PollExpIntFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        PollExpIntFldW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
The controller is ready for getting another STIG request."]
    #[inline(always)]
    #[must_use]
    pub fn stig_req_int_fld(
        &mut self,
    ) -> StigReqIntFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        StigReqIntFldW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
RX CRC data error CRC data from Flash Device does not correspond to the one dynamically calculated by the controller."]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_data_err_fld(
        &mut self,
    ) -> RxCrcDataErrFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        RxCrcDataErrFldW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
RX CRC data valid New RX CRC data was captured from Flash Device"]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_data_val_fld(
        &mut self,
    ) -> RxCrcDataValFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        RxCrcDataValFldW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
TX CRC chunk was broken This interrupt informs the system that program page SPI transfer was discontinued somewhere inside the chunk."]
    #[inline(always)]
    #[must_use]
    pub fn tx_crc_chunk_brk_fld(
        &mut self,
    ) -> TxCrcChunkBrkFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        TxCrcChunkBrkFldW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
ECC failure This interrupt informs the system that Flash Device reported ECC error."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_fail_fld(
        &mut self,
    ) -> EccFailFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec>
    {
        EccFailFldW::new(self, 19)
    }
}
#[doc = "Interrupt Status Register: The status fields in this register are set when the described event occurs and the interrupt is enabled in the mask register. When any of these bit fields are set, the interrupt output is asserted high. The fields are each cleared by writing a 1 to the field. Note that bit fields 6 thru 10 are only valid when legacy SPI mode is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_status_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec
{
    const RESET_VALUE: u32 = 0;
}
