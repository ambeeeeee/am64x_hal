#[doc = "Register `PR1_CFG__SLV__REGS_pa_stat_pdsp_stat0` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPaStatPdspStat0Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pa_stat_pdsp_stat0` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPaStatPdspStat0Spec>;
#[doc = "Field `PA_PDSP0_READY` reader - 0:0\\]
pa_pdsp0_ready"]
pub type PaPdsp0ReadyR = crate::BitReader;
#[doc = "Field `PA_PDSP0_READY` writer - 0:0\\]
pa_pdsp0_ready"]
pub type PaPdsp0ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA_PDSP0_STATUS` reader - 3:1\\]
pa_pdsp0_status"]
pub type PaPdsp0StatusR = crate::FieldReader;
#[doc = "Field `PA_PDSP0_STATUS` writer - 3:1\\]
pa_pdsp0_status"]
pub type PaPdsp0StatusW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
pa_pdsp0_ready"]
    #[inline(always)]
    pub fn pa_pdsp0_ready(&self) -> PaPdsp0ReadyR {
        PaPdsp0ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
pa_pdsp0_status"]
    #[inline(always)]
    pub fn pa_pdsp0_status(&self) -> PaPdsp0StatusR {
        PaPdsp0StatusR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
pa_pdsp0_ready"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pdsp0_ready(&mut self) -> PaPdsp0ReadyW<Pr1Cfg_Slv_RegsPaStatPdspStat0Spec> {
        PaPdsp0ReadyW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
pa_pdsp0_status"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pdsp0_status(&mut self) -> PaPdsp0StatusW<Pr1Cfg_Slv_RegsPaStatPdspStat0Spec> {
        PaPdsp0StatusW::new(self, 1)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPaStatPdspStat0Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPaStatPdspStat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pa_stat_pdsp_stat0::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPaStatPdspStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pa_stat_pdsp_stat0::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPaStatPdspStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pa_stat_pdsp_stat0 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPaStatPdspStat0Spec {
    const RESET_VALUE: u32 = 0;
}
