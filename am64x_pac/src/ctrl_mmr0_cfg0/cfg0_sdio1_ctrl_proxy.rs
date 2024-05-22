#[doc = "Register `CFG0_SDIO1_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Sdio1CtrlProxySpec>;
#[doc = "Register `CFG0_SDIO1_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Sdio1CtrlProxySpec>;
#[doc = "Field `SDIO1_CTRL_DRV_STR_PROXY` reader - 4:0\\]
Selects the SDIO drive strength"]
pub type Sdio1CtrlDrvStrProxyR = crate::FieldReader;
#[doc = "Field `SDIO1_CTRL_DRV_STR_PROXY` writer - 4:0\\]
Selects the SDIO drive strength"]
pub type Sdio1CtrlDrvStrProxyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the SDIO drive strength"]
    #[inline(always)]
    pub fn sdio1_ctrl_drv_str_proxy(&self) -> Sdio1CtrlDrvStrProxyR {
        Sdio1CtrlDrvStrProxyR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the SDIO drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_ctrl_drv_str_proxy(&mut self) -> Sdio1CtrlDrvStrProxyW<Cfg0Sdio1CtrlProxySpec> {
        Sdio1CtrlDrvStrProxyW::new(self, 0)
    }
}
#[doc = "CFG0_SDIO1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_sdio1_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_sdio1_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Sdio1CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Sdio1CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_sdio1_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Sdio1CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_sdio1_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Sdio1CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_SDIO1_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Sdio1CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
