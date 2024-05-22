#[doc = "Register `CFG0_MAC_ID0_PROXY` reader"]
pub type R = crate::R<Cfg0MacId0ProxySpec>;
#[doc = "Register `CFG0_MAC_ID0_PROXY` writer"]
pub type W = crate::W<Cfg0MacId0ProxySpec>;
#[doc = "Field `MAC_ID0_MACID_LO_PROXY` reader - 31:0\\]
32 lsbs of MAC address"]
pub type MacId0MacidLoProxyR = crate::FieldReader<u32>;
#[doc = "Field `MAC_ID0_MACID_LO_PROXY` writer - 31:0\\]
32 lsbs of MAC address"]
pub type MacId0MacidLoProxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
32 lsbs of MAC address"]
    #[inline(always)]
    pub fn mac_id0_macid_lo_proxy(&self) -> MacId0MacidLoProxyR {
        MacId0MacidLoProxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
32 lsbs of MAC address"]
    #[inline(always)]
    #[must_use]
    pub fn mac_id0_macid_lo_proxy(&mut self) -> MacId0MacidLoProxyW<Cfg0MacId0ProxySpec> {
        MacId0MacidLoProxyW::new(self, 0)
    }
}
#[doc = "CFG0_MAC_ID0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mac_id0_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mac_id0_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MacId0ProxySpec;
impl crate::RegisterSpec for Cfg0MacId0ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mac_id0_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0MacId0ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mac_id0_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0MacId0ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAC_ID0_PROXY to value 0"]
impl crate::Resettable for Cfg0MacId0ProxySpec {
    const RESET_VALUE: u32 = 0;
}
