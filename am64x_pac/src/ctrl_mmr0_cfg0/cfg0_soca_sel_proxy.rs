#[doc = "Register `CFG0_SOCA_SEL_PROXY` reader"]
pub type R = crate::R<Cfg0SocaSelProxySpec>;
#[doc = "Register `CFG0_SOCA_SEL_PROXY` writer"]
pub type W = crate::W<Cfg0SocaSelProxySpec>;
#[doc = "Field `SOCA_SEL_SOCA_SEL_PROXY` reader - 1:0\\]
Selects the SOC A output source"]
pub type SocaSelSocaSelProxyR = crate::FieldReader;
#[doc = "Field `SOCA_SEL_SOCA_SEL_PROXY` writer - 1:0\\]
Selects the SOC A output source"]
pub type SocaSelSocaSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the SOC A output source"]
    #[inline(always)]
    pub fn soca_sel_soca_sel_proxy(&self) -> SocaSelSocaSelProxyR {
        SocaSelSocaSelProxyR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the SOC A output source"]
    #[inline(always)]
    #[must_use]
    pub fn soca_sel_soca_sel_proxy(&mut self) -> SocaSelSocaSelProxyW<Cfg0SocaSelProxySpec> {
        SocaSelSocaSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_SOCA_SEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_soca_sel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_soca_sel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0SocaSelProxySpec;
impl crate::RegisterSpec for Cfg0SocaSelProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_soca_sel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0SocaSelProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_soca_sel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0SocaSelProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_SOCA_SEL_PROXY to value 0"]
impl crate::Resettable for Cfg0SocaSelProxySpec {
    const RESET_VALUE: u32 = 0;
}
