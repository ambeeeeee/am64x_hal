#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts_ena` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts_ena` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec>;
#[doc = "Field `CMD_COMPLETE` reader - 0:0\\]
'0' Masked '1' Enabled"]
pub type CmdCompleteR = crate::BitReader;
#[doc = "Field `CMD_COMPLETE` writer - 0:0\\]
'0' Masked '1' Enabled"]
pub type CmdCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFER_COMPLETE` reader - 1:1\\]
'0' Masked '1' Enabled"]
pub type XferCompleteR = crate::BitReader;
#[doc = "Field `XFER_COMPLETE` writer - 1:1\\]
'0' Masked '1' Enabled"]
pub type XferCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_GAP_EVENT` reader - 2:2\\]
'0' Masked '1' Enabled"]
pub type BlkGapEventR = crate::BitReader;
#[doc = "Field `BLK_GAP_EVENT` writer - 2:2\\]
'0' Masked '1' Enabled"]
pub type BlkGapEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_INTERRUPT` reader - 3:3\\]
'0' Masked, '1' Enabled"]
pub type DmaInterruptR = crate::BitReader;
#[doc = "Field `DMA_INTERRUPT` writer - 3:3\\]
'0' Masked, '1' Enabled"]
pub type DmaInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_WR_READY` reader - 4:4\\]
'0' Masked '1' Enabled"]
pub type BufWrReadyR = crate::BitReader;
#[doc = "Field `BUF_WR_READY` writer - 4:4\\]
'0' Masked '1' Enabled"]
pub type BufWrReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_RD_READY` reader - 5:5\\]
'0' Masked '1' Enabled"]
pub type BufRdReadyR = crate::BitReader;
#[doc = "Field `BUF_RD_READY` writer - 5:5\\]
'0' Masked '1' Enabled"]
pub type BufRdReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERTION` reader - 6:6\\]
'0' Masked '1' Enabled"]
pub type CardInsertionR = crate::BitReader;
#[doc = "Field `CARD_INSERTION` writer - 6:6\\]
'0' Masked '1' Enabled"]
pub type CardInsertionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REMOVAL` reader - 7:7\\]
'0' Masked '1' Enabled"]
pub type CardRemovalR = crate::BitReader;
#[doc = "Field `CARD_REMOVAL` writer - 7:7\\]
'0' Masked '1' Enabled"]
pub type CardRemovalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INTERRUPT` reader - 8:8\\]
If this bit is set to 0, the HC shall clear Interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The HD may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all Interrupt requests from the card are cleared to prevent inadvertent Interrupts. By setting this bit to 0, interrupt input should be masked by implementation so that the interrupt Input is not affected by external signal in any state \\[ex. floating\\]."]
pub type CardInterruptR = crate::BitReader;
#[doc = "Field `CARD_INTERRUPT` writer - 8:8\\]
If this bit is set to 0, the HC shall clear Interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The HD may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all Interrupt requests from the card are cleared to prevent inadvertent Interrupts. By setting this bit to 0, interrupt input should be masked by implementation so that the interrupt Input is not affected by external signal in any state \\[ex. floating\\]."]
pub type CardInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTA` reader - 9:9\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_A and may set this bit again after all interrupt requests to INT_A pin are cleared to prevent inadvertent interrupts."]
pub type IntaR = crate::BitReader;
#[doc = "Field `INTA` writer - 9:9\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_A and may set this bit again after all interrupt requests to INT_A pin are cleared to prevent inadvertent interrupts."]
pub type IntaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTB` reader - 10:10\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_B and may set this bit again after all interrupt requests to INT_B pin are cleared to prevent inadvertent interrupts."]
pub type IntbR = crate::BitReader;
#[doc = "Field `INTB` writer - 10:10\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_B and may set this bit again after all interrupt requests to INT_B pin are cleared to prevent inadvertent interrupts."]
pub type IntbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTC` reader - 11:11\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_C and may set this bit again after all interrupt requests to INT_C pin are cleared to prevent inadvertent interrupts."]
pub type IntcR = crate::BitReader;
#[doc = "Field `INTC` writer - 11:11\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_C and may set this bit again after all interrupt requests to INT_C pin are cleared to prevent inadvertent interrupts."]
pub type IntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNING_EVENT` reader - 12:12\\]
0 - Masked 1 - Enabled"]
pub type RetuningEventR = crate::BitReader;
#[doc = "Field `RETUNING_EVENT` writer - 12:12\\]
0 - Masked 1 - Enabled"]
pub type RetuningEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCV_BOOT_ACK` reader - 13:13\\]
'0' Masked '1' Enabled"]
pub type RcvBootAckR = crate::BitReader;
#[doc = "Field `RCV_BOOT_ACK` writer - 13:13\\]
'0' Masked '1' Enabled"]
pub type RcvBootAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_COMPLETE` reader - 14:14\\]
'0' Masked '1' Enabled"]
pub type BootCompleteR = crate::BitReader;
#[doc = "Field `BOOT_COMPLETE` writer - 14:14\\]
'0' Masked '1' Enabled"]
pub type BootCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT15_FIXED0` reader - 15:15\\]
The HC shall control error Interrupts using the Error Interrupt Status Enable register."]
pub type Bit15Fixed0R = crate::BitReader;
#[doc = "Field `BIT15_FIXED0` writer - 15:15\\]
The HC shall control error Interrupts using the Error Interrupt Status Enable register."]
pub type Bit15Fixed0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CmdCompleteR {
        CmdCompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn xfer_complete(&self) -> XferCompleteR {
        XferCompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn blk_gap_event(&self) -> BlkGapEventR {
        BlkGapEventR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
'0' Masked, '1' Enabled"]
    #[inline(always)]
    pub fn dma_interrupt(&self) -> DmaInterruptR {
        DmaInterruptR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready(&self) -> BufWrReadyR {
        BufWrReadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready(&self) -> BufRdReadyR {
        BufRdReadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn card_insertion(&self) -> CardInsertionR {
        CardInsertionR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn card_removal(&self) -> CardRemovalR {
        CardRemovalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
If this bit is set to 0, the HC shall clear Interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The HD may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all Interrupt requests from the card are cleared to prevent inadvertent Interrupts. By setting this bit to 0, interrupt input should be masked by implementation so that the interrupt Input is not affected by external signal in any state \\[ex. floating\\]."]
    #[inline(always)]
    pub fn card_interrupt(&self) -> CardInterruptR {
        CardInterruptR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_A and may set this bit again after all interrupt requests to INT_A pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    pub fn inta(&self) -> IntaR {
        IntaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_B and may set this bit again after all interrupt requests to INT_B pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    pub fn intb(&self) -> IntbR {
        IntbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_C and may set this bit again after all interrupt requests to INT_C pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    pub fn intc(&self) -> IntcR {
        IntcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0 - Masked 1 - Enabled"]
    #[inline(always)]
    pub fn retuning_event(&self) -> RetuningEventR {
        RetuningEventR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn rcv_boot_ack(&self) -> RcvBootAckR {
        RcvBootAckR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn boot_complete(&self) -> BootCompleteR {
        BootCompleteR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
The HC shall control error Interrupts using the Error Interrupt Status Enable register."]
    #[inline(always)]
    pub fn bit15_fixed0(&self) -> Bit15Fixed0R {
        Bit15Fixed0R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete(&mut self) -> CmdCompleteW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        CmdCompleteW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete(&mut self) -> XferCompleteW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        XferCompleteW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn blk_gap_event(&mut self) -> BlkGapEventW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        BlkGapEventW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
'0' Masked, '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dma_interrupt(&mut self) -> DmaInterruptW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        DmaInterruptW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ready(&mut self) -> BufWrReadyW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        BufWrReadyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ready(&mut self) -> BufRdReadyW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        BufRdReadyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_insertion(&mut self) -> CardInsertionW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        CardInsertionW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal(&mut self) -> CardRemovalW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        CardRemovalW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
If this bit is set to 0, the HC shall clear Interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The HD may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all Interrupt requests from the card are cleared to prevent inadvertent Interrupts. By setting this bit to 0, interrupt input should be masked by implementation so that the interrupt Input is not affected by external signal in any state \\[ex. floating\\]."]
    #[inline(always)]
    #[must_use]
    pub fn card_interrupt(&mut self) -> CardInterruptW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        CardInterruptW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_A and may set this bit again after all interrupt requests to INT_A pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn inta(&mut self) -> IntaW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        IntaW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_B and may set this bit again after all interrupt requests to INT_B pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn intb(&mut self) -> IntbW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        IntbW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
If this bit is set to 0, the Host Controller shall clear the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_C and may set this bit again after all interrupt requests to INT_C pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn intc(&mut self) -> IntcW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        IntcW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
0 - Masked 1 - Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn retuning_event(&mut self) -> RetuningEventW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        RetuningEventW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rcv_boot_ack(&mut self) -> RcvBootAckW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        RcvBootAckW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn boot_complete(&mut self) -> BootCompleteW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        BootCompleteW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
The HC shall control error Interrupts using the Error Interrupt Status Enable register."]
    #[inline(always)]
    #[must_use]
    pub fn bit15_fixed0(&mut self) -> Bit15Fixed0W<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec> {
        Bit15Fixed0W::new(self, 15)
    }
}
#[doc = "This register is used to enable the normal interrupt status register fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts_ena to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec {
    const RESET_VALUE: u16 = 0;
}
