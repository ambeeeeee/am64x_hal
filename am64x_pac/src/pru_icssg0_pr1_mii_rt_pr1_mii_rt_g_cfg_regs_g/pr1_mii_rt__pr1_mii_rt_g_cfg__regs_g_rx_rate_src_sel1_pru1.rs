#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_rate_src_sel1_pru1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_rate_src_sel1_pru1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec>;
#[doc = "Field `RX_RATE_SRC_SEL4` reader - 5:0\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel4R = crate::FieldReader;
#[doc = "Field `RX_RATE_SRC_SEL4` writer - 5:0\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_RATE_SRC_SEL5` reader - 13:8\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel5R = crate::FieldReader;
#[doc = "Field `RX_RATE_SRC_SEL5` writer - 13:8\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_RATE_SRC_SEL6` reader - 21:16\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel6R = crate::FieldReader;
#[doc = "Field `RX_RATE_SRC_SEL6` writer - 21:16\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_RATE_SRC_SEL7` reader - 29:24\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel7R = crate::FieldReader;
#[doc = "Field `RX_RATE_SRC_SEL7` writer - 29:24\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
pub type RxRateSrcSel7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    pub fn rx_rate_src_sel4(&self) -> RxRateSrcSel4R {
        RxRateSrcSel4R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    pub fn rx_rate_src_sel5(&self) -> RxRateSrcSel5R {
        RxRateSrcSel5R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    pub fn rx_rate_src_sel6(&self) -> RxRateSrcSel6R {
        RxRateSrcSel6R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    pub fn rx_rate_src_sel7(&self) -> RxRateSrcSel7R {
        RxRateSrcSel7R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_src_sel4(
        &mut self,
    ) -> RxRateSrcSel4W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec> {
        RxRateSrcSel4W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_src_sel5(
        &mut self,
    ) -> RxRateSrcSel5W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec> {
        RxRateSrcSel5W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_src_sel6(
        &mut self,
    ) -> RxRateSrcSel6W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec> {
        RxRateSrcSel6W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Map which filter/flag/class hit that rate logic uses, see table for mapping"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rate_src_sel7(
        &mut self,
    ) -> RxRateSrcSel7W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec> {
        RxRateSrcSel7W::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_rate_src_sel1_pru1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_rate_src_sel1_pru1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_rate_src_sel1_pru1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_rate_src_sel1_pru1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_rate_src_sel1_pru1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_rate_src_sel1_pru1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxRateSrcSel1Pru1Spec {
    const RESET_VALUE: u32 = 0;
}
