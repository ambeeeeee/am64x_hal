#[doc = "Register `APBADDR_ETM_CPU0_TRCITIATBINR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcitiatbinrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCITIATBINR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcitiatbinrSpec>;
#[doc = "Field `ATREADYM` reader - 0:0\\]
Returns the value of the ATREADYMn input pin"]
pub type AtreadymR = crate::BitReader;
#[doc = "Field `ATREADYM` writer - 0:0\\]
Returns the value of the ATREADYMn input pin"]
pub type AtreadymW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFVALIDM` reader - 1:1\\]
Returns the value of the AFVALIDMn input pin"]
pub type AfvalidmR = crate::BitReader;
#[doc = "Field `AFVALIDM` writer - 1:1\\]
Returns the value of the AFVALIDMn input pin"]
pub type AfvalidmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Returns the value of the ATREADYMn input pin"]
    #[inline(always)]
    pub fn atreadym(&self) -> AtreadymR {
        AtreadymR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Returns the value of the AFVALIDMn input pin"]
    #[inline(always)]
    pub fn afvalidm(&self) -> AfvalidmR {
        AfvalidmR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Returns the value of the ATREADYMn input pin"]
    #[inline(always)]
    #[must_use]
    pub fn atreadym(&mut self) -> AtreadymW<ApbaddrEtmCpu0TrcitiatbinrSpec> {
        AtreadymW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Returns the value of the AFVALIDMn input pin"]
    #[inline(always)]
    #[must_use]
    pub fn afvalidm(&mut self) -> AfvalidmW<ApbaddrEtmCpu0TrcitiatbinrSpec> {
        AfvalidmW::new(self, 1)
    }
}
#[doc = "Integration Instruction ATB In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcitiatbinr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcitiatbinr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcitiatbinrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcitiatbinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcitiatbinr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcitiatbinrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcitiatbinr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcitiatbinrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCITIATBINR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcitiatbinrSpec {
    const RESET_VALUE: u32 = 0;
}
