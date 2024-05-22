#[doc = "Register `VBUS_MERRPR` reader"]
pub type R = crate::R<VbusMerrprSpec>;
#[doc = "Register `VBUS_MERRPR` writer"]
pub type W = crate::W<VbusMerrprSpec>;
#[doc = "Field `M` reader - 31:0\\]
Records pending error conditions. Each bit n represents a module."]
pub type MR = crate::FieldReader<u32>;
#[doc = "Field `M` writer - 31:0\\]
Records pending error conditions. Each bit n represents a module."]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Records pending error conditions. Each bit n represents a module."]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Records pending error conditions. Each bit n represents a module."]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<VbusMerrprSpec> {
        MW::new(self, 0)
    }
}
#[doc = "This register records pending error conditions for all modules. Each bit represents one module (index 0 for modules 0-31, index 1 for modules 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_merrpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_merrpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusMerrprSpec;
impl crate::RegisterSpec for VbusMerrprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_merrpr::R`](R) reader structure"]
impl crate::Readable for VbusMerrprSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_merrpr::W`](W) writer structure"]
impl crate::Writable for VbusMerrprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_MERRPR to value 0"]
impl crate::Resettable for VbusMerrprSpec {
    const RESET_VALUE: u32 = 0;
}
