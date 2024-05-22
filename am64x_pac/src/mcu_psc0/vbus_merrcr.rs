#[doc = "Register `VBUS_MERRCR` reader"]
pub type R = crate::R<VbusMerrcrSpec>;
#[doc = "Register `VBUS_MERRCR` writer"]
pub type W = crate::W<VbusMerrcrSpec>;
#[doc = "Field `M` reader - 31:0\\]
Write of 1 clears the corresponding MERRPR bit."]
pub type MR = crate::FieldReader<u32>;
#[doc = "Field `M` writer - 31:0\\]
Write of 1 clears the corresponding MERRPR bit."]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Write of 1 clears the corresponding MERRPR bit."]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Write of 1 clears the corresponding MERRPR bit."]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<VbusMerrcrSpec> {
        MW::new(self, 0)
    }
}
#[doc = "This register has no storage. Read from this register returns 0. Each bit represents one module (index 0 for modules 0-31, index 1 for modules 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_merrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_merrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusMerrcrSpec;
impl crate::RegisterSpec for VbusMerrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_merrcr::R`](R) reader structure"]
impl crate::Readable for VbusMerrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_merrcr::W`](W) writer structure"]
impl crate::Writable for VbusMerrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_MERRCR to value 0"]
impl crate::Resettable for VbusMerrcrSpec {
    const RESET_VALUE: u32 = 0;
}
