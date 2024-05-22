#[doc = "Register `APBADDR_ETM_CPU0_TRCSYNCPR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcsyncprSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCSYNCPR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcsyncprSpec>;
#[doc = "Field `PERIOD` reader - 4:0\\]
Controls how many bytes of trace, the sum of instruction and data, that a trace unit can generate before a periodic trace synchronization request occurs. The number of bytes is always a power of two and the permitted values are: 00000 Periodic trace synchronization requests are disabled. This setting does not disable other types of trace synchronization request. 01000 Periodic trace synchronization request occurs after 2^8, or 256, bytes of trace. 01001 Periodic trace synchronization request occurs after 2^9, or 512, bytes of trace. 01010 Periodic trace synchronization request occurs after 2^10, or 1024, bytes of trace. and so on up to 0b10100, for which the request occurs after 2^20, or 1048576, bytes of trace.Values between 0b00001 and 0b001111 are reserved, as are values from 0b10101 onwards."]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - 4:0\\]
Controls how many bytes of trace, the sum of instruction and data, that a trace unit can generate before a periodic trace synchronization request occurs. The number of bytes is always a power of two and the permitted values are: 00000 Periodic trace synchronization requests are disabled. This setting does not disable other types of trace synchronization request. 01000 Periodic trace synchronization request occurs after 2^8, or 256, bytes of trace. 01001 Periodic trace synchronization request occurs after 2^9, or 512, bytes of trace. 01010 Periodic trace synchronization request occurs after 2^10, or 1024, bytes of trace. and so on up to 0b10100, for which the request occurs after 2^20, or 1048576, bytes of trace.Values between 0b00001 and 0b001111 are reserved, as are values from 0b10101 onwards."]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RES0_TRCSYNCPR_31_5` reader - 31:5\\]
Reserved, RES0."]
pub type Res0Trcsyncpr31_5R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCSYNCPR_31_5` writer - 31:5\\]
Reserved, RES0."]
pub type Res0Trcsyncpr31_5W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Controls how many bytes of trace, the sum of instruction and data, that a trace unit can generate before a periodic trace synchronization request occurs. The number of bytes is always a power of two and the permitted values are: 00000 Periodic trace synchronization requests are disabled. This setting does not disable other types of trace synchronization request. 01000 Periodic trace synchronization request occurs after 2^8, or 256, bytes of trace. 01001 Periodic trace synchronization request occurs after 2^9, or 512, bytes of trace. 01010 Periodic trace synchronization request occurs after 2^10, or 1024, bytes of trace. and so on up to 0b10100, for which the request occurs after 2^20, or 1048576, bytes of trace.Values between 0b00001 and 0b001111 are reserved, as are values from 0b10101 onwards."]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcsyncpr_31_5(&self) -> Res0Trcsyncpr31_5R {
        Res0Trcsyncpr31_5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Controls how many bytes of trace, the sum of instruction and data, that a trace unit can generate before a periodic trace synchronization request occurs. The number of bytes is always a power of two and the permitted values are: 00000 Periodic trace synchronization requests are disabled. This setting does not disable other types of trace synchronization request. 01000 Periodic trace synchronization request occurs after 2^8, or 256, bytes of trace. 01001 Periodic trace synchronization request occurs after 2^9, or 512, bytes of trace. 01010 Periodic trace synchronization request occurs after 2^10, or 1024, bytes of trace. and so on up to 0b10100, for which the request occurs after 2^20, or 1048576, bytes of trace.Values between 0b00001 and 0b001111 are reserved, as are values from 0b10101 onwards."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PeriodW<ApbaddrEtmCpu0TrcsyncprSpec> {
        PeriodW::new(self, 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcsyncpr_31_5(&mut self) -> Res0Trcsyncpr31_5W<ApbaddrEtmCpu0TrcsyncprSpec> {
        Res0Trcsyncpr31_5W::new(self, 5)
    }
}
#[doc = "Synchronization Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcsyncpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcsyncpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcsyncprSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcsyncprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcsyncpr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcsyncprSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcsyncpr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcsyncprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCSYNCPR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcsyncprSpec {
    const RESET_VALUE: u32 = 0;
}
