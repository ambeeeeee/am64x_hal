#[doc = "Register `APBADDR_ETM_CPU0_TRCITIATBOUTR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcitiatboutrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCITIATBOUTR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcitiatboutrSpec>;
#[doc = "Field `ATVALID` reader - 0:0\\]
Drives the ATVALIDMn output pin"]
pub type AtvalidR = crate::BitReader;
#[doc = "Field `ATVALID` writer - 0:0\\]
Drives the ATVALIDMn output pin"]
pub type AtvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFREADY` reader - 1:1\\]
Drives the AFREADYMn output pin"]
pub type AfreadyR = crate::BitReader;
#[doc = "Field `AFREADY` writer - 1:1\\]
Drives the AFREADYMn output pin"]
pub type AfreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTES` reader - 9:8\\]
Drives the ATBYTESMn\\[1:0\\]
output pins"]
pub type BytesR = crate::FieldReader;
#[doc = "Field `BYTES` writer - 9:8\\]
Drives the ATBYTESMn\\[1:0\\]
output pins"]
pub type BytesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Drives the ATVALIDMn output pin"]
    #[inline(always)]
    pub fn atvalid(&self) -> AtvalidR {
        AtvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Drives the AFREADYMn output pin"]
    #[inline(always)]
    pub fn afready(&self) -> AfreadyR {
        AfreadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Drives the ATBYTESMn\\[1:0\\]
output pins"]
    #[inline(always)]
    pub fn bytes(&self) -> BytesR {
        BytesR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Drives the ATVALIDMn output pin"]
    #[inline(always)]
    #[must_use]
    pub fn atvalid(&mut self) -> AtvalidW<ApbaddrEtmCpu0TrcitiatboutrSpec> {
        AtvalidW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Drives the AFREADYMn output pin"]
    #[inline(always)]
    #[must_use]
    pub fn afready(&mut self) -> AfreadyW<ApbaddrEtmCpu0TrcitiatboutrSpec> {
        AfreadyW::new(self, 1)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Drives the ATBYTESMn\\[1:0\\]
output pins"]
    #[inline(always)]
    #[must_use]
    pub fn bytes(&mut self) -> BytesW<ApbaddrEtmCpu0TrcitiatboutrSpec> {
        BytesW::new(self, 8)
    }
}
#[doc = "Integration Instruction ATB Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcitiatboutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcitiatboutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcitiatboutrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcitiatboutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcitiatboutr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcitiatboutrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcitiatboutr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcitiatboutrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCITIATBOUTR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcitiatboutrSpec {
    const RESET_VALUE: u32 = 0;
}
