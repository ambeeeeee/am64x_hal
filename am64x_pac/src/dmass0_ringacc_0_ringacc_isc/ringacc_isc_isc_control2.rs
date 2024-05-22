#[doc = "Register `RINGACC_ISC_ISC_control2` reader"]
pub type R = crate::R<RingaccIscIscControl2Spec>;
#[doc = "Register `RINGACC_ISC_ISC_control2` writer"]
pub type W = crate::W<RingaccIscIscControl2Spec>;
#[doc = "Field `VIRTID` reader - 27:16\\]
Virt ID."]
pub type VirtidR = crate::FieldReader<u16>;
#[doc = "Field `VIRTID` writer - 27:16\\]
Virt ID."]
pub type VirtidW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ATYPE` reader - 29:28\\]
Defines the output address type. 0 = physical no memory attributes. 1 = intermediate. 2 = virtual. 3 = physical with memory attributes."]
pub type AtypeR = crate::FieldReader;
#[doc = "Field `ATYPE` writer - 29:28\\]
Defines the output address type. 0 = physical no memory attributes. 1 = intermediate. 2 = virtual. 3 = physical with memory attributes."]
pub type AtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PASS_V` reader - 31:31\\]
No virtID replacement, pass through value."]
pub type PassVR = crate::BitReader;
#[doc = "Field `PASS_V` writer - 31:31\\]
No virtID replacement, pass through value."]
pub type PassVW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:27 - 27:16\\]
Virt ID."]
    #[inline(always)]
    pub fn virtid(&self) -> VirtidR {
        VirtidR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Defines the output address type. 0 = physical no memory attributes. 1 = intermediate. 2 = virtual. 3 = physical with memory attributes."]
    #[inline(always)]
    pub fn atype(&self) -> AtypeR {
        AtypeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
No virtID replacement, pass through value."]
    #[inline(always)]
    pub fn pass_v(&self) -> PassVR {
        PassVR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:27 - 27:16\\]
Virt ID."]
    #[inline(always)]
    #[must_use]
    pub fn virtid(&mut self) -> VirtidW<RingaccIscIscControl2Spec> {
        VirtidW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Defines the output address type. 0 = physical no memory attributes. 1 = intermediate. 2 = virtual. 3 = physical with memory attributes."]
    #[inline(always)]
    #[must_use]
    pub fn atype(&mut self) -> AtypeW<RingaccIscIscControl2Spec> {
        AtypeW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
No virtID replacement, pass through value."]
    #[inline(always)]
    #[must_use]
    pub fn pass_v(&mut self) -> PassVW<RingaccIscIscControl2Spec> {
        PassVW::new(self, 31)
    }
}
#[doc = "The ISC a Region b Control Register 2 defines the control fields for the ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_isc_isc_control2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_isc_isc_control2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccIscIscControl2Spec;
impl crate::RegisterSpec for RingaccIscIscControl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_isc_isc_control2::R`](R) reader structure"]
impl crate::Readable for RingaccIscIscControl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_isc_isc_control2::W`](W) writer structure"]
impl crate::Writable for RingaccIscIscControl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_ISC_ISC_control2 to value 0"]
impl crate::Resettable for RingaccIscIscControl2Spec {
    const RESET_VALUE: u32 = 0;
}
