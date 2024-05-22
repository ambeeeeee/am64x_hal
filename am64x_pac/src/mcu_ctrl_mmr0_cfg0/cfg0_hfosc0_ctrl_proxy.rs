#[doc = "Register `CFG0_HFOSC0_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Hfosc0CtrlProxySpec>;
#[doc = "Register `CFG0_HFOSC0_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Hfosc0CtrlProxySpec>;
#[doc = "Field `HFOSC0_CTRL_BP_C_PROXY` reader - 4:4\\]
Oscillator bypass control. When set oscillator is in bypass mode"]
pub type Hfosc0CtrlBpCProxyR = crate::BitReader;
#[doc = "Field `HFOSC0_CTRL_BP_C_PROXY` writer - 4:4\\]
Oscillator bypass control. When set oscillator is in bypass mode"]
pub type Hfosc0CtrlBpCProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Oscillator bypass control. When set oscillator is in bypass mode"]
    #[inline(always)]
    pub fn hfosc0_ctrl_bp_c_proxy(&self) -> Hfosc0CtrlBpCProxyR {
        Hfosc0CtrlBpCProxyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Oscillator bypass control. When set oscillator is in bypass mode"]
    #[inline(always)]
    #[must_use]
    pub fn hfosc0_ctrl_bp_c_proxy(&mut self) -> Hfosc0CtrlBpCProxyW<Cfg0Hfosc0CtrlProxySpec> {
        Hfosc0CtrlBpCProxyW::new(self, 4)
    }
}
#[doc = "CFG0_HFOSC0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Hfosc0CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Hfosc0CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_hfosc0_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Hfosc0CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_hfosc0_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Hfosc0CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_HFOSC0_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Hfosc0CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
