#[doc = "Register `APBADDR_ETM_CPU0_TRCITATBIDR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcitatbidrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCITATBIDR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcitatbidrSpec>;
#[doc = "Field `ID` reader - 6:0\\]
Drives the ATIDMn\\[6:0\\]
output pins"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - 6:0\\]
Drives the ATIDMn\\[6:0\\]
output pins"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RES0_TRCITATBIDR_31_7` reader - 31:7\\]
Reserved RES0"]
pub type Res0Trcitatbidr31_7R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCITATBIDR_31_7` writer - 31:7\\]
Reserved RES0"]
pub type Res0Trcitatbidr31_7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Drives the ATIDMn\\[6:0\\]
output pins"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved RES0"]
    #[inline(always)]
    pub fn res0_trcitatbidr_31_7(&self) -> Res0Trcitatbidr31_7R {
        Res0Trcitatbidr31_7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Drives the ATIDMn\\[6:0\\]
output pins"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<ApbaddrEtmCpu0TrcitatbidrSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved RES0"]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcitatbidr_31_7(&mut self) -> Res0Trcitatbidr31_7W<ApbaddrEtmCpu0TrcitatbidrSpec> {
        Res0Trcitatbidr31_7W::new(self, 7)
    }
}
#[doc = "Integration ATB Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcitatbidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcitatbidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcitatbidrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcitatbidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcitatbidr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcitatbidrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcitatbidr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcitatbidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCITATBIDR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcitatbidrSpec {
    const RESET_VALUE: u32 = 0;
}
