#[doc = "Register `APBADDR_PMU_CPU0_PMCCFILTR_EL0` reader"]
pub type R = crate::R<ApbaddrPmuCpu0PmccfiltrEl0Spec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMCCFILTR_EL0` writer"]
pub type W = crate::W<ApbaddrPmuCpu0PmccfiltrEl0Spec>;
#[doc = "Field `RES0_PMCCFILTR_EL0_25_0` reader - 25:0\\]
Reserved, RES0."]
pub type Res0PmccfiltrEl0_25_0R = crate::FieldReader<u32>;
#[doc = "Field `RES0_PMCCFILTR_EL0_25_0` writer - 25:0\\]
Reserved, RES0."]
pub type Res0PmccfiltrEl0_25_0W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `M` reader - 26:26\\]
Secure EL3 filtering bit. Most applications can ignore this bit and set the value to zero. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, cycles in Secure EL3 are counted.Otherwise, cycles in Secure EL3 are not counted."]
pub type MR = crate::BitReader;
#[doc = "Field `M` writer - 26:26\\]
Secure EL3 filtering bit. Most applications can ignore this bit and set the value to zero. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, cycles in Secure EL3 are counted.Otherwise, cycles in Secure EL3 are not counted."]
pub type MW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSH` reader - 27:27\\]
Non-secure Hyp modes filtering bit. Controls counting in Non-secure EL2. If EL2 is not implemented, this bit is RES0. 0 Do not count cycles in EL2. 1 Count cycles in EL2."]
pub type NshR = crate::BitReader;
#[doc = "Field `NSH` writer - 27:27\\]
Non-secure Hyp modes filtering bit. Controls counting in Non-secure EL2. If EL2 is not implemented, this bit is RES0. 0 Do not count cycles in EL2. 1 Count cycles in EL2."]
pub type NshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSU` reader - 28:28\\]
Non-secure user modes filtering bit. Controls counting in Non-secure EL0. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of U, cycles in Non-secure EL0 are counted.Otherwise, cycles in Non-secure EL0 are not counted."]
pub type NsuR = crate::BitReader;
#[doc = "Field `NSU` writer - 28:28\\]
Non-secure user modes filtering bit. Controls counting in Non-secure EL0. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of U, cycles in Non-secure EL0 are counted.Otherwise, cycles in Non-secure EL0 are not counted."]
pub type NsuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSK` reader - 29:29\\]
Non-secure kernel modes filtering bit. Controls counting in Non-secure EL1. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, cycles in Non-secure EL1 are counted.Otherwise, cycles in Non-secure EL1 are not counted."]
pub type NskR = crate::BitReader;
#[doc = "Field `NSK` writer - 29:29\\]
Non-secure kernel modes filtering bit. Controls counting in Non-secure EL1. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, cycles in Non-secure EL1 are counted.Otherwise, cycles in Non-secure EL1 are not counted."]
pub type NskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U` reader - 30:30\\]
EL0 filtering bit. Controls counting in EL0. If EL3 is implemented, then counting in Non-secure EL0 is further controlled by the NSU bit. The possible values of this bit are: 0 Count cycles in EL0. 1 Do not count cycles in EL0."]
pub type UR = crate::BitReader;
#[doc = "Field `U` writer - 30:30\\]
EL0 filtering bit. Controls counting in EL0. If EL3 is implemented, then counting in Non-secure EL0 is further controlled by the NSU bit. The possible values of this bit are: 0 Count cycles in EL0. 1 Do not count cycles in EL0."]
pub type UW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P` reader - 31:31\\]
EL1 modes filtering bit. Controls counting in EL1. If EL3 is implemented, then counting in Non-secure EL1 is further controlled by the NSK bit. The possible values of this bit are: 0 Count cycles in EL1. 1 Do not count cycles in EL1."]
pub type PR = crate::BitReader;
#[doc = "Field `P` writer - 31:31\\]
EL1 modes filtering bit. Controls counting in EL1. If EL3 is implemented, then counting in Non-secure EL1 is further controlled by the NSK bit. The possible values of this bit are: 0 Count cycles in EL1. 1 Do not count cycles in EL1."]
pub type PW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - 25:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmccfiltr_el0_25_0(&self) -> Res0PmccfiltrEl0_25_0R {
        Res0PmccfiltrEl0_25_0R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - 26:26\\]
Secure EL3 filtering bit. Most applications can ignore this bit and set the value to zero. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, cycles in Secure EL3 are counted.Otherwise, cycles in Secure EL3 are not counted."]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Non-secure Hyp modes filtering bit. Controls counting in Non-secure EL2. If EL2 is not implemented, this bit is RES0. 0 Do not count cycles in EL2. 1 Count cycles in EL2."]
    #[inline(always)]
    pub fn nsh(&self) -> NshR {
        NshR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Non-secure user modes filtering bit. Controls counting in Non-secure EL0. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of U, cycles in Non-secure EL0 are counted.Otherwise, cycles in Non-secure EL0 are not counted."]
    #[inline(always)]
    pub fn nsu(&self) -> NsuR {
        NsuR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Non-secure kernel modes filtering bit. Controls counting in Non-secure EL1. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, cycles in Non-secure EL1 are counted.Otherwise, cycles in Non-secure EL1 are not counted."]
    #[inline(always)]
    pub fn nsk(&self) -> NskR {
        NskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
EL0 filtering bit. Controls counting in EL0. If EL3 is implemented, then counting in Non-secure EL0 is further controlled by the NSU bit. The possible values of this bit are: 0 Count cycles in EL0. 1 Do not count cycles in EL0."]
    #[inline(always)]
    pub fn u(&self) -> UR {
        UR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
EL1 modes filtering bit. Controls counting in EL1. If EL3 is implemented, then counting in Non-secure EL1 is further controlled by the NSK bit. The possible values of this bit are: 0 Count cycles in EL1. 1 Do not count cycles in EL1."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - 25:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmccfiltr_el0_25_0(
        &mut self,
    ) -> Res0PmccfiltrEl0_25_0W<ApbaddrPmuCpu0PmccfiltrEl0Spec> {
        Res0PmccfiltrEl0_25_0W::new(self, 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Secure EL3 filtering bit. Most applications can ignore this bit and set the value to zero. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, cycles in Secure EL3 are counted.Otherwise, cycles in Secure EL3 are not counted."]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<ApbaddrPmuCpu0PmccfiltrEl0Spec> {
        MW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Non-secure Hyp modes filtering bit. Controls counting in Non-secure EL2. If EL2 is not implemented, this bit is RES0. 0 Do not count cycles in EL2. 1 Count cycles in EL2."]
    #[inline(always)]
    #[must_use]
    pub fn nsh(&mut self) -> NshW<ApbaddrPmuCpu0PmccfiltrEl0Spec> {
        NshW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Non-secure user modes filtering bit. Controls counting in Non-secure EL0. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of U, cycles in Non-secure EL0 are counted.Otherwise, cycles in Non-secure EL0 are not counted."]
    #[inline(always)]
    #[must_use]
    pub fn nsu(&mut self) -> NsuW<ApbaddrPmuCpu0PmccfiltrEl0Spec> {
        NsuW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Non-secure kernel modes filtering bit. Controls counting in Non-secure EL1. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, cycles in Non-secure EL1 are counted.Otherwise, cycles in Non-secure EL1 are not counted."]
    #[inline(always)]
    #[must_use]
    pub fn nsk(&mut self) -> NskW<ApbaddrPmuCpu0PmccfiltrEl0Spec> {
        NskW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
EL0 filtering bit. Controls counting in EL0. If EL3 is implemented, then counting in Non-secure EL0 is further controlled by the NSU bit. The possible values of this bit are: 0 Count cycles in EL0. 1 Do not count cycles in EL0."]
    #[inline(always)]
    #[must_use]
    pub fn u(&mut self) -> UW<ApbaddrPmuCpu0PmccfiltrEl0Spec> {
        UW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
EL1 modes filtering bit. Controls counting in EL1. If EL3 is implemented, then counting in Non-secure EL1 is further controlled by the NSK bit. The possible values of this bit are: 0 Count cycles in EL1. 1 Do not count cycles in EL1."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<ApbaddrPmuCpu0PmccfiltrEl0Spec> {
        PW::new(self, 31)
    }
}
#[doc = "Performance Monitors Cycle Counter Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmccfiltr_el0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmccfiltr_el0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0PmccfiltrEl0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu0PmccfiltrEl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmccfiltr_el0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0PmccfiltrEl0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmccfiltr_el0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0PmccfiltrEl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMCCFILTR_EL0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu0PmccfiltrEl0Spec {
    const RESET_VALUE: u32 = 0;
}
