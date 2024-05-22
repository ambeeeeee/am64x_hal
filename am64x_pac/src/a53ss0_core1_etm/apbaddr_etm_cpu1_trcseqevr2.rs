#[doc = "Register `APBADDR_ETM_CPU1_TRCSEQEVR2` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcseqevr2Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCSEQEVR2` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcseqevr2Spec>;
#[doc = "Field `F_N` reader - 7:0\\]
Forward field. Contains an event number. When the event occurs then the sequencer state moves from state n to state n+1.For example, for TRCSEQEVR1, if F1==0x12 then when event 0x12 occurs, the sequencer moves from state 1 to state 2."]
pub type FNR = crate::FieldReader;
#[doc = "Field `F_N` writer - 7:0\\]
Forward field. Contains an event number. When the event occurs then the sequencer state moves from state n to state n+1.For example, for TRCSEQEVR1, if F1==0x12 then when event 0x12 occurs, the sequencer moves from state 1 to state 2."]
pub type FNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `B_N` reader - 15:8\\]
Backward field. Contains an event number. When the event occurs then the sequencer state moves from state n+1 to state n.For example, for TRCSEQEVR2, if B2==0x14 then when event 0x14 occurs, the sequencer moves from state 3 to state 2."]
pub type BNR = crate::FieldReader;
#[doc = "Field `B_N` writer - 15:8\\]
Backward field. Contains an event number. When the event occurs then the sequencer state moves from state n+1 to state n.For example, for TRCSEQEVR2, if B2==0x14 then when event 0x14 occurs, the sequencer moves from state 3 to state 2."]
pub type BNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_TRCSEQEVR2_31_16` reader - 31:16\\]
Reserved, RES0."]
pub type Res0Trcseqevr2_31_16R = crate::FieldReader<u16>;
#[doc = "Field `RES0_TRCSEQEVR2_31_16` writer - 31:16\\]
Reserved, RES0."]
pub type Res0Trcseqevr2_31_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Forward field. Contains an event number. When the event occurs then the sequencer state moves from state n to state n+1.For example, for TRCSEQEVR1, if F1==0x12 then when event 0x12 occurs, the sequencer moves from state 1 to state 2."]
    #[inline(always)]
    pub fn f_n(&self) -> FNR {
        FNR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Backward field. Contains an event number. When the event occurs then the sequencer state moves from state n+1 to state n.For example, for TRCSEQEVR2, if B2==0x14 then when event 0x14 occurs, the sequencer moves from state 3 to state 2."]
    #[inline(always)]
    pub fn b_n(&self) -> BNR {
        BNR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcseqevr2_31_16(&self) -> Res0Trcseqevr2_31_16R {
        Res0Trcseqevr2_31_16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Forward field. Contains an event number. When the event occurs then the sequencer state moves from state n to state n+1.For example, for TRCSEQEVR1, if F1==0x12 then when event 0x12 occurs, the sequencer moves from state 1 to state 2."]
    #[inline(always)]
    #[must_use]
    pub fn f_n(&mut self) -> FNW<ApbaddrEtmCpu1Trcseqevr2Spec> {
        FNW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Backward field. Contains an event number. When the event occurs then the sequencer state moves from state n+1 to state n.For example, for TRCSEQEVR2, if B2==0x14 then when event 0x14 occurs, the sequencer moves from state 3 to state 2."]
    #[inline(always)]
    #[must_use]
    pub fn b_n(&mut self) -> BNW<ApbaddrEtmCpu1Trcseqevr2Spec> {
        BNW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcseqevr2_31_16(&mut self) -> Res0Trcseqevr2_31_16W<ApbaddrEtmCpu1Trcseqevr2Spec> {
        Res0Trcseqevr2_31_16W::new(self, 16)
    }
}
#[doc = "Sequencer State Transition Control Registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcseqevr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcseqevr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcseqevr2Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcseqevr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcseqevr2::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcseqevr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcseqevr2::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcseqevr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCSEQEVR2 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1Trcseqevr2Spec {
    const RESET_VALUE: u32 = 0;
}
