#[doc = "Register `CTI__CFG__CSCTI_CFG_CTICONTROL` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCticontrolSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTICONTROL` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCticontrolSpec>;
#[doc = "Field `GLBEN` reader - 0:0\\]
Enables or disables the ECT."]
pub type GlbenR = crate::BitReader;
#[doc = "Field `GLBEN` writer - 0:0\\]
Enables or disables the ECT."]
pub type GlbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables or disables the ECT."]
    #[inline(always)]
    pub fn glben(&self) -> GlbenR {
        GlbenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables or disables the ECT."]
    #[inline(always)]
    #[must_use]
    pub fn glben(&mut self) -> GlbenW<Cti_Cfg_CsctiCfgCticontrolSpec> {
        GlbenW::new(self, 0)
    }
}
#[doc = "The CTI Control Register enables the CTI. >\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_cticontrol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_cticontrol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCticontrolSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCticontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_cticontrol::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCticontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_cticontrol::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCticontrolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTICONTROL to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCticontrolSpec {
    const RESET_VALUE: u32 = 0;
}
