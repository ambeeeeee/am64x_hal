#[doc = "Register `INTAGGR_INTR_STATUS_MSKD` reader"]
pub type R = crate::R<IntaggrIntrStatusMskdSpec>;
#[doc = "Register `INTAGGR_INTR_STATUS_MSKD` writer"]
pub type W = crate::W<IntaggrIntrStatusMskdSpec>;
#[doc = "Field `INTR_STATUSM` reader - 63:0\\]
Masked state of bits in internal interrupt status register. This value is the result of bitwise ANDing the interrupt enable and status registers"]
pub type IntrStatusmR = crate::FieldReader<u64>;
#[doc = "Field `INTR_STATUSM` writer - 63:0\\]
Masked state of bits in internal interrupt status register. This value is the result of bitwise ANDing the interrupt enable and status registers"]
pub type IntrStatusmW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - 63:0\\]
Masked state of bits in internal interrupt status register. This value is the result of bitwise ANDing the interrupt enable and status registers"]
    #[inline(always)]
    pub fn intr_statusm(&self) -> IntrStatusmR {
        IntrStatusmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - 63:0\\]
Masked state of bits in internal interrupt status register. This value is the result of bitwise ANDing the interrupt enable and status registers"]
    #[inline(always)]
    #[must_use]
    pub fn intr_statusm(&mut self) -> IntrStatusmW<IntaggrIntrStatusMskdSpec> {
        IntrStatusmW::new(self, 0)
    }
}
#[doc = "The Interrupt Masked Status register can be read by software to determine the cause of an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_intr_status_mskd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_intr_status_mskd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrIntrStatusMskdSpec;
impl crate::RegisterSpec for IntaggrIntrStatusMskdSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_intr_status_mskd::R`](R) reader structure"]
impl crate::Readable for IntaggrIntrStatusMskdSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_intr_status_mskd::W`](W) writer structure"]
impl crate::Writable for IntaggrIntrStatusMskdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_INTR_STATUS_MSKD to value 0"]
impl crate::Resettable for IntaggrIntrStatusMskdSpec {
    const RESET_VALUE: u64 = 0;
}
