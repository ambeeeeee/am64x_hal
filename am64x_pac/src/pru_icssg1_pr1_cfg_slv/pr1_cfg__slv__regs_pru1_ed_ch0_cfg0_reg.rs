#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg0_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg0_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec>;
#[doc = "Field `PRU1_ED_TX_WDLY0` reader - "]
pub type Pru1EdTxWdly0R = crate::FieldReader<u16>;
#[doc = "Field `PRU1_ED_TX_WDLY0` writer - "]
pub type Pru1EdTxWdly0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PRU1_ED_TX_FRAME_SIZE0` reader - "]
pub type Pru1EdTxFrameSize0R = crate::FieldReader;
#[doc = "Field `PRU1_ED_TX_FRAME_SIZE0` writer - "]
pub type Pru1EdTxFrameSize0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_ED_RX_FRAME_SIZE0` reader - "]
pub type Pru1EdRxFrameSize0R = crate::FieldReader<u16>;
#[doc = "Field `PRU1_ED_RX_FRAME_SIZE0` writer - "]
pub type Pru1EdRxFrameSize0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PRU1_ED_RX_SNOOP0` reader - "]
pub type Pru1EdRxSnoop0R = crate::BitReader;
#[doc = "Field `PRU1_ED_RX_SNOOP0` writer - "]
pub type Pru1EdRxSnoop0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_CLK_OUT_OVR_EN0` reader - "]
pub type Pru1EdClkOutOvrEn0R = crate::BitReader;
#[doc = "Field `PRU1_ED_CLK_OUT_OVR_EN0` writer - "]
pub type Pru1EdClkOutOvrEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_SW_CLK_OUT0` reader - "]
pub type Pru1EdSwClkOut0R = crate::BitReader;
#[doc = "Field `PRU1_ED_SW_CLK_OUT0` writer - "]
pub type Pru1EdSwClkOut0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_TX_FIFO_SWAP_BITS0` reader - "]
pub type Pru1EdTxFifoSwapBits0R = crate::BitReader;
#[doc = "Field `PRU1_ED_TX_FIFO_SWAP_BITS0` writer - "]
pub type Pru1EdTxFifoSwapBits0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn pru1_ed_tx_wdly0(&self) -> Pru1EdTxWdly0R {
        Pru1EdTxWdly0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_ed_tx_frame_size0(&self) -> Pru1EdTxFrameSize0R {
        Pru1EdTxFrameSize0R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn pru1_ed_rx_frame_size0(&self) -> Pru1EdRxFrameSize0R {
        Pru1EdRxFrameSize0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pru1_ed_rx_snoop0(&self) -> Pru1EdRxSnoop0R {
        Pru1EdRxSnoop0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pru1_ed_clk_out_ovr_en0(&self) -> Pru1EdClkOutOvrEn0R {
        Pru1EdClkOutOvrEn0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pru1_ed_sw_clk_out0(&self) -> Pru1EdSwClkOut0R {
        Pru1EdSwClkOut0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pru1_ed_tx_fifo_swap_bits0(&self) -> Pru1EdTxFifoSwapBits0R {
        Pru1EdTxFifoSwapBits0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tx_wdly0(&mut self) -> Pru1EdTxWdly0W<Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec> {
        Pru1EdTxWdly0W::new(self, 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tx_frame_size0(
        &mut self,
    ) -> Pru1EdTxFrameSize0W<Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec> {
        Pru1EdTxFrameSize0W::new(self, 11)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_frame_size0(
        &mut self,
    ) -> Pru1EdRxFrameSize0W<Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec> {
        Pru1EdRxFrameSize0W::new(self, 16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_snoop0(&mut self) -> Pru1EdRxSnoop0W<Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec> {
        Pru1EdRxSnoop0W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_clk_out_ovr_en0(
        &mut self,
    ) -> Pru1EdClkOutOvrEn0W<Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec> {
        Pru1EdClkOutOvrEn0W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_sw_clk_out0(&mut self) -> Pru1EdSwClkOut0W<Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec> {
        Pru1EdSwClkOut0W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tx_fifo_swap_bits0(
        &mut self,
    ) -> Pru1EdTxFifoSwapBits0W<Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec> {
        Pru1EdTxFifoSwapBits0W::new(self, 31)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg0_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec {
    const RESET_VALUE: u32 = 0;
}
