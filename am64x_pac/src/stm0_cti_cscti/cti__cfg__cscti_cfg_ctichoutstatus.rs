#[doc = "Register `CTI__CFG__CSCTI_CFG_CTICHOUTSTATUS` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtichoutstatusSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTICHOUTSTATUS` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtichoutstatusSpec>;
#[doc = "Field `CTICHOUTSTATUS` reader - 3:0\\]
Shows the status of the ctichout outputs. 1 = ctichout is active. 0 = ctichout is inactive. There is one bit of the field for each channel output."]
pub type CtichoutstatusR = crate::FieldReader;
#[doc = "Field `CTICHOUTSTATUS` writer - 3:0\\]
Shows the status of the ctichout outputs. 1 = ctichout is active. 0 = ctichout is inactive. There is one bit of the field for each channel output."]
pub type CtichoutstatusW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Shows the status of the ctichout outputs. 1 = ctichout is active. 0 = ctichout is inactive. There is one bit of the field for each channel output."]
    #[inline(always)]
    pub fn ctichoutstatus(&self) -> CtichoutstatusR {
        CtichoutstatusR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Shows the status of the ctichout outputs. 1 = ctichout is active. 0 = ctichout is inactive. There is one bit of the field for each channel output."]
    #[inline(always)]
    #[must_use]
    pub fn ctichoutstatus(&mut self) -> CtichoutstatusW<Cti_Cfg_CsctiCfgCtichoutstatusSpec> {
        CtichoutstatusW::new(self, 0)
    }
}
#[doc = "The CTI Channel Out Status Register provides the status of the CTI ctichout outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctichoutstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctichoutstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtichoutstatusSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtichoutstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctichoutstatus::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtichoutstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctichoutstatus::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtichoutstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTICHOUTSTATUS to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtichoutstatusSpec {
    const RESET_VALUE: u32 = 0;
}
