#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_pa_stat_push1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_pa_stat_push1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec>;
#[doc = "Field `PA_STAT_PUSH0_1` reader - 7:0\\]
pa stat push0"]
pub type PaStatPush0_1R = crate::FieldReader;
#[doc = "Field `PA_STAT_PUSH0_1` writer - 7:0\\]
pa stat push0"]
pub type PaStatPush0_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_STAT_PUSH1_1` reader - 15:8\\]
pa stat push1"]
pub type PaStatPush1_1R = crate::FieldReader;
#[doc = "Field `PA_STAT_PUSH1_1` writer - 15:8\\]
pa stat push1"]
pub type PaStatPush1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_STAT_PUSH2_1` reader - 23:16\\]
pa stat push2"]
pub type PaStatPush2_1R = crate::FieldReader;
#[doc = "Field `PA_STAT_PUSH2_1` writer - 23:16\\]
pa stat push2"]
pub type PaStatPush2_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_STAT_PUSH3_1` reader - 31:24\\]
pa stat push3"]
pub type PaStatPush3_1R = crate::FieldReader;
#[doc = "Field `PA_STAT_PUSH3_1` writer - 31:24\\]
pa stat push3"]
pub type PaStatPush3_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
pa stat push0"]
    #[inline(always)]
    pub fn pa_stat_push0_1(&self) -> PaStatPush0_1R {
        PaStatPush0_1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
pa stat push1"]
    #[inline(always)]
    pub fn pa_stat_push1_1(&self) -> PaStatPush1_1R {
        PaStatPush1_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
pa stat push2"]
    #[inline(always)]
    pub fn pa_stat_push2_1(&self) -> PaStatPush2_1R {
        PaStatPush2_1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
pa stat push3"]
    #[inline(always)]
    pub fn pa_stat_push3_1(&self) -> PaStatPush3_1R {
        PaStatPush3_1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
pa stat push0"]
    #[inline(always)]
    #[must_use]
    pub fn pa_stat_push0_1(
        &mut self,
    ) -> PaStatPush0_1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec> {
        PaStatPush0_1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
pa stat push1"]
    #[inline(always)]
    #[must_use]
    pub fn pa_stat_push1_1(
        &mut self,
    ) -> PaStatPush1_1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec> {
        PaStatPush1_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
pa stat push2"]
    #[inline(always)]
    #[must_use]
    pub fn pa_stat_push2_1(
        &mut self,
    ) -> PaStatPush2_1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec> {
        PaStatPush2_1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
pa stat push3"]
    #[inline(always)]
    #[must_use]
    pub fn pa_stat_push3_1(
        &mut self,
    ) -> PaStatPush3_1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec> {
        PaStatPush3_1W::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_pa_stat_push1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_pa_stat_push1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_pa_stat_push1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_pa_stat_push1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_pa_stat_push1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_pa_stat_push1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush1Spec {
    const RESET_VALUE: u32 = 0;
}
