#[doc = "Register `CFG0_SOCB_SEL_PROXY` reader"]
pub type R = crate::R<Cfg0SocbSelProxySpec>;
#[doc = "Register `CFG0_SOCB_SEL_PROXY` writer"]
pub type W = crate::W<Cfg0SocbSelProxySpec>;
#[doc = "Field `SOCB_SEL_SOCB_SEL_PROXY` reader - 1:0\\]
Selects the SOC B output source"]
pub type SocbSelSocbSelProxyR = crate::FieldReader;
#[doc = "Field `SOCB_SEL_SOCB_SEL_PROXY` writer - 1:0\\]
Selects the SOC B output source"]
pub type SocbSelSocbSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the SOC B output source"]
    #[inline(always)]
    pub fn socb_sel_socb_sel_proxy(&self) -> SocbSelSocbSelProxyR {
        SocbSelSocbSelProxyR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the SOC B output source"]
    #[inline(always)]
    #[must_use]
    pub fn socb_sel_socb_sel_proxy(&mut self) -> SocbSelSocbSelProxyW<Cfg0SocbSelProxySpec> {
        SocbSelSocbSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_SOCB_SEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_socb_sel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_socb_sel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0SocbSelProxySpec;
impl crate::RegisterSpec for Cfg0SocbSelProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_socb_sel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0SocbSelProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_socb_sel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0SocbSelProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_SOCB_SEL_PROXY to value 0"]
impl crate::Resettable for Cfg0SocbSelProxySpec {
    const RESET_VALUE: u32 = 0;
}
