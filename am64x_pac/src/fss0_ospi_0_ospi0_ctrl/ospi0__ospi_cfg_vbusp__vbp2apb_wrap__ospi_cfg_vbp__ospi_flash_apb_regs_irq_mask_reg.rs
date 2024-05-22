#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_mask_reg` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_mask_reg` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec>;
#[doc = "Field `MODE_M_FAIL_MASK_FLD` reader - 0:0\\]
Mode M Failure Mask"]
pub type ModeMFailMaskFldR = crate::BitReader;
#[doc = "Field `MODE_M_FAIL_MASK_FLD` writer - 0:0\\]
Mode M Failure Mask"]
pub type ModeMFailMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW_DET_MASK_FLD` reader - 1:1\\]
Underflow Detected Mask"]
pub type UnderflowDetMaskFldR = crate::BitReader;
#[doc = "Field `UNDERFLOW_DET_MASK_FLD` writer - 1:1\\]
Underflow Detected Mask"]
pub type UnderflowDetMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECT_OP_DONE_MASK_FLD` reader - 2:2\\]
Indirect Complete Mask"]
pub type IndirectOpDoneMaskFldR = crate::BitReader;
#[doc = "Field `INDIRECT_OP_DONE_MASK_FLD` writer - 2:2\\]
Indirect Complete Mask"]
pub type IndirectOpDoneMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECT_READ_REJECT_MASK_FLD` reader - 3:3\\]
Indirect Read Reject Mask"]
pub type IndirectReadRejectMaskFldR = crate::BitReader;
#[doc = "Field `INDIRECT_READ_REJECT_MASK_FLD` writer - 3:3\\]
Indirect Read Reject Mask"]
pub type IndirectReadRejectMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROT_WR_ATTEMPT_MASK_FLD` reader - 4:4\\]
Protected Area Write Attempt Mask"]
pub type ProtWrAttemptMaskFldR = crate::BitReader;
#[doc = "Field `PROT_WR_ATTEMPT_MASK_FLD` writer - 4:4\\]
Protected Area Write Attempt Mask"]
pub type ProtWrAttemptMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLEGAL_ACCESS_DET_MASK_FLD` reader - 5:5\\]
Illegal Access Detected Mask"]
pub type IllegalAccessDetMaskFldR = crate::BitReader;
#[doc = "Field `ILLEGAL_ACCESS_DET_MASK_FLD` writer - 5:5\\]
Illegal Access Detected Mask"]
pub type IllegalAccessDetMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDIRECT_XFER_LEVEL_BREACH_MASK_FLD` reader - 6:6\\]
Transfer Watermark Breach Mask"]
pub type IndirectXferLevelBreachMaskFldR = crate::BitReader;
#[doc = "Field `INDIRECT_XFER_LEVEL_BREACH_MASK_FLD` writer - 6:6\\]
Transfer Watermark Breach Mask"]
pub type IndirectXferLevelBreachMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECV_OVERFLOW_MASK_FLD` reader - 7:7\\]
Receive Overflow Mask"]
pub type RecvOverflowMaskFldR = crate::BitReader;
#[doc = "Field `RECV_OVERFLOW_MASK_FLD` writer - 7:7\\]
Receive Overflow Mask"]
pub type RecvOverflowMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_NOT_FULL_MASK_FLD` reader - 8:8\\]
Small TX FIFO not full Mask"]
pub type TxFifoNotFullMaskFldR = crate::BitReader;
#[doc = "Field `TX_FIFO_NOT_FULL_MASK_FLD` writer - 8:8\\]
Small TX FIFO not full Mask"]
pub type TxFifoNotFullMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_FULL_MASK_FLD` reader - 9:9\\]
Small TX FIFO full Mask"]
pub type TxFifoFullMaskFldR = crate::BitReader;
#[doc = "Field `TX_FIFO_FULL_MASK_FLD` writer - 9:9\\]
Small TX FIFO full Mask"]
pub type TxFifoFullMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_NOT_EMPTY_MASK_FLD` reader - 10:10\\]
Small RX FIFO not empty Mask"]
pub type RxFifoNotEmptyMaskFldR = crate::BitReader;
#[doc = "Field `RX_FIFO_NOT_EMPTY_MASK_FLD` writer - 10:10\\]
Small RX FIFO not empty Mask"]
pub type RxFifoNotEmptyMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_FULL_MASK_FLD` reader - 11:11\\]
Small RX FIFO full Mask"]
pub type RxFifoFullMaskFldR = crate::BitReader;
#[doc = "Field `RX_FIFO_FULL_MASK_FLD` writer - 11:11\\]
Small RX FIFO full Mask"]
pub type RxFifoFullMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDRD_SRAM_FULL_MASK_FLD` reader - 12:12\\]
Indirect Read Partition overflow mask"]
pub type IndrdSramFullMaskFldR = crate::BitReader;
#[doc = "Field `INDRD_SRAM_FULL_MASK_FLD` writer - 12:12\\]
Indirect Read Partition overflow mask"]
pub type IndrdSramFullMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLL_EXP_INT_MASK_FLD` reader - 13:13\\]
Polling expiration detected Mask"]
pub type PollExpIntMaskFldR = crate::BitReader;
#[doc = "Field `POLL_EXP_INT_MASK_FLD` writer - 13:13\\]
Polling expiration detected Mask"]
pub type PollExpIntMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIG_REQ_MASK_FLD` reader - 14:14\\]
STIG request completion Mask"]
pub type StigReqMaskFldR = crate::BitReader;
#[doc = "Field `STIG_REQ_MASK_FLD` writer - 14:14\\]
STIG request completion Mask"]
pub type StigReqMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CRC_DATA_ERR_MASK_FLD` reader - 16:16\\]
RX CRC data error Mask"]
pub type RxCrcDataErrMaskFldR = crate::BitReader;
#[doc = "Field `RX_CRC_DATA_ERR_MASK_FLD` writer - 16:16\\]
RX CRC data error Mask"]
pub type RxCrcDataErrMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CRC_DATA_VAL_MASK_FLD` reader - 17:17\\]
RX CRC data valid Mask"]
pub type RxCrcDataValMaskFldR = crate::BitReader;
#[doc = "Field `RX_CRC_DATA_VAL_MASK_FLD` writer - 17:17\\]
RX CRC data valid Mask"]
pub type RxCrcDataValMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CRC_CHUNK_BRK_MASK_FLD` reader - 18:18\\]
TX CRC chunk was broken Mask"]
pub type TxCrcChunkBrkMaskFldR = crate::BitReader;
#[doc = "Field `TX_CRC_CHUNK_BRK_MASK_FLD` writer - 18:18\\]
TX CRC chunk was broken Mask"]
pub type TxCrcChunkBrkMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_FAIL_MASK_FLD` reader - 19:19\\]
ECC failure Mask"]
pub type EccFailMaskFldR = crate::BitReader;
#[doc = "Field `ECC_FAIL_MASK_FLD` writer - 19:19\\]
ECC failure Mask"]
pub type EccFailMaskFldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Mode M Failure Mask"]
    #[inline(always)]
    pub fn mode_m_fail_mask_fld(&self) -> ModeMFailMaskFldR {
        ModeMFailMaskFldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Underflow Detected Mask"]
    #[inline(always)]
    pub fn underflow_det_mask_fld(&self) -> UnderflowDetMaskFldR {
        UnderflowDetMaskFldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indirect Complete Mask"]
    #[inline(always)]
    pub fn indirect_op_done_mask_fld(&self) -> IndirectOpDoneMaskFldR {
        IndirectOpDoneMaskFldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indirect Read Reject Mask"]
    #[inline(always)]
    pub fn indirect_read_reject_mask_fld(&self) -> IndirectReadRejectMaskFldR {
        IndirectReadRejectMaskFldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Protected Area Write Attempt Mask"]
    #[inline(always)]
    pub fn prot_wr_attempt_mask_fld(&self) -> ProtWrAttemptMaskFldR {
        ProtWrAttemptMaskFldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Illegal Access Detected Mask"]
    #[inline(always)]
    pub fn illegal_access_det_mask_fld(&self) -> IllegalAccessDetMaskFldR {
        IllegalAccessDetMaskFldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Transfer Watermark Breach Mask"]
    #[inline(always)]
    pub fn indirect_xfer_level_breach_mask_fld(&self) -> IndirectXferLevelBreachMaskFldR {
        IndirectXferLevelBreachMaskFldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Overflow Mask"]
    #[inline(always)]
    pub fn recv_overflow_mask_fld(&self) -> RecvOverflowMaskFldR {
        RecvOverflowMaskFldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Small TX FIFO not full Mask"]
    #[inline(always)]
    pub fn tx_fifo_not_full_mask_fld(&self) -> TxFifoNotFullMaskFldR {
        TxFifoNotFullMaskFldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Small TX FIFO full Mask"]
    #[inline(always)]
    pub fn tx_fifo_full_mask_fld(&self) -> TxFifoFullMaskFldR {
        TxFifoFullMaskFldR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Small RX FIFO not empty Mask"]
    #[inline(always)]
    pub fn rx_fifo_not_empty_mask_fld(&self) -> RxFifoNotEmptyMaskFldR {
        RxFifoNotEmptyMaskFldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Small RX FIFO full Mask"]
    #[inline(always)]
    pub fn rx_fifo_full_mask_fld(&self) -> RxFifoFullMaskFldR {
        RxFifoFullMaskFldR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Indirect Read Partition overflow mask"]
    #[inline(always)]
    pub fn indrd_sram_full_mask_fld(&self) -> IndrdSramFullMaskFldR {
        IndrdSramFullMaskFldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Polling expiration detected Mask"]
    #[inline(always)]
    pub fn poll_exp_int_mask_fld(&self) -> PollExpIntMaskFldR {
        PollExpIntMaskFldR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
STIG request completion Mask"]
    #[inline(always)]
    pub fn stig_req_mask_fld(&self) -> StigReqMaskFldR {
        StigReqMaskFldR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
RX CRC data error Mask"]
    #[inline(always)]
    pub fn rx_crc_data_err_mask_fld(&self) -> RxCrcDataErrMaskFldR {
        RxCrcDataErrMaskFldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
RX CRC data valid Mask"]
    #[inline(always)]
    pub fn rx_crc_data_val_mask_fld(&self) -> RxCrcDataValMaskFldR {
        RxCrcDataValMaskFldR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
TX CRC chunk was broken Mask"]
    #[inline(always)]
    pub fn tx_crc_chunk_brk_mask_fld(&self) -> TxCrcChunkBrkMaskFldR {
        TxCrcChunkBrkMaskFldR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
ECC failure Mask"]
    #[inline(always)]
    pub fn ecc_fail_mask_fld(&self) -> EccFailMaskFldR {
        EccFailMaskFldR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Mode M Failure Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mode_m_fail_mask_fld(
        &mut self,
    ) -> ModeMFailMaskFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec>
    {
        ModeMFailMaskFldW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Underflow Detected Mask"]
    #[inline(always)]
    #[must_use]
    pub fn underflow_det_mask_fld(
        &mut self,
    ) -> UnderflowDetMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        UnderflowDetMaskFldW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indirect Complete Mask"]
    #[inline(always)]
    #[must_use]
    pub fn indirect_op_done_mask_fld(
        &mut self,
    ) -> IndirectOpDoneMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        IndirectOpDoneMaskFldW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indirect Read Reject Mask"]
    #[inline(always)]
    #[must_use]
    pub fn indirect_read_reject_mask_fld(
        &mut self,
    ) -> IndirectReadRejectMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        IndirectReadRejectMaskFldW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Protected Area Write Attempt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn prot_wr_attempt_mask_fld(
        &mut self,
    ) -> ProtWrAttemptMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        ProtWrAttemptMaskFldW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Illegal Access Detected Mask"]
    #[inline(always)]
    #[must_use]
    pub fn illegal_access_det_mask_fld(
        &mut self,
    ) -> IllegalAccessDetMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        IllegalAccessDetMaskFldW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Transfer Watermark Breach Mask"]
    #[inline(always)]
    #[must_use]
    pub fn indirect_xfer_level_breach_mask_fld(
        &mut self,
    ) -> IndirectXferLevelBreachMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        IndirectXferLevelBreachMaskFldW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Overflow Mask"]
    #[inline(always)]
    #[must_use]
    pub fn recv_overflow_mask_fld(
        &mut self,
    ) -> RecvOverflowMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        RecvOverflowMaskFldW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Small TX FIFO not full Mask"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_not_full_mask_fld(
        &mut self,
    ) -> TxFifoNotFullMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        TxFifoNotFullMaskFldW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Small TX FIFO full Mask"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_full_mask_fld(
        &mut self,
    ) -> TxFifoFullMaskFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec>
    {
        TxFifoFullMaskFldW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Small RX FIFO not empty Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_not_empty_mask_fld(
        &mut self,
    ) -> RxFifoNotEmptyMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        RxFifoNotEmptyMaskFldW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Small RX FIFO full Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_full_mask_fld(
        &mut self,
    ) -> RxFifoFullMaskFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec>
    {
        RxFifoFullMaskFldW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Indirect Read Partition overflow mask"]
    #[inline(always)]
    #[must_use]
    pub fn indrd_sram_full_mask_fld(
        &mut self,
    ) -> IndrdSramFullMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        IndrdSramFullMaskFldW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Polling expiration detected Mask"]
    #[inline(always)]
    #[must_use]
    pub fn poll_exp_int_mask_fld(
        &mut self,
    ) -> PollExpIntMaskFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec>
    {
        PollExpIntMaskFldW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
STIG request completion Mask"]
    #[inline(always)]
    #[must_use]
    pub fn stig_req_mask_fld(
        &mut self,
    ) -> StigReqMaskFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec>
    {
        StigReqMaskFldW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
RX CRC data error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_data_err_mask_fld(
        &mut self,
    ) -> RxCrcDataErrMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        RxCrcDataErrMaskFldW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
RX CRC data valid Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_data_val_mask_fld(
        &mut self,
    ) -> RxCrcDataValMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        RxCrcDataValMaskFldW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
TX CRC chunk was broken Mask"]
    #[inline(always)]
    #[must_use]
    pub fn tx_crc_chunk_brk_mask_fld(
        &mut self,
    ) -> TxCrcChunkBrkMaskFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec,
    > {
        TxCrcChunkBrkMaskFldW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
ECC failure Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_fail_mask_fld(
        &mut self,
    ) -> EccFailMaskFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec>
    {
        EccFailMaskFldW::new(self, 19)
    }
}
#[doc = "Interrupt Mask: 0 : the interrupt for the corresponding interrupt status register bit is disabled. 1 : the interrupt for the corresponding interrupt status register bit is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_mask_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec
{
    const RESET_VALUE: u32 = 0;
}
