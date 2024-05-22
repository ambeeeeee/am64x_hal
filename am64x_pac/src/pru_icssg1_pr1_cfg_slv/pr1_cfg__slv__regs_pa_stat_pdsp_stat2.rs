#[doc = "Register `PR1_CFG__SLV__REGS_pa_stat_pdsp_stat2` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPaStatPdspStat2Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pa_stat_pdsp_stat2` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPaStatPdspStat2Spec>;
#[doc = "Field `PA_PDSP2_READY` reader - 0:0\\]
pa_pdsp2_ready"]
pub type PaPdsp2ReadyR = crate::BitReader;
#[doc = "Field `PA_PDSP2_READY` writer - 0:0\\]
pa_pdsp2_ready"]
pub type PaPdsp2ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA_PDSP2_STATUS` reader - 3:1\\]
pa_pdsp2_status"]
pub type PaPdsp2StatusR = crate::FieldReader;
#[doc = "Field `PA_PDSP2_STATUS` writer - 3:1\\]
pa_pdsp2_status"]
pub type PaPdsp2StatusW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
pa_pdsp2_ready"]
    #[inline(always)]
    pub fn pa_pdsp2_ready(&self) -> PaPdsp2ReadyR {
        PaPdsp2ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
pa_pdsp2_status"]
    #[inline(always)]
    pub fn pa_pdsp2_status(&self) -> PaPdsp2StatusR {
        PaPdsp2StatusR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
pa_pdsp2_ready"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pdsp2_ready(&mut self) -> PaPdsp2ReadyW<Pr1Cfg_Slv_RegsPaStatPdspStat2Spec> {
        PaPdsp2ReadyW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
pa_pdsp2_status"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pdsp2_status(&mut self) -> PaPdsp2StatusW<Pr1Cfg_Slv_RegsPaStatPdspStat2Spec> {
        PaPdsp2StatusW::new(self, 1)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_stat2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_stat2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPaStatPdspStat2Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPaStatPdspStat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pa_stat_pdsp_stat2::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPaStatPdspStat2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pa_stat_pdsp_stat2::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPaStatPdspStat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pa_stat_pdsp_stat2 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPaStatPdspStat2Spec {
    const RESET_VALUE: u32 = 0;
}
