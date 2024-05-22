#[doc = "Register `INTAGGR_INTR_STATUS_SET` reader"]
pub type R = crate::R<IntaggrIntrStatusSetSpec>;
#[doc = "Register `INTAGGR_INTR_STATUS_SET` writer"]
pub type W = crate::W<IntaggrIntrStatusSetSpec>;
#[doc = "Field `INTR_STATUS` reader - 63:0\\]
Raw state (not enabled/masked) of bits in internal interrupt status register. Writing a 1 to any bit of this register will cause the corresponding raw status bit to be set"]
pub type IntrStatusR = crate::FieldReader<u64>;
#[doc = "Field `INTR_STATUS` writer - 63:0\\]
Raw state (not enabled/masked) of bits in internal interrupt status register. Writing a 1 to any bit of this register will cause the corresponding raw status bit to be set"]
pub type IntrStatusW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - 63:0\\]
Raw state (not enabled/masked) of bits in internal interrupt status register. Writing a 1 to any bit of this register will cause the corresponding raw status bit to be set"]
    #[inline(always)]
    pub fn intr_status(&self) -> IntrStatusR {
        IntrStatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - 63:0\\]
Raw state (not enabled/masked) of bits in internal interrupt status register. Writing a 1 to any bit of this register will cause the corresponding raw status bit to be set"]
    #[inline(always)]
    #[must_use]
    pub fn intr_status(&mut self) -> IntrStatusW<IntaggrIntrStatusSetSpec> {
        IntrStatusW::new(self, 0)
    }
}
#[doc = "The Interrupt Status register is read by software to determine the cause of an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_intr_status_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_intr_status_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrIntrStatusSetSpec;
impl crate::RegisterSpec for IntaggrIntrStatusSetSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_intr_status_set::R`](R) reader structure"]
impl crate::Readable for IntaggrIntrStatusSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_intr_status_set::W`](W) writer structure"]
impl crate::Writable for IntaggrIntrStatusSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_INTR_STATUS_SET to value 0"]
impl crate::Resettable for IntaggrIntrStatusSetSpec {
    const RESET_VALUE: u64 = 0;
}
