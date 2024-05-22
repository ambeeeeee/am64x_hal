#[doc = "Register `EPWM_REGS_TBPHS` reader"]
pub type R = crate::R<EpwmRegsTbphsSpec>;
#[doc = "Register `EPWM_REGS_TBPHS` writer"]
pub type W = crate::W<EpwmRegsTbphsSpec>;
#[doc = "Field `TBPHS` reader - 15:0\\]
These bits set time-base counter phase of the selected ePWM relative to the time-base that is supplying the synchronization input signal \\[a\\]
If TBCTL\\[PHSEN\\]
= 0, then the synchronization event is ignored and the time-base counter is not loaded with the phase \\[b\\]
If TBCTL\\[PHSEN\\]
= 1, then the time-base counter \\[TBCNT\\]
will be loaded with the phase \\[TBPHS\\]
when a synchronization event occurs The synchronization event can be initiated by the input synchronization signal \\[EPWMxSYNCI\\]
or by a software forced synchronization"]
pub type TbphsR = crate::FieldReader<u16>;
#[doc = "Field `TBPHS` writer - 15:0\\]
These bits set time-base counter phase of the selected ePWM relative to the time-base that is supplying the synchronization input signal \\[a\\]
If TBCTL\\[PHSEN\\]
= 0, then the synchronization event is ignored and the time-base counter is not loaded with the phase \\[b\\]
If TBCTL\\[PHSEN\\]
= 1, then the time-base counter \\[TBCNT\\]
will be loaded with the phase \\[TBPHS\\]
when a synchronization event occurs The synchronization event can be initiated by the input synchronization signal \\[EPWMxSYNCI\\]
or by a software forced synchronization"]
pub type TbphsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
These bits set time-base counter phase of the selected ePWM relative to the time-base that is supplying the synchronization input signal \\[a\\]
If TBCTL\\[PHSEN\\]
= 0, then the synchronization event is ignored and the time-base counter is not loaded with the phase \\[b\\]
If TBCTL\\[PHSEN\\]
= 1, then the time-base counter \\[TBCNT\\]
will be loaded with the phase \\[TBPHS\\]
when a synchronization event occurs The synchronization event can be initiated by the input synchronization signal \\[EPWMxSYNCI\\]
or by a software forced synchronization"]
    #[inline(always)]
    pub fn tbphs(&self) -> TbphsR {
        TbphsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
These bits set time-base counter phase of the selected ePWM relative to the time-base that is supplying the synchronization input signal \\[a\\]
If TBCTL\\[PHSEN\\]
= 0, then the synchronization event is ignored and the time-base counter is not loaded with the phase \\[b\\]
If TBCTL\\[PHSEN\\]
= 1, then the time-base counter \\[TBCNT\\]
will be loaded with the phase \\[TBPHS\\]
when a synchronization event occurs The synchronization event can be initiated by the input synchronization signal \\[EPWMxSYNCI\\]
or by a software forced synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn tbphs(&mut self) -> TbphsW<EpwmRegsTbphsSpec> {
        TbphsW::new(self, 0)
    }
}
#[doc = "Time Base Phase Register. This register is only available on ePWM instances that include the high-resolution PWM (HRPWM) extension, otherwise, this location is reserved.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbphs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbphs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTbphsSpec;
impl crate::RegisterSpec for EpwmRegsTbphsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tbphs::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTbphsSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tbphs::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTbphsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TBPHS to value 0"]
impl crate::Resettable for EpwmRegsTbphsSpec {
    const RESET_VALUE: u16 = 0;
}
