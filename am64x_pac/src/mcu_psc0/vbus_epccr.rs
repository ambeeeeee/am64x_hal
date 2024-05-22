#[doc = "Register `VBUS_EPCCR` reader"]
pub type R = crate::R<VbusEpccrSpec>;
#[doc = "Register `VBUS_EPCCR` writer"]
pub type W = crate::W<VbusEpccrSpec>;
#[doc = "Field `EPC` reader - 31:0\\]
Write of 1 clears the corresponding EPCPR bit"]
pub type EpcR = crate::FieldReader<u32>;
#[doc = "Field `EPC` writer - 31:0\\]
Write of 1 clears the corresponding EPCPR bit"]
pub type EpcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Write of 1 clears the corresponding EPCPR bit"]
    #[inline(always)]
    pub fn epc(&self) -> EpcR {
        EpcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Write of 1 clears the corresponding EPCPR bit"]
    #[inline(always)]
    #[must_use]
    pub fn epc(&mut self) -> EpcW<VbusEpccrSpec> {
        EpcW::new(self, 0)
    }
}
#[doc = "This register has no storage. Read from this register returns 0. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_epccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_epccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusEpccrSpec;
impl crate::RegisterSpec for VbusEpccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_epccr::R`](R) reader structure"]
impl crate::Readable for VbusEpccrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_epccr::W`](W) writer structure"]
impl crate::Writable for VbusEpccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_EPCCR to value 0"]
impl crate::Resettable for VbusEpccrSpec {
    const RESET_VALUE: u32 = 0;
}
