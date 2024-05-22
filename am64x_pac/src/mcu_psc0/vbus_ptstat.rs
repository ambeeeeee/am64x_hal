#[doc = "Register `VBUS_PTSTAT` reader"]
pub type R = crate::R<VbusPtstatSpec>;
#[doc = "Register `VBUS_PTSTAT` writer"]
pub type W = crate::W<VbusPtstatSpec>;
#[doc = "Field `GOSTAT` reader - 31:0\\]
Power Domain n Transition Command Status"]
pub type GostatR = crate::FieldReader<u32>;
#[doc = "Field `GOSTAT` writer - 31:0\\]
Power Domain n Transition Command Status"]
pub type GostatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Power Domain n Transition Command Status"]
    #[inline(always)]
    pub fn gostat(&self) -> GostatR {
        GostatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Power Domain n Transition Command Status"]
    #[inline(always)]
    #[must_use]
    pub fn gostat(&mut self) -> GostatW<VbusPtstatSpec> {
        GostatW::new(self, 0)
    }
}
#[doc = "This is a status register. One bit for each power domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_ptstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_ptstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusPtstatSpec;
impl crate::RegisterSpec for VbusPtstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_ptstat::R`](R) reader structure"]
impl crate::Readable for VbusPtstatSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_ptstat::W`](W) writer structure"]
impl crate::Writable for VbusPtstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_PTSTAT to value 0"]
impl crate::Resettable for VbusPtstatSpec {
    const RESET_VALUE: u32 = 0;
}
