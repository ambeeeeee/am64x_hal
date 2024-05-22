#[doc = "Register `VBUS_EPCPR` reader"]
pub type R = crate::R<VbusEpcprSpec>;
#[doc = "Register `VBUS_EPCPR` writer"]
pub type W = crate::W<VbusEpcprSpec>;
#[doc = "Field `EPC` reader - 31:0\\]
External Power Control Intervention Request for Power Domain n"]
pub type EpcR = crate::FieldReader<u32>;
#[doc = "Field `EPC` writer - 31:0\\]
External Power Control Intervention Request for Power Domain n"]
pub type EpcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
External Power Control Intervention Request for Power Domain n"]
    #[inline(always)]
    pub fn epc(&self) -> EpcR {
        EpcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
External Power Control Intervention Request for Power Domain n"]
    #[inline(always)]
    #[must_use]
    pub fn epc(&mut self) -> EpcW<VbusEpcprSpec> {
        EpcW::new(self, 0)
    }
}
#[doc = "This register records pending external power control conditions. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_epcpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_epcpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusEpcprSpec;
impl crate::RegisterSpec for VbusEpcprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_epcpr::R`](R) reader structure"]
impl crate::Readable for VbusEpcprSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_epcpr::W`](W) writer structure"]
impl crate::Writable for VbusEpcprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_EPCPR to value 0"]
impl crate::Resettable for VbusEpcprSpec {
    const RESET_VALUE: u32 = 0;
}
