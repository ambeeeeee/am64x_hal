#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_RID_INDX_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgEccRidIndxRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_RID_INDX_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgEccRidIndxRegSpec>;
#[doc = "Field `ECCRID_ADR` reader - 5:0\\]
This index specifies the ECC cache entry number that the eccrid_val is mapped to."]
pub type EccridAdrR = crate::FieldReader;
#[doc = "Field `ECCRID_ADR` writer - 5:0\\]
This index specifies the ECC cache entry number that the eccrid_val is mapped to."]
pub type EccridAdrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
This index specifies the ECC cache entry number that the eccrid_val is mapped to."]
    #[inline(always)]
    pub fn eccrid_adr(&self) -> EccridAdrR {
        EccridAdrR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
This index specifies the ECC cache entry number that the eccrid_val is mapped to."]
    #[inline(always)]
    #[must_use]
    pub fn eccrid_adr(&mut self) -> EccridAdrW<Regs_SsCfg_SscfgEccRidIndxRegSpec> {
        EccridAdrW::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_ECC_RID_INDX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_rid_indx_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_rid_indx_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgEccRidIndxRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgEccRidIndxRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ecc_rid_indx_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgEccRidIndxRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ecc_rid_indx_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgEccRidIndxRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_ECC_RID_INDX_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgEccRidIndxRegSpec {
    const RESET_VALUE: u32 = 0;
}
