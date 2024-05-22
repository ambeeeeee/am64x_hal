#[doc = "Register `CFG0_SERDES0_LN0_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Serdes0Ln0CtrlProxySpec>;
#[doc = "Register `CFG0_SERDES0_LN0_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Serdes0Ln0CtrlProxySpec>;
#[doc = "Field `SERDES0_LN0_CTRL_LANE_FUNC_SEL_PROXY` reader - 1:0\\]
Selects the SERDES0 lane0 function"]
pub type Serdes0Ln0CtrlLaneFuncSelProxyR = crate::FieldReader;
#[doc = "Field `SERDES0_LN0_CTRL_LANE_FUNC_SEL_PROXY` writer - 1:0\\]
Selects the SERDES0 lane0 function"]
pub type Serdes0Ln0CtrlLaneFuncSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the SERDES0 lane0 function"]
    #[inline(always)]
    pub fn serdes0_ln0_ctrl_lane_func_sel_proxy(&self) -> Serdes0Ln0CtrlLaneFuncSelProxyR {
        Serdes0Ln0CtrlLaneFuncSelProxyR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the SERDES0 lane0 function"]
    #[inline(always)]
    #[must_use]
    pub fn serdes0_ln0_ctrl_lane_func_sel_proxy(
        &mut self,
    ) -> Serdes0Ln0CtrlLaneFuncSelProxyW<Cfg0Serdes0Ln0CtrlProxySpec> {
        Serdes0Ln0CtrlLaneFuncSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_SERDES0_LN0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_serdes0_ln0_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_serdes0_ln0_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Serdes0Ln0CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Serdes0Ln0CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_serdes0_ln0_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Serdes0Ln0CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_serdes0_ln0_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Serdes0Ln0CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_SERDES0_LN0_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Serdes0Ln0CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
