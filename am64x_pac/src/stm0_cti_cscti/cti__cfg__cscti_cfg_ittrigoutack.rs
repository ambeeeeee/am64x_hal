#[doc = "Register `CTI__CFG__CSCTI_CFG_ITTRIGOUTACK` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgIttrigoutackSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_ITTRIGOUTACK` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgIttrigoutackSpec>;
#[doc = "Field `CTTRIGOUTACK` reader - 7:0\\]
Read the value of the CTTRIGOUTACK inputs."]
pub type CttrigoutackR = crate::FieldReader;
#[doc = "Field `CTTRIGOUTACK` writer - 7:0\\]
Read the value of the CTTRIGOUTACK inputs."]
pub type CttrigoutackW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read the value of the CTTRIGOUTACK inputs."]
    #[inline(always)]
    pub fn cttrigoutack(&self) -> CttrigoutackR {
        CttrigoutackR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read the value of the CTTRIGOUTACK inputs."]
    #[inline(always)]
    #[must_use]
    pub fn cttrigoutack(&mut self) -> CttrigoutackW<Cti_Cfg_CsctiCfgIttrigoutackSpec> {
        CttrigoutackW::new(self, 0)
    }
}
#[doc = "This register is a read-only register. It can be used to read the values of the CTTRIGOUTACK inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ittrigoutack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ittrigoutack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgIttrigoutackSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgIttrigoutackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ittrigoutack::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgIttrigoutackSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ittrigoutack::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgIttrigoutackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_ITTRIGOUTACK to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgIttrigoutackSpec {
    const RESET_VALUE: u32 = 0;
}
