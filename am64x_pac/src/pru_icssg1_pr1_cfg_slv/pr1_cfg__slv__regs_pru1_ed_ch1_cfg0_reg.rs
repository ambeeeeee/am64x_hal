#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg0_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg0_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec>;
#[doc = "Field `PRU1_ED_TX_WDLY1` reader - "]
pub type Pru1EdTxWdly1R = crate::FieldReader<u16>;
#[doc = "Field `PRU1_ED_TX_WDLY1` writer - "]
pub type Pru1EdTxWdly1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PRU1_ED_TX_FRAME_SIZE1` reader - "]
pub type Pru1EdTxFrameSize1R = crate::FieldReader;
#[doc = "Field `PRU1_ED_TX_FRAME_SIZE1` writer - "]
pub type Pru1EdTxFrameSize1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_ED_RX_FRAME_SIZE1` reader - "]
pub type Pru1EdRxFrameSize1R = crate::FieldReader<u16>;
#[doc = "Field `PRU1_ED_RX_FRAME_SIZE1` writer - "]
pub type Pru1EdRxFrameSize1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PRU1_ED_RX_SNOOP1` reader - "]
pub type Pru1EdRxSnoop1R = crate::BitReader;
#[doc = "Field `PRU1_ED_RX_SNOOP1` writer - "]
pub type Pru1EdRxSnoop1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_CLK_OUT_OVR_EN1` reader - "]
pub type Pru1EdClkOutOvrEn1R = crate::BitReader;
#[doc = "Field `PRU1_ED_CLK_OUT_OVR_EN1` writer - "]
pub type Pru1EdClkOutOvrEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_SW_CLK_OUT1` reader - "]
pub type Pru1EdSwClkOut1R = crate::BitReader;
#[doc = "Field `PRU1_ED_SW_CLK_OUT1` writer - "]
pub type Pru1EdSwClkOut1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_TX_FIFO_SWAP_BITS1` reader - "]
pub type Pru1EdTxFifoSwapBits1R = crate::BitReader;
#[doc = "Field `PRU1_ED_TX_FIFO_SWAP_BITS1` writer - "]
pub type Pru1EdTxFifoSwapBits1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn pru1_ed_tx_wdly1(&self) -> Pru1EdTxWdly1R {
        Pru1EdTxWdly1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_ed_tx_frame_size1(&self) -> Pru1EdTxFrameSize1R {
        Pru1EdTxFrameSize1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn pru1_ed_rx_frame_size1(&self) -> Pru1EdRxFrameSize1R {
        Pru1EdRxFrameSize1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pru1_ed_rx_snoop1(&self) -> Pru1EdRxSnoop1R {
        Pru1EdRxSnoop1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pru1_ed_clk_out_ovr_en1(&self) -> Pru1EdClkOutOvrEn1R {
        Pru1EdClkOutOvrEn1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pru1_ed_sw_clk_out1(&self) -> Pru1EdSwClkOut1R {
        Pru1EdSwClkOut1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pru1_ed_tx_fifo_swap_bits1(&self) -> Pru1EdTxFifoSwapBits1R {
        Pru1EdTxFifoSwapBits1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tx_wdly1(&mut self) -> Pru1EdTxWdly1W<Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec> {
        Pru1EdTxWdly1W::new(self, 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tx_frame_size1(
        &mut self,
    ) -> Pru1EdTxFrameSize1W<Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec> {
        Pru1EdTxFrameSize1W::new(self, 11)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_frame_size1(
        &mut self,
    ) -> Pru1EdRxFrameSize1W<Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec> {
        Pru1EdRxFrameSize1W::new(self, 16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_snoop1(&mut self) -> Pru1EdRxSnoop1W<Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec> {
        Pru1EdRxSnoop1W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_clk_out_ovr_en1(
        &mut self,
    ) -> Pru1EdClkOutOvrEn1W<Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec> {
        Pru1EdClkOutOvrEn1W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_sw_clk_out1(&mut self) -> Pru1EdSwClkOut1W<Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec> {
        Pru1EdSwClkOut1W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tx_fifo_swap_bits1(
        &mut self,
    ) -> Pru1EdTxFifoSwapBits1W<Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec> {
        Pru1EdTxFifoSwapBits1W::new(self, 31)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg0_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec {
    const RESET_VALUE: u32 = 0;
}
