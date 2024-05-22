#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCHIN` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgItchinSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCHIN` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgItchinSpec>;
#[doc = "Field `CTCHIN` reader - 3:0\\]
Read the value of the CTCHIN inputs."]
pub type CtchinR = crate::FieldReader;
#[doc = "Field `CTCHIN` writer - 3:0\\]
Read the value of the CTCHIN inputs."]
pub type CtchinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Read the value of the CTCHIN inputs."]
    #[inline(always)]
    pub fn ctchin(&self) -> CtchinR {
        CtchinR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Read the value of the CTCHIN inputs."]
    #[inline(always)]
    #[must_use]
    pub fn ctchin(&mut self) -> CtchinW<Cti_Cfg_CsctiCfgItchinSpec> {
        CtchinW::new(self, 0)
    }
}
#[doc = "This register is a read-only register. It can be used to read the values of the CTCHIN inputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itchin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itchin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgItchinSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgItchinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_itchin::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgItchinSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_itchin::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgItchinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_ITCHIN to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgItchinSpec {
    const RESET_VALUE: u32 = 0;
}
