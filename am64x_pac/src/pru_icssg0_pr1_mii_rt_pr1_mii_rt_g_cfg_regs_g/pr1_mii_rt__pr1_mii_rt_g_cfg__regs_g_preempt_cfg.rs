#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_preempt_cfg` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_preempt_cfg` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec>;
#[doc = "Field `EXP_SMD` reader - 15:8\\]
None preemptable frame start, or express frame"]
pub type ExpSmdR = crate::FieldReader;
#[doc = "Field `EXP_SMD` writer - 15:8\\]
None preemptable frame start, or express frame"]
pub type ExpSmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMD_V` reader - 23:16\\]
Verification frame TAG"]
pub type SmdVR = crate::FieldReader;
#[doc = "Field `SMD_V` writer - 23:16\\]
Verification frame TAG"]
pub type SmdVW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMD_R` reader - 31:24\\]
Response frame TAG"]
pub type SmdRR = crate::FieldReader;
#[doc = "Field `SMD_R` writer - 31:24\\]
Response frame TAG"]
pub type SmdRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
None preemptable frame start, or express frame"]
    #[inline(always)]
    pub fn exp_smd(&self) -> ExpSmdR {
        ExpSmdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Verification frame TAG"]
    #[inline(always)]
    pub fn smd_v(&self) -> SmdVR {
        SmdVR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Response frame TAG"]
    #[inline(always)]
    pub fn smd_r(&self) -> SmdRR {
        SmdRR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
None preemptable frame start, or express frame"]
    #[inline(always)]
    #[must_use]
    pub fn exp_smd(&mut self) -> ExpSmdW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec> {
        ExpSmdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Verification frame TAG"]
    #[inline(always)]
    #[must_use]
    pub fn smd_v(&mut self) -> SmdVW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec> {
        SmdVW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Response frame TAG"]
    #[inline(always)]
    #[must_use]
    pub fn smd_r(&mut self) -> SmdRW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec> {
        SmdRW::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_preempt_cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_preempt_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_preempt_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_preempt_cfg::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_preempt_cfg::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_preempt_cfg to value 0x2507_1300"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGPreemptCfgSpec {
    const RESET_VALUE: u32 = 0x2507_1300;
}
