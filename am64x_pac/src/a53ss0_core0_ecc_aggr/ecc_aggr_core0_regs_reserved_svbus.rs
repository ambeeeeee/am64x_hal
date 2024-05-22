#[doc = "Register `ECC_AGGR_CORE0_REGS_reserved_svbus` reader"]
pub type R = crate::R<EccAggrCore0RegsReservedSvbusSpec>;
#[doc = "Register `ECC_AGGR_CORE0_REGS_reserved_svbus` writer"]
pub type W = crate::W<EccAggrCore0RegsReservedSvbusSpec>;
#[doc = "Field `DATA` reader - 31:0\\]
Serial VBUS register data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - 31:0\\]
Serial VBUS register data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Serial VBUS register data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Serial VBUS register data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<EccAggrCore0RegsReservedSvbusSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_reserved_svbus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_reserved_svbus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrCore0RegsReservedSvbusSpec;
impl crate::RegisterSpec for EccAggrCore0RegsReservedSvbusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_core0_regs_reserved_svbus::R`](R) reader structure"]
impl crate::Readable for EccAggrCore0RegsReservedSvbusSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_core0_regs_reserved_svbus::W`](W) writer structure"]
impl crate::Writable for EccAggrCore0RegsReservedSvbusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_CORE0_REGS_reserved_svbus to value 0"]
impl crate::Resettable for EccAggrCore0RegsReservedSvbusSpec {
    const RESET_VALUE: u32 = 0;
}
