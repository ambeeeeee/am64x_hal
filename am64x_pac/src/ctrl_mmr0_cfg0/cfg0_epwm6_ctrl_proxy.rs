#[doc = "Register `CFG0_EPWM6_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Epwm6CtrlProxySpec>;
#[doc = "Register `CFG0_EPWM6_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Epwm6CtrlProxySpec>;
#[doc = "Field `EPWM6_CTRL_EALLOW_PROXY` reader - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm6CtrlEallowProxyR = crate::BitReader;
#[doc = "Field `EPWM6_CTRL_EALLOW_PROXY` writer - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm6CtrlEallowProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM6_CTRL_SYNCIN_SEL_PROXY` reader - 10:8\\]
Selects the source of the EPWM6 synchronization input"]
pub type Epwm6CtrlSyncinSelProxyR = crate::FieldReader;
#[doc = "Field `EPWM6_CTRL_SYNCIN_SEL_PROXY` writer - 10:8\\]
Selects the source of the EPWM6 synchronization input"]
pub type Epwm6CtrlSyncinSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    pub fn epwm6_ctrl_eallow_proxy(&self) -> Epwm6CtrlEallowProxyR {
        Epwm6CtrlEallowProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the source of the EPWM6 synchronization input"]
    #[inline(always)]
    pub fn epwm6_ctrl_syncin_sel_proxy(&self) -> Epwm6CtrlSyncinSelProxyR {
        Epwm6CtrlSyncinSelProxyR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    #[must_use]
    pub fn epwm6_ctrl_eallow_proxy(&mut self) -> Epwm6CtrlEallowProxyW<Cfg0Epwm6CtrlProxySpec> {
        Epwm6CtrlEallowProxyW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the source of the EPWM6 synchronization input"]
    #[inline(always)]
    #[must_use]
    pub fn epwm6_ctrl_syncin_sel_proxy(
        &mut self,
    ) -> Epwm6CtrlSyncinSelProxyW<Cfg0Epwm6CtrlProxySpec> {
        Epwm6CtrlSyncinSelProxyW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM6_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm6_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm6_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Epwm6CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Epwm6CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm6_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Epwm6CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm6_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Epwm6CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM6_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Epwm6CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
