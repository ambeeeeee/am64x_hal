#[doc = "Register `ECC_AGGR_REGSREGS_reserved_svbus` reader"]
pub type R = crate::R<EccAggrRegsregsReservedSvbusSpec>;
#[doc = "Register `ECC_AGGR_REGSREGS_reserved_svbus` writer"]
pub type W = crate::W<EccAggrRegsregsReservedSvbusSpec>;
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
    pub fn data(&mut self) -> DataW<EccAggrRegsregsReservedSvbusSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_reserved_svbus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_reserved_svbus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrRegsregsReservedSvbusSpec;
impl crate::RegisterSpec for EccAggrRegsregsReservedSvbusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_regsregs_reserved_svbus::R`](R) reader structure"]
impl crate::Readable for EccAggrRegsregsReservedSvbusSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_regsregs_reserved_svbus::W`](W) writer structure"]
impl crate::Writable for EccAggrRegsregsReservedSvbusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_REGSREGS_reserved_svbus to value 0"]
impl crate::Resettable for EccAggrRegsregsReservedSvbusSpec {
    const RESET_VALUE: u32 = 0;
}
