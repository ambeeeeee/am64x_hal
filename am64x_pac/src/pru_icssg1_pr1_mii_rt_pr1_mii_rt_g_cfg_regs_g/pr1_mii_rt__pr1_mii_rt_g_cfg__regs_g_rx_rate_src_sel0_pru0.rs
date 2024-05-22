#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_rate_src_sel0_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_rate_src_sel0_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec>;
#[doc = "Field `RX_RATE_SRC_SEL0` reader - 5:0\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel0R = crate::FieldReader;
#[doc = "Field `RX_RATE_SRC_SEL0` writer - 5:0\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_RATE_SRC_SEL1` reader - 13:8\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel1R = crate::FieldReader;
#[doc = "Field `RX_RATE_SRC_SEL1` writer - 13:8\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_RATE_SRC_SEL2` reader - 21:16\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel2R = crate::FieldReader;
#[doc = "Field `RX_RATE_SRC_SEL2` writer - 21:16\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_RATE_SRC_SEL3` reader - 29:24\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel3R = crate::FieldReader;
#[doc = "Field `RX_RATE_SRC_SEL3` writer - 29:24\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    pub fn rx_rate_src_sel0(&self) -> RxRateSrcSel0R {
        RxRateSrcSel0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    pub fn rx_rate_src_sel1(&self) -> RxRateSrcSel1R {
        RxRateSrcSel1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    pub fn rx_rate_src_sel2(&self) -> RxRateSrcSel2R {
        RxRateSrcSel2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    pub fn rx_rate_src_sel3(&self) -> RxRateSrcSel3R {
        RxRateSrcSel3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_src_sel0(
        &mut self,
    ) -> RxRateSrcSel0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec> {
        RxRateSrcSel0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_src_sel1(
        &mut self,
    ) -> RxRateSrcSel1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec> {
        RxRateSrcSel1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_src_sel2(
        &mut self,
    ) -> RxRateSrcSel2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec> {
        RxRateSrcSel2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_src_sel3(
        &mut self,
    ) -> RxRateSrcSel3W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec> {
        RxRateSrcSel3W::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_rate_src_sel0_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_rate_src_sel0_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_rate_src_sel0_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_rate_src_sel0_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_rate_src_sel0_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_rate_src_sel0_pru0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel0Pru0Spec {
    const RESET_VALUE: u32 = 0;
}
