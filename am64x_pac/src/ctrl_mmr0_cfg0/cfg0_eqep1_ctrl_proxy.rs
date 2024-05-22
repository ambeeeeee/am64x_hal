#[doc = "Register `CFG0_EQEP1_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Eqep1CtrlProxySpec>;
#[doc = "Register `CFG0_EQEP1_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Eqep1CtrlProxySpec>;
#[doc = "Field `EQEP1_CTRL_SOCA_SEL_PROXY` reader - 4:0\\]
Selects the source of SOCA input for EQEP1"]
pub type Eqep1CtrlSocaSelProxyR = crate::FieldReader;
#[doc = "Field `EQEP1_CTRL_SOCA_SEL_PROXY` writer - 4:0\\]
Selects the source of SOCA input for EQEP1"]
pub type Eqep1CtrlSocaSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the source of SOCA input for EQEP1"]
    #[inline(always)]
    pub fn eqep1_ctrl_soca_sel_proxy(&self) -> Eqep1CtrlSocaSelProxyR {
        Eqep1CtrlSocaSelProxyR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the source of SOCA input for EQEP1"]
    #[inline(always)]
    #[must_use]
    pub fn eqep1_ctrl_soca_sel_proxy(&mut self) -> Eqep1CtrlSocaSelProxyW<Cfg0Eqep1CtrlProxySpec> {
        Eqep1CtrlSocaSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_EQEP1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep1_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep1_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Eqep1CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Eqep1CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_eqep1_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Eqep1CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_eqep1_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Eqep1CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EQEP1_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Eqep1CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
