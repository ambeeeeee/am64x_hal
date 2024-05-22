#[doc = "Register `PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg0` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg0` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec>;
#[doc = "Field `PA_PDSP0_INDEX` reader - 13:0\\]
pa_pdsp0_index"]
pub type PaPdsp0IndexR = crate::FieldReader<u16>;
#[doc = "Field `PA_PDSP0_INDEX` writer - 13:0\\]
pa_pdsp0_index"]
pub type PaPdsp0IndexW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PA_PDSP0_INC_VAL` reader - 30:14\\]
pa_pdsp0_inc_val"]
pub type PaPdsp0IncValR = crate::FieldReader<u32>;
#[doc = "Field `PA_PDSP0_INC_VAL` writer - 30:14\\]
pa_pdsp0_inc_val"]
pub type PaPdsp0IncValW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `PA_PDSP0_INC_TYPE` reader - 31:31\\]
pa_pdsp0_inc_type"]
pub type PaPdsp0IncTypeR = crate::BitReader;
#[doc = "Field `PA_PDSP0_INC_TYPE` writer - 31:31\\]
pa_pdsp0_inc_type"]
pub type PaPdsp0IncTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
pa_pdsp0_index"]
    #[inline(always)]
    pub fn pa_pdsp0_index(&self) -> PaPdsp0IndexR {
        PaPdsp0IndexR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:30 - 30:14\\]
pa_pdsp0_inc_val"]
    #[inline(always)]
    pub fn pa_pdsp0_inc_val(&self) -> PaPdsp0IncValR {
        PaPdsp0IncValR::new((self.bits >> 14) & 0x0001_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
pa_pdsp0_inc_type"]
    #[inline(always)]
    pub fn pa_pdsp0_inc_type(&self) -> PaPdsp0IncTypeR {
        PaPdsp0IncTypeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
pa_pdsp0_index"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pdsp0_index(&mut self) -> PaPdsp0IndexW<Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec> {
        PaPdsp0IndexW::new(self, 0)
    }
    #[doc = "Bits 14:30 - 30:14\\]
pa_pdsp0_inc_val"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pdsp0_inc_val(&mut self) -> PaPdsp0IncValW<Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec> {
        PaPdsp0IncValW::new(self, 14)
    }
    #[doc = "Bit 31 - 31:31\\]
pa_pdsp0_inc_type"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pdsp0_inc_type(&mut self) -> PaPdsp0IncTypeW<Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec> {
        PaPdsp0IncTypeW::new(self, 31)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg0::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg0::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg0 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec {
    const RESET_VALUE: u32 = 0;
}
