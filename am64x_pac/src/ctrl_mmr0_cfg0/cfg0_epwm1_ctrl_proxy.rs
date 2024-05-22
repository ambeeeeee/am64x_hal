#[doc = "Register `CFG0_EPWM1_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Epwm1CtrlProxySpec>;
#[doc = "Register `CFG0_EPWM1_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Epwm1CtrlProxySpec>;
#[doc = "Field `EPWM1_CTRL_EALLOW_PROXY` reader - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm1CtrlEallowProxyR = crate::BitReader;
#[doc = "Field `EPWM1_CTRL_EALLOW_PROXY` writer - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm1CtrlEallowProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    pub fn epwm1_ctrl_eallow_proxy(&self) -> Epwm1CtrlEallowProxyR {
        Epwm1CtrlEallowProxyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    #[must_use]
    pub fn epwm1_ctrl_eallow_proxy(&mut self) -> Epwm1CtrlEallowProxyW<Cfg0Epwm1CtrlProxySpec> {
        Epwm1CtrlEallowProxyW::new(self, 4)
    }
}
#[doc = "CFG0_EPWM1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm1_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm1_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Epwm1CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Epwm1CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm1_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Epwm1CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm1_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Epwm1CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM1_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Epwm1CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
