#[doc = "Register `CFG0_EPWM0_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Epwm0CtrlProxySpec>;
#[doc = "Register `CFG0_EPWM0_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Epwm0CtrlProxySpec>;
#[doc = "Field `EPWM0_CTRL_EALLOW_PROXY` reader - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm0CtrlEallowProxyR = crate::BitReader;
#[doc = "Field `EPWM0_CTRL_EALLOW_PROXY` writer - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm0CtrlEallowProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM0_CTRL_SYNCIN_SEL_PROXY` reader - 10:8\\]
Selects the source of the EPWM0 synchronization input"]
pub type Epwm0CtrlSyncinSelProxyR = crate::FieldReader;
#[doc = "Field `EPWM0_CTRL_SYNCIN_SEL_PROXY` writer - 10:8\\]
Selects the source of the EPWM0 synchronization input"]
pub type Epwm0CtrlSyncinSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    pub fn epwm0_ctrl_eallow_proxy(&self) -> Epwm0CtrlEallowProxyR {
        Epwm0CtrlEallowProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the source of the EPWM0 synchronization input"]
    #[inline(always)]
    pub fn epwm0_ctrl_syncin_sel_proxy(&self) -> Epwm0CtrlSyncinSelProxyR {
        Epwm0CtrlSyncinSelProxyR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    #[must_use]
    pub fn epwm0_ctrl_eallow_proxy(&mut self) -> Epwm0CtrlEallowProxyW<Cfg0Epwm0CtrlProxySpec> {
        Epwm0CtrlEallowProxyW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the source of the EPWM0 synchronization input"]
    #[inline(always)]
    #[must_use]
    pub fn epwm0_ctrl_syncin_sel_proxy(
        &mut self,
    ) -> Epwm0CtrlSyncinSelProxyW<Cfg0Epwm0CtrlProxySpec> {
        Epwm0CtrlSyncinSelProxyW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm0_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm0_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Epwm0CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Epwm0CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm0_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Epwm0CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm0_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Epwm0CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM0_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Epwm0CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
