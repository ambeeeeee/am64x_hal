#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec>;
#[doc = "Field `RX_ENABLE1` reader - "]
pub type RxEnable1R = crate::BitReader;
#[doc = "Field `RX_ENABLE1` writer - "]
pub type RxEnable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_RDY_MODE_DIS1` reader - "]
pub type RxDataRdyModeDis1R = crate::BitReader;
#[doc = "Field `RX_DATA_RDY_MODE_DIS1` writer - "]
pub type RxDataRdyModeDis1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CUT_PREAMBLE1` reader - "]
pub type RxCutPreamble1R = crate::BitReader;
#[doc = "Field `RX_CUT_PREAMBLE1` writer - "]
pub type RxCutPreamble1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MUX_SEL1` reader - "]
pub type RxMuxSel1R = crate::BitReader;
#[doc = "Field `RX_MUX_SEL1` writer - "]
pub type RxMuxSel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_L2_EN1` reader - "]
pub type RxL2En1R = crate::BitReader;
#[doc = "Field `RX_L2_EN1` writer - "]
pub type RxL2En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BYTE_SWAP1` reader - "]
pub type RxByteSwap1R = crate::BitReader;
#[doc = "Field `RX_BYTE_SWAP1` writer - "]
pub type RxByteSwap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_AUTO_FWD_PRE1` reader - "]
pub type RxAutoFwdPre1R = crate::BitReader;
#[doc = "Field `RX_AUTO_FWD_PRE1` writer - "]
pub type RxAutoFwdPre1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SFD_RAW1` reader - "]
pub type RxSfdRaw1R = crate::BitReader;
#[doc = "Field `RX_SFD_RAW1` writer - "]
pub type RxSfdRaw1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ERR_RAW1` reader - "]
pub type RxErrRaw1R = crate::BitReader;
#[doc = "Field `RX_ERR_RAW1` writer - "]
pub type RxErrRaw1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EOF_SCLR_DIS1` reader - "]
pub type RxEofSclrDis1R = crate::BitReader;
#[doc = "Field `RX_EOF_SCLR_DIS1` writer - "]
pub type RxEofSclrDis1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_enable1(&self) -> RxEnable1R {
        RxEnable1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_data_rdy_mode_dis1(&self) -> RxDataRdyModeDis1R {
        RxDataRdyModeDis1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_cut_preamble1(&self) -> RxCutPreamble1R {
        RxCutPreamble1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_mux_sel1(&self) -> RxMuxSel1R {
        RxMuxSel1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_l2_en1(&self) -> RxL2En1R {
        RxL2En1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_byte_swap1(&self) -> RxByteSwap1R {
        RxByteSwap1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_auto_fwd_pre1(&self) -> RxAutoFwdPre1R {
        RxAutoFwdPre1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_sfd_raw1(&self) -> RxSfdRaw1R {
        RxSfdRaw1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rx_err_raw1(&self) -> RxErrRaw1R {
        RxErrRaw1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_eof_sclr_dis1(&self) -> RxEofSclrDis1R {
        RxEofSclrDis1R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_enable1(&mut self) -> RxEnable1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxEnable1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_rdy_mode_dis1(
        &mut self,
    ) -> RxDataRdyModeDis1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxDataRdyModeDis1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_cut_preamble1(&mut self) -> RxCutPreamble1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxCutPreamble1W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mux_sel1(&mut self) -> RxMuxSel1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxMuxSel1W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rx_l2_en1(&mut self) -> RxL2En1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxL2En1W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_byte_swap1(&mut self) -> RxByteSwap1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxByteSwap1W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rx_auto_fwd_pre1(&mut self) -> RxAutoFwdPre1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxAutoFwdPre1W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sfd_raw1(&mut self) -> RxSfdRaw1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxSfdRaw1W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rx_err_raw1(&mut self) -> RxErrRaw1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxErrRaw1W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_eof_sclr_dis1(&mut self) -> RxEofSclrDis1W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec> {
        RxEofSclrDis1W::new(self, 9)
    }
}
#[doc = "MIIRXCFG1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg1 to value 0x08"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec {
    const RESET_VALUE: u32 = 0x08;
}
