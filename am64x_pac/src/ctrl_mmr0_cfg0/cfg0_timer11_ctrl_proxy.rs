#[doc = "Register `CFG0_TIMER11_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Timer11CtrlProxySpec>;
#[doc = "Register `CFG0_TIMER11_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Timer11CtrlProxySpec>;
#[doc = "Field `TIMER11_CTRL_CASCADE_EN_PROXY` reader - 8:8\\]
Activates cascading of TIMER11 to TIMER10"]
pub type Timer11CtrlCascadeEnProxyR = crate::BitReader;
#[doc = "Field `TIMER11_CTRL_CASCADE_EN_PROXY` writer - 8:8\\]
Activates cascading of TIMER11 to TIMER10"]
pub type Timer11CtrlCascadeEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Activates cascading of TIMER11 to TIMER10"]
    #[inline(always)]
    pub fn timer11_ctrl_cascade_en_proxy(&self) -> Timer11CtrlCascadeEnProxyR {
        Timer11CtrlCascadeEnProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Activates cascading of TIMER11 to TIMER10"]
    #[inline(always)]
    #[must_use]
    pub fn timer11_ctrl_cascade_en_proxy(
        &mut self,
    ) -> Timer11CtrlCascadeEnProxyW<Cfg0Timer11CtrlProxySpec> {
        Timer11CtrlCascadeEnProxyW::new(self, 8)
    }
}
#[doc = "CFG0_TIMER11_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer11_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer11_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Timer11CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Timer11CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_timer11_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Timer11CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_timer11_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Timer11CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_TIMER11_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Timer11CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
