#[doc = "Register `PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg0_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg0_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec>;
#[doc = "Field `PRU0_ED_TX_WDLY2` reader - "]
pub type Pru0EdTxWdly2R = crate::FieldReader<u16>;
#[doc = "Field `PRU0_ED_TX_WDLY2` writer - "]
pub type Pru0EdTxWdly2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PRU0_ED_TX_FRAME_SIZE2` reader - "]
pub type Pru0EdTxFrameSize2R = crate::FieldReader;
#[doc = "Field `PRU0_ED_TX_FRAME_SIZE2` writer - "]
pub type Pru0EdTxFrameSize2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_ED_RX_FRAME_SIZE2` reader - "]
pub type Pru0EdRxFrameSize2R = crate::FieldReader<u16>;
#[doc = "Field `PRU0_ED_RX_FRAME_SIZE2` writer - "]
pub type Pru0EdRxFrameSize2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PRU0_ED_RX_SNOOP2` reader - "]
pub type Pru0EdRxSnoop2R = crate::BitReader;
#[doc = "Field `PRU0_ED_RX_SNOOP2` writer - "]
pub type Pru0EdRxSnoop2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ED_CLK_OUT_OVR_EN2` reader - "]
pub type Pru0EdClkOutOvrEn2R = crate::BitReader;
#[doc = "Field `PRU0_ED_CLK_OUT_OVR_EN2` writer - "]
pub type Pru0EdClkOutOvrEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ED_SW_CLK_OUT2` reader - "]
pub type Pru0EdSwClkOut2R = crate::BitReader;
#[doc = "Field `PRU0_ED_SW_CLK_OUT2` writer - "]
pub type Pru0EdSwClkOut2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ED_TX_FIFO_SWAP_BITS2` reader - "]
pub type Pru0EdTxFifoSwapBits2R = crate::BitReader;
#[doc = "Field `PRU0_ED_TX_FIFO_SWAP_BITS2` writer - "]
pub type Pru0EdTxFifoSwapBits2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn pru0_ed_tx_wdly2(&self) -> Pru0EdTxWdly2R {
        Pru0EdTxWdly2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_ed_tx_frame_size2(&self) -> Pru0EdTxFrameSize2R {
        Pru0EdTxFrameSize2R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn pru0_ed_rx_frame_size2(&self) -> Pru0EdRxFrameSize2R {
        Pru0EdRxFrameSize2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pru0_ed_rx_snoop2(&self) -> Pru0EdRxSnoop2R {
        Pru0EdRxSnoop2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pru0_ed_clk_out_ovr_en2(&self) -> Pru0EdClkOutOvrEn2R {
        Pru0EdClkOutOvrEn2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pru0_ed_sw_clk_out2(&self) -> Pru0EdSwClkOut2R {
        Pru0EdSwClkOut2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pru0_ed_tx_fifo_swap_bits2(&self) -> Pru0EdTxFifoSwapBits2R {
        Pru0EdTxFifoSwapBits2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_tx_wdly2(&mut self) -> Pru0EdTxWdly2W<Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec> {
        Pru0EdTxWdly2W::new(self, 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_tx_frame_size2(
        &mut self,
    ) -> Pru0EdTxFrameSize2W<Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec> {
        Pru0EdTxFrameSize2W::new(self, 11)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_rx_frame_size2(
        &mut self,
    ) -> Pru0EdRxFrameSize2W<Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec> {
        Pru0EdRxFrameSize2W::new(self, 16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_rx_snoop2(&mut self) -> Pru0EdRxSnoop2W<Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec> {
        Pru0EdRxSnoop2W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_clk_out_ovr_en2(
        &mut self,
    ) -> Pru0EdClkOutOvrEn2W<Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec> {
        Pru0EdClkOutOvrEn2W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_sw_clk_out2(&mut self) -> Pru0EdSwClkOut2W<Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec> {
        Pru0EdSwClkOut2W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_tx_fifo_swap_bits2(
        &mut self,
    ) -> Pru0EdTxFifoSwapBits2W<Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec> {
        Pru0EdTxFifoSwapBits2W::new(self, 31)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg0_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec {
    const RESET_VALUE: u32 = 0;
}
