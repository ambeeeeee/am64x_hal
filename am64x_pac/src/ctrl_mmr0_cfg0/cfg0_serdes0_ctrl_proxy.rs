#[doc = "Register `CFG0_SERDES0_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Serdes0CtrlProxySpec>;
#[doc = "Register `CFG0_SERDES0_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Serdes0CtrlProxySpec>;
#[doc = "Field `SERDES0_CTRL_RET_EN_PROXY` reader - 8:8\\]
Retention activate"]
pub type Serdes0CtrlRetEnProxyR = crate::BitReader;
#[doc = "Field `SERDES0_CTRL_RET_EN_PROXY` writer - 8:8\\]
Retention activate"]
pub type Serdes0CtrlRetEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERDES0_CTRL_REF_SEL_PROXY` reader - 12:12\\]
REFCLK output select"]
pub type Serdes0CtrlRefSelProxyR = crate::BitReader;
#[doc = "Field `SERDES0_CTRL_REF_SEL_PROXY` writer - 12:12\\]
REFCLK output select"]
pub type Serdes0CtrlRefSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Retention activate"]
    #[inline(always)]
    pub fn serdes0_ctrl_ret_en_proxy(&self) -> Serdes0CtrlRetEnProxyR {
        Serdes0CtrlRetEnProxyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
REFCLK output select"]
    #[inline(always)]
    pub fn serdes0_ctrl_ref_sel_proxy(&self) -> Serdes0CtrlRefSelProxyR {
        Serdes0CtrlRefSelProxyR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Retention activate"]
    #[inline(always)]
    #[must_use]
    pub fn serdes0_ctrl_ret_en_proxy(
        &mut self,
    ) -> Serdes0CtrlRetEnProxyW<Cfg0Serdes0CtrlProxySpec> {
        Serdes0CtrlRetEnProxyW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
REFCLK output select"]
    #[inline(always)]
    #[must_use]
    pub fn serdes0_ctrl_ref_sel_proxy(
        &mut self,
    ) -> Serdes0CtrlRefSelProxyW<Cfg0Serdes0CtrlProxySpec> {
        Serdes0CtrlRefSelProxyW::new(self, 12)
    }
}
#[doc = "CFG0_SERDES0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_serdes0_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_serdes0_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Serdes0CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Serdes0CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_serdes0_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Serdes0CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_serdes0_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Serdes0CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_SERDES0_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0Serdes0CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
