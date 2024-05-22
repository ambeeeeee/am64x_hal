#[doc = "Register `CFG0_POR_BANDGAP_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0PorBandgapCtrlProxySpec>;
#[doc = "Register `CFG0_POR_BANDGAP_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0PorBandgapCtrlProxySpec>;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPC_PROXY` reader - 7:0\\]
Bandgap slope trim bits. Bit7 is used to calculate the offset"]
pub type PorBandgapCtrlBgapcProxyR = crate::FieldReader;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPC_PROXY` writer - 7:0\\]
Bandgap slope trim bits. Bit7 is used to calculate the offset"]
pub type PorBandgapCtrlBgapcProxyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPV_PROXY` reader - 15:8\\]
Bandgap output voltage magnitude trim bits"]
pub type PorBandgapCtrlBgapvProxyR = crate::FieldReader;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPV_PROXY` writer - 15:8\\]
Bandgap output voltage magnitude trim bits"]
pub type PorBandgapCtrlBgapvProxyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPI_PROXY` reader - 19:16\\]
Bandgap output current trim bits"]
pub type PorBandgapCtrlBgapiProxyR = crate::FieldReader;
#[doc = "Field `POR_BANDGAP_CTRL_BGAPI_PROXY` writer - 19:16\\]
Bandgap output current trim bits"]
pub type PorBandgapCtrlBgapiProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bandgap slope trim bits. Bit7 is used to calculate the offset"]
    #[inline(always)]
    pub fn por_bandgap_ctrl_bgapc_proxy(&self) -> PorBandgapCtrlBgapcProxyR {
        PorBandgapCtrlBgapcProxyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bandgap output voltage magnitude trim bits"]
    #[inline(always)]
    pub fn por_bandgap_ctrl_bgapv_proxy(&self) -> PorBandgapCtrlBgapvProxyR {
        PorBandgapCtrlBgapvProxyR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Bandgap output current trim bits"]
    #[inline(always)]
    pub fn por_bandgap_ctrl_bgapi_proxy(&self) -> PorBandgapCtrlBgapiProxyR {
        PorBandgapCtrlBgapiProxyR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Bandgap slope trim bits. Bit7 is used to calculate the offset"]
    #[inline(always)]
    #[must_use]
    pub fn por_bandgap_ctrl_bgapc_proxy(
        &mut self,
    ) -> PorBandgapCtrlBgapcProxyW<Cfg0PorBandgapCtrlProxySpec> {
        PorBandgapCtrlBgapcProxyW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bandgap output voltage magnitude trim bits"]
    #[inline(always)]
    #[must_use]
    pub fn por_bandgap_ctrl_bgapv_proxy(
        &mut self,
    ) -> PorBandgapCtrlBgapvProxyW<Cfg0PorBandgapCtrlProxySpec> {
        PorBandgapCtrlBgapvProxyW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Bandgap output current trim bits"]
    #[inline(always)]
    #[must_use]
    pub fn por_bandgap_ctrl_bgapi_proxy(
        &mut self,
    ) -> PorBandgapCtrlBgapiProxyW<Cfg0PorBandgapCtrlProxySpec> {
        PorBandgapCtrlBgapiProxyW::new(self, 16)
    }
}
#[doc = "CFG0_POR_BANDGAP_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_bandgap_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_bandgap_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PorBandgapCtrlProxySpec;
impl crate::RegisterSpec for Cfg0PorBandgapCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_por_bandgap_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PorBandgapCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_por_bandgap_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PorBandgapCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POR_BANDGAP_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0PorBandgapCtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
