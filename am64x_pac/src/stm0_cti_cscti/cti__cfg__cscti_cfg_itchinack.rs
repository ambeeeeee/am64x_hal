#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCHINACK` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgItchinackSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCHINACK` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgItchinackSpec>;
#[doc = "Field `CTCHINACK` reader - 3:0\\]
Set the value of the CTCHINACK outputs."]
pub type CtchinackR = crate::FieldReader;
#[doc = "Field `CTCHINACK` writer - 3:0\\]
Set the value of the CTCHINACK outputs."]
pub type CtchinackW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Set the value of the CTCHINACK outputs."]
    #[inline(always)]
    pub fn ctchinack(&self) -> CtchinackR {
        CtchinackR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Set the value of the CTCHINACK outputs."]
    #[inline(always)]
    #[must_use]
    pub fn ctchinack(&mut self) -> CtchinackW<Cti_Cfg_CsctiCfgItchinackSpec> {
        CtchinackW::new(self, 0)
    }
}
#[doc = "This register is a write-only register. It can be used to set the value of the CTCHINACK outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itchinack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itchinack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgItchinackSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgItchinackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_itchinack::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgItchinackSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_itchinack::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgItchinackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_ITCHINACK to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgItchinackSpec {
    const RESET_VALUE: u32 = 0;
}
