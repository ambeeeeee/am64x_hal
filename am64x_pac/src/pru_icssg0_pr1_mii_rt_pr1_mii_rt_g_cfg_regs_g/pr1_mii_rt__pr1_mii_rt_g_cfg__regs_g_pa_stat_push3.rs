#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_pa_stat_push3` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_pa_stat_push3` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec>;
#[doc = "Field `PA_STAT_PUSH0_3` reader - 7:0\\]
pa stat push0"]
pub type PaStatPush0_3R = crate::FieldReader;
#[doc = "Field `PA_STAT_PUSH0_3` writer - 7:0\\]
pa stat push0"]
pub type PaStatPush0_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_STAT_PUSH1_3` reader - 15:8\\]
pa stat push1"]
pub type PaStatPush1_3R = crate::FieldReader;
#[doc = "Field `PA_STAT_PUSH1_3` writer - 15:8\\]
pa stat push1"]
pub type PaStatPush1_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_STAT_PUSH2_3` reader - 23:16\\]
pa stat push2"]
pub type PaStatPush2_3R = crate::FieldReader;
#[doc = "Field `PA_STAT_PUSH2_3` writer - 23:16\\]
pa stat push2"]
pub type PaStatPush2_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PA_STAT_PUSH3_3` reader - 31:24\\]
pa stat push3"]
pub type PaStatPush3_3R = crate::FieldReader;
#[doc = "Field `PA_STAT_PUSH3_3` writer - 31:24\\]
pa stat push3"]
pub type PaStatPush3_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
pa stat push0"]
    #[inline(always)]
    pub fn pa_stat_push0_3(&self) -> PaStatPush0_3R {
        PaStatPush0_3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
pa stat push1"]
    #[inline(always)]
    pub fn pa_stat_push1_3(&self) -> PaStatPush1_3R {
        PaStatPush1_3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
pa stat push2"]
    #[inline(always)]
    pub fn pa_stat_push2_3(&self) -> PaStatPush2_3R {
        PaStatPush2_3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
pa stat push3"]
    #[inline(always)]
    pub fn pa_stat_push3_3(&self) -> PaStatPush3_3R {
        PaStatPush3_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
pa stat push0"]
    #[inline(always)]
    #[must_use]
    pub fn pa_stat_push0_3(
        &mut self,
    ) -> PaStatPush0_3W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec> {
        PaStatPush0_3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
pa stat push1"]
    #[inline(always)]
    #[must_use]
    pub fn pa_stat_push1_3(
        &mut self,
    ) -> PaStatPush1_3W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec> {
        PaStatPush1_3W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
pa stat push2"]
    #[inline(always)]
    #[must_use]
    pub fn pa_stat_push2_3(
        &mut self,
    ) -> PaStatPush2_3W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec> {
        PaStatPush2_3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
pa stat push3"]
    #[inline(always)]
    #[must_use]
    pub fn pa_stat_push3_3(
        &mut self,
    ) -> PaStatPush3_3W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec> {
        PaStatPush3_3W::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_pa_stat_push3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_pa_stat_push3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_pa_stat_push3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_pa_stat_push3::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_pa_stat_push3::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_pa_stat_push3 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPaStatPush3Spec {
    const RESET_VALUE: u32 = 0;
}
