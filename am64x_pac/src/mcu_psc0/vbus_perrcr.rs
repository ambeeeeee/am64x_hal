#[doc = "Register `VBUS_PERRCR` reader"]
pub type R = crate::R<VbusPerrcrSpec>;
#[doc = "Register `VBUS_PERRCR` writer"]
pub type W = crate::W<VbusPerrcrSpec>;
#[doc = "Field `P` reader - 31:0\\]
Write of 1 clears the corresponding PERRPR bit."]
pub type PR = crate::FieldReader<u32>;
#[doc = "Field `P` writer - 31:0\\]
Write of 1 clears the corresponding PERRPR bit."]
pub type PW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Write of 1 clears the corresponding PERRPR bit."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Write of 1 clears the corresponding PERRPR bit."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<VbusPerrcrSpec> {
        PW::new(self, 0)
    }
}
#[doc = "This register has no storage. Read from this register returns 0. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_perrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_perrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusPerrcrSpec;
impl crate::RegisterSpec for VbusPerrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_perrcr::R`](R) reader structure"]
impl crate::Readable for VbusPerrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_perrcr::W`](W) writer structure"]
impl crate::Writable for VbusPerrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_PERRCR to value 0"]
impl crate::Resettable for VbusPerrcrSpec {
    const RESET_VALUE: u32 = 0;
}
