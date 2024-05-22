#[doc = "Register `CFG0_MAC_ID1` reader"]
pub type R = crate::R<Cfg0MacId1Spec>;
#[doc = "Register `CFG0_MAC_ID1` writer"]
pub type W = crate::W<Cfg0MacId1Spec>;
#[doc = "Field `MAC_ID1_MACID_HI` reader - 15:0\\]
16 msbs of MAC address"]
pub type MacId1MacidHiR = crate::FieldReader<u16>;
#[doc = "Field `MAC_ID1_MACID_HI` writer - 15:0\\]
16 msbs of MAC address"]
pub type MacId1MacidHiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
16 msbs of MAC address"]
    #[inline(always)]
    pub fn mac_id1_macid_hi(&self) -> MacId1MacidHiR {
        MacId1MacidHiR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
16 msbs of MAC address"]
    #[inline(always)]
    #[must_use]
    pub fn mac_id1_macid_hi(&mut self) -> MacId1MacidHiW<Cfg0MacId1Spec> {
        MacId1MacidHiW::new(self, 0)
    }
}
#[doc = "CFG0_MAC_ID1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mac_id1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mac_id1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MacId1Spec;
impl crate::RegisterSpec for Cfg0MacId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mac_id1::R`](R) reader structure"]
impl crate::Readable for Cfg0MacId1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mac_id1::W`](W) writer structure"]
impl crate::Writable for Cfg0MacId1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAC_ID1 to value 0"]
impl crate::Resettable for Cfg0MacId1Spec {
    const RESET_VALUE: u32 = 0;
}
