#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCHOUT` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgItchoutSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCHOUT` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgItchoutSpec>;
#[doc = "Field `CTCHOUT` reader - 3:0\\]
Set the value of the CTCHOUT outputs."]
pub type CtchoutR = crate::FieldReader;
#[doc = "Field `CTCHOUT` writer - 3:0\\]
Set the value of the CTCHOUT outputs."]
pub type CtchoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Set the value of the CTCHOUT outputs."]
    #[inline(always)]
    pub fn ctchout(&self) -> CtchoutR {
        CtchoutR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Set the value of the CTCHOUT outputs."]
    #[inline(always)]
    #[must_use]
    pub fn ctchout(&mut self) -> CtchoutW<Cti_Cfg_CsctiCfgItchoutSpec> {
        CtchoutW::new(self, 0)
    }
}
#[doc = "This register is a write-only register. It can be used to set the value of the CTCHOUT outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itchout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itchout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgItchoutSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgItchoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_itchout::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgItchoutSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_itchout::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgItchoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_ITCHOUT to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgItchoutSpec {
    const RESET_VALUE: u32 = 0;
}
