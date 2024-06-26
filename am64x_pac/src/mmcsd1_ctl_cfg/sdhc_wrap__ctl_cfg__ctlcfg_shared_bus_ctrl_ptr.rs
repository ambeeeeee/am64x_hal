#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_shared_bus_ctrl_ptr` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtrSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_shared_bus_ctrl_ptr` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtrSpec>;
#[doc = "Field `SHARED_BUS_CTRL_PTR` reader - 15:0\\]
Pointer for Shared Bus Control Register"]
pub type SharedBusCtrlPtrR = crate::FieldReader<u16>;
#[doc = "Field `SHARED_BUS_CTRL_PTR` writer - 15:0\\]
Pointer for Shared Bus Control Register"]
pub type SharedBusCtrlPtrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Pointer for Shared Bus Control Register"]
    #[inline(always)]
    pub fn shared_bus_ctrl_ptr(&self) -> SharedBusCtrlPtrR {
        SharedBusCtrlPtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Pointer for Shared Bus Control Register"]
    #[inline(always)]
    #[must_use]
    pub fn shared_bus_ctrl_ptr(
        &mut self,
    ) -> SharedBusCtrlPtrW<SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtrSpec> {
        SharedBusCtrlPtrW::new(self, 0)
    }
}
#[doc = "This register is pointer for UHS-II Shared Bus Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtrSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_shared_bus_ctrl_ptr to value 0x0304"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtrSpec {
    const RESET_VALUE: u16 = 0x0304;
}
