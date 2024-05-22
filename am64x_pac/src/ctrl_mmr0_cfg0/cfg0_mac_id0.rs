#[doc = "Register `CFG0_MAC_ID0` reader"]
pub type R = crate::R<Cfg0MacId0Spec>;
#[doc = "Register `CFG0_MAC_ID0` writer"]
pub type W = crate::W<Cfg0MacId0Spec>;
#[doc = "Field `MAC_ID0_MACID_LO` reader - 31:0\\]
32 lsbs of MAC address"]
pub type MacId0MacidLoR = crate::FieldReader<u32>;
#[doc = "Field `MAC_ID0_MACID_LO` writer - 31:0\\]
32 lsbs of MAC address"]
pub type MacId0MacidLoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
32 lsbs of MAC address"]
    #[inline(always)]
    pub fn mac_id0_macid_lo(&self) -> MacId0MacidLoR {
        MacId0MacidLoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
32 lsbs of MAC address"]
    #[inline(always)]
    #[must_use]
    pub fn mac_id0_macid_lo(&mut self) -> MacId0MacidLoW<Cfg0MacId0Spec> {
        MacId0MacidLoW::new(self, 0)
    }
}
#[doc = "CFG0_MAC_ID0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mac_id0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mac_id0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MacId0Spec;
impl crate::RegisterSpec for Cfg0MacId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mac_id0::R`](R) reader structure"]
impl crate::Readable for Cfg0MacId0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mac_id0::W`](W) writer structure"]
impl crate::Writable for Cfg0MacId0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAC_ID0 to value 0"]
impl crate::Resettable for Cfg0MacId0Spec {
    const RESET_VALUE: u32 = 0;
}
