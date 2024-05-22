#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCTRL` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgItctrlSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_ITCTRL` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgItctrlSpec>;
#[doc = "Field `INTEGRATION_MODE` reader - 0:0\\]
Allows the component to switch from functional mode to integration mode or back."]
pub type IntegrationModeR = crate::BitReader;
#[doc = "Field `INTEGRATION_MODE` writer - 0:0\\]
Allows the component to switch from functional mode to integration mode or back."]
pub type IntegrationModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Allows the component to switch from functional mode to integration mode or back."]
    #[inline(always)]
    pub fn integration_mode(&self) -> IntegrationModeR {
        IntegrationModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Allows the component to switch from functional mode to integration mode or back."]
    #[inline(always)]
    #[must_use]
    pub fn integration_mode(&mut self) -> IntegrationModeW<Cti_Cfg_CsctiCfgItctrlSpec> {
        IntegrationModeW::new(self, 0)
    }
}
#[doc = "This register is used to enable topology detection. For more information see the CoreSight Architecture Specification. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purpose of integration testing and topology solving. Note : When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_itctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_itctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgItctrlSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgItctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_itctrl::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgItctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_itctrl::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgItctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_ITCTRL to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgItctrlSpec {
    const RESET_VALUE: u32 = 0;
}
