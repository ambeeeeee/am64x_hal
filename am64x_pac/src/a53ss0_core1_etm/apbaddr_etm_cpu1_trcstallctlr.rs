#[doc = "Register `APBADDR_ETM_CPU1_TRCSTALLCTLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcstallctlrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCSTALLCTLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcstallctlrSpec>;
#[doc = "Field `LEVEL` reader - 3:2\\]
The field can support 4 monotonic levels from 0b00 to 0b11"]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - 3:2\\]
The field can support 4 monotonic levels from 0b00 to 0b11"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ISTALL` reader - 8:8\\]
Controls if the trace unit can stall the processor when the instruction trace buffer space is less than LEVEL"]
pub type IstallR = crate::BitReader;
#[doc = "Field `ISTALL` writer - 8:8\\]
Controls if the trace unit can stall the processor when the instruction trace buffer space is less than LEVEL"]
pub type IstallW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:3 - 3:2\\]
The field can support 4 monotonic levels from 0b00 to 0b11"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls if the trace unit can stall the processor when the instruction trace buffer space is less than LEVEL"]
    #[inline(always)]
    pub fn istall(&self) -> IstallR {
        IstallR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - 3:2\\]
The field can support 4 monotonic levels from 0b00 to 0b11"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LevelW<ApbaddrEtmCpu1TrcstallctlrSpec> {
        LevelW::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls if the trace unit can stall the processor when the instruction trace buffer space is less than LEVEL"]
    #[inline(always)]
    #[must_use]
    pub fn istall(&mut self) -> IstallW<ApbaddrEtmCpu1TrcstallctlrSpec> {
        IstallW::new(self, 8)
    }
}
#[doc = "Stall Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcstallctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcstallctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcstallctlrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcstallctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcstallctlr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcstallctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcstallctlr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcstallctlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCSTALLCTLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcstallctlrSpec {
    const RESET_VALUE: u32 = 0;
}
