#[doc = "Register `APBADDR_ETM_CPU1_TRCSEQRSTEVR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcseqrstevrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCSEQRSTEVR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcseqrstevrSpec>;
#[doc = "Field `RST` reader - 7:0\\]
Contains an event number. When the event occurs then the sequencer state moves to state 0."]
pub type RstR = crate::FieldReader;
#[doc = "Field `RST` writer - 7:0\\]
Contains an event number. When the event occurs then the sequencer state moves to state 0."]
pub type RstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_TRCSEQRSTEVR_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trcseqrstevr31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCSEQRSTEVR_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trcseqrstevr31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Contains an event number. When the event occurs then the sequencer state moves to state 0."]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcseqrstevr_31_8(&self) -> Res0Trcseqrstevr31_8R {
        Res0Trcseqrstevr31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Contains an event number. When the event occurs then the sequencer state moves to state 0."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<ApbaddrEtmCpu1TrcseqrstevrSpec> {
        RstW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcseqrstevr_31_8(
        &mut self,
    ) -> Res0Trcseqrstevr31_8W<ApbaddrEtmCpu1TrcseqrstevrSpec> {
        Res0Trcseqrstevr31_8W::new(self, 8)
    }
}
#[doc = "Sequencer Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcseqrstevr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcseqrstevr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcseqrstevrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcseqrstevrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcseqrstevr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcseqrstevrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcseqrstevr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcseqrstevrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCSEQRSTEVR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcseqrstevrSpec {
    const RESET_VALUE: u32 = 0;
}
