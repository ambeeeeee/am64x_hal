#[doc = "Register `CFG0_EQEP0_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Eqep0CtrlProxySpec>;
#[doc = "Register `CFG0_EQEP0_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Eqep0CtrlProxySpec>;
#[doc = "Field `EQEP0_CTRL_SOCA_SEL_PROXY` reader - 4:0\\]
Selects the source of SOCA input for EQEP0"]
pub type Eqep0CtrlSocaSelProxyR = crate::FieldReader;
#[doc = "Field `EQEP0_CTRL_SOCA_SEL_PROXY` writer - 4:0\\]
Selects the source of SOCA input for EQEP0"]
pub type Eqep0CtrlSocaSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the source of SOCA input for EQEP0"]
    #[inline(always)]
    pub fn eqep0_ctrl_soca_sel_proxy(&self) -> Eqep0CtrlSocaSelProxyR {
        Eqep0CtrlSocaSelProxyR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the source of SOCA input for EQEP0"]
    #[inline(always)]
    #[must_use]
    pub fn eqep0_ctrl_soca_sel_proxy(&mut self) -> Eqep0CtrlSocaSelProxyW<Cfg0Eqep0CtrlProxySpec> {
        Eqep0CtrlSocaSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_EQEP0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep0_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep0_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Eqep0CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Eqep0CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_eqep0_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Eqep0CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_eqep0_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Eqep0CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EQEP0_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Eqep0CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
