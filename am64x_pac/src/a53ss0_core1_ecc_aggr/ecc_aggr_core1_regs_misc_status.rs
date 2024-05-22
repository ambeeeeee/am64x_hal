#[doc = "Register `ECC_AGGR_CORE1_REGS_misc_status` reader"]
pub type R = crate::R<EccAggrCore1RegsMiscStatusSpec>;
#[doc = "Register `ECC_AGGR_CORE1_REGS_misc_status` writer"]
pub type W = crate::W<EccAggrCore1RegsMiscStatusSpec>;
#[doc = "Field `NUM_RAMS` reader - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
pub type NumRamsR = crate::FieldReader<u16>;
#[doc = "Field `NUM_RAMS` writer - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
pub type NumRamsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
    #[inline(always)]
    pub fn num_rams(&self) -> NumRamsR {
        NumRamsR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
    #[inline(always)]
    #[must_use]
    pub fn num_rams(&mut self) -> NumRamsW<EccAggrCore1RegsMiscStatusSpec> {
        NumRamsW::new(self, 0)
    }
}
#[doc = "Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_misc_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_misc_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrCore1RegsMiscStatusSpec;
impl crate::RegisterSpec for EccAggrCore1RegsMiscStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_core1_regs_misc_status::R`](R) reader structure"]
impl crate::Readable for EccAggrCore1RegsMiscStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_core1_regs_misc_status::W`](W) writer structure"]
impl crate::Writable for EccAggrCore1RegsMiscStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_CORE1_REGS_misc_status to value 0x27"]
impl crate::Resettable for EccAggrCore1RegsMiscStatusSpec {
    const RESET_VALUE: u32 = 0x27;
}
