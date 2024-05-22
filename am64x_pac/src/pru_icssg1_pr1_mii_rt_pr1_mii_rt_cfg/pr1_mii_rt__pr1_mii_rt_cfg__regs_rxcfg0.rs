#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec>;
#[doc = "Field `RX_ENABLE0` reader - "]
pub type RxEnable0R = crate::BitReader;
#[doc = "Field `RX_ENABLE0` writer - "]
pub type RxEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_RDY_MODE_DIS0` reader - "]
pub type RxDataRdyModeDis0R = crate::BitReader;
#[doc = "Field `RX_DATA_RDY_MODE_DIS0` writer - "]
pub type RxDataRdyModeDis0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CUT_PREAMBLE0` reader - "]
pub type RxCutPreamble0R = crate::BitReader;
#[doc = "Field `RX_CUT_PREAMBLE0` writer - "]
pub type RxCutPreamble0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MUX_SEL0` reader - "]
pub type RxMuxSel0R = crate::BitReader;
#[doc = "Field `RX_MUX_SEL0` writer - "]
pub type RxMuxSel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_L2_EN0` reader - "]
pub type RxL2En0R = crate::BitReader;
#[doc = "Field `RX_L2_EN0` writer - "]
pub type RxL2En0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BYTE_SWAP0` reader - "]
pub type RxByteSwap0R = crate::BitReader;
#[doc = "Field `RX_BYTE_SWAP0` writer - "]
pub type RxByteSwap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_AUTO_FWD_PRE0` reader - "]
pub type RxAutoFwdPre0R = crate::BitReader;
#[doc = "Field `RX_AUTO_FWD_PRE0` writer - "]
pub type RxAutoFwdPre0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SFD_RAW0` reader - "]
pub type RxSfdRaw0R = crate::BitReader;
#[doc = "Field `RX_SFD_RAW0` writer - "]
pub type RxSfdRaw0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ERR_RAW0` reader - "]
pub type RxErrRaw0R = crate::BitReader;
#[doc = "Field `RX_ERR_RAW0` writer - "]
pub type RxErrRaw0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EOF_SCLR_DIS0` reader - "]
pub type RxEofSclrDis0R = crate::BitReader;
#[doc = "Field `RX_EOF_SCLR_DIS0` writer - "]
pub type RxEofSclrDis0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_enable0(&self) -> RxEnable0R {
        RxEnable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_data_rdy_mode_dis0(&self) -> RxDataRdyModeDis0R {
        RxDataRdyModeDis0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_cut_preamble0(&self) -> RxCutPreamble0R {
        RxCutPreamble0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_mux_sel0(&self) -> RxMuxSel0R {
        RxMuxSel0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_l2_en0(&self) -> RxL2En0R {
        RxL2En0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_byte_swap0(&self) -> RxByteSwap0R {
        RxByteSwap0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_auto_fwd_pre0(&self) -> RxAutoFwdPre0R {
        RxAutoFwdPre0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_sfd_raw0(&self) -> RxSfdRaw0R {
        RxSfdRaw0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rx_err_raw0(&self) -> RxErrRaw0R {
        RxErrRaw0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_eof_sclr_dis0(&self) -> RxEofSclrDis0R {
        RxEofSclrDis0R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_enable0(&mut self) -> RxEnable0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxEnable0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_rdy_mode_dis0(
        &mut self,
    ) -> RxDataRdyModeDis0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxDataRdyModeDis0W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_cut_preamble0(&mut self) -> RxCutPreamble0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxCutPreamble0W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mux_sel0(&mut self) -> RxMuxSel0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxMuxSel0W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rx_l2_en0(&mut self) -> RxL2En0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxL2En0W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_byte_swap0(&mut self) -> RxByteSwap0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxByteSwap0W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rx_auto_fwd_pre0(&mut self) -> RxAutoFwdPre0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxAutoFwdPre0W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sfd_raw0(&mut self) -> RxSfdRaw0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxSfdRaw0W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rx_err_raw0(&mut self) -> RxErrRaw0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxErrRaw0W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_eof_sclr_dis0(&mut self) -> RxEofSclrDis0W<Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec> {
        RxEofSclrDis0W::new(self, 9)
    }
}
#[doc = "MIIRXCFG0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec {
    const RESET_VALUE: u32 = 0;
}
