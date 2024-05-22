#[doc = "Register `VBUS_PERRPR` reader"]
pub type R = crate::R<VbusPerrprSpec>;
#[doc = "Register `VBUS_PERRPR` writer"]
pub type W = crate::W<VbusPerrprSpec>;
#[doc = "Field `P` reader - 31:0\\]
Power Domain n Error Condition. Each bit n represents a power domain."]
pub type PR = crate::FieldReader<u32>;
#[doc = "Field `P` writer - 31:0\\]
Power Domain n Error Condition. Each bit n represents a power domain."]
pub type PW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Power Domain n Error Condition. Each bit n represents a power domain."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Power Domain n Error Condition. Each bit n represents a power domain."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<VbusPerrprSpec> {
        PW::new(self, 0)
    }
}
#[doc = "This register records pending error conditions for each power domain. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_perrpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_perrpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusPerrprSpec;
impl crate::RegisterSpec for VbusPerrprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_perrpr::R`](R) reader structure"]
impl crate::Readable for VbusPerrprSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_perrpr::W`](W) writer structure"]
impl crate::Writable for VbusPerrprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_PERRPR to value 0"]
impl crate::Resettable for VbusPerrprSpec {
    const RESET_VALUE: u32 = 0;
}
