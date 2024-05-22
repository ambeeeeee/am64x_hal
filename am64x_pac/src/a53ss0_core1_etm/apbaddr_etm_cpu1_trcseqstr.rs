#[doc = "Register `APBADDR_ETM_CPU1_TRCSEQSTR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcseqstrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCSEQSTR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcseqstrSpec>;
#[doc = "Field `STATE` reader - 1:0\\]
Sets or returns the state of the sequencer: 00 State 0. 01 State 1. 10 State 2. 11 State 3."]
pub type StateR = crate::FieldReader;
#[doc = "Field `STATE` writer - 1:0\\]
Sets or returns the state of the sequencer: 00 State 0. 01 State 1. 10 State 2. 11 State 3."]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_TRCSEQSTR_31_2` reader - 31:2\\]
Reserved, RES0."]
pub type Res0Trcseqstr31_2R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCSEQSTR_31_2` writer - 31:2\\]
Reserved, RES0."]
pub type Res0Trcseqstr31_2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Sets or returns the state of the sequencer: 00 State 0. 01 State 1. 10 State 2. 11 State 3."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcseqstr_31_2(&self) -> Res0Trcseqstr31_2R {
        Res0Trcseqstr31_2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Sets or returns the state of the sequencer: 00 State 0. 01 State 1. 10 State 2. 11 State 3."]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> StateW<ApbaddrEtmCpu1TrcseqstrSpec> {
        StateW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcseqstr_31_2(&mut self) -> Res0Trcseqstr31_2W<ApbaddrEtmCpu1TrcseqstrSpec> {
        Res0Trcseqstr31_2W::new(self, 2)
    }
}
#[doc = "Sequencer State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcseqstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcseqstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcseqstrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcseqstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcseqstr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcseqstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcseqstr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcseqstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCSEQSTR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcseqstrSpec {
    const RESET_VALUE: u32 = 0;
}
