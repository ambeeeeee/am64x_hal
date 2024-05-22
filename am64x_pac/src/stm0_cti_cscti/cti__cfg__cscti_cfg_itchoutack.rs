#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCHOUTACK` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgItchoutackSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCHOUTACK` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgItchoutackSpec>;
#[doc = "Field `CTCHOUTACK` reader - 3:0\\]
Read the values of the CTCHOUTACK inputs."]
pub type CtchoutackR = crate::FieldReader;
#[doc = "Field `CTCHOUTACK` writer - 3:0\\]
Read the values of the CTCHOUTACK inputs."]
pub type CtchoutackW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Read the values of the CTCHOUTACK inputs."]
    #[inline(always)]
    pub fn ctchoutack(&self) -> CtchoutackR {
        CtchoutackR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Read the values of the CTCHOUTACK inputs."]
    #[inline(always)]
    #[must_use]
    pub fn ctchoutack(&mut self) -> CtchoutackW<Cti_Cfg_CsctiCfgItchoutackSpec> {
        CtchoutackW::new(self, 0)
    }
}
#[doc = "This register is a read-only register. It can be used to read the values of the CTCHOUTACK inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itchoutack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itchoutack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgItchoutackSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgItchoutackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_itchoutack::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgItchoutackSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_itchoutack::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgItchoutackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_ITCHOUTACK to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgItchoutackSpec {
    const RESET_VALUE: u32 = 0;
}
