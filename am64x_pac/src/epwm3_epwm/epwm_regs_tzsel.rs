#[doc = "Register `EPWM_REGS_TZSEL` reader"]
pub type R = crate::R<EpwmRegsTzselSpec>;
#[doc = "Register `EPWM_REGS_TZSEL` writer"]
pub type W = crate::W<EpwmRegsTzselSpec>;
#[doc = "Field `CBCN` reader - 7:0\\]
Trip-zone n \\[TZn\\]
select Cycle-by-Cycle \\[CBC\\]
trip-zone enable/disable When any of the enabled pins go low, a cycle-by-cycle trip event occurs for this ePWM module When the event occurs, the action defined in the TZCTL register is taken on the EPWMxA and EPWMxB outputs A cycle-by-cycle trip condition is automatically cleared when the time-base counter reaches zero"]
pub type CbcnR = crate::FieldReader;
#[doc = "Field `CBCN` writer - 7:0\\]
Trip-zone n \\[TZn\\]
select Cycle-by-Cycle \\[CBC\\]
trip-zone enable/disable When any of the enabled pins go low, a cycle-by-cycle trip event occurs for this ePWM module When the event occurs, the action defined in the TZCTL register is taken on the EPWMxA and EPWMxB outputs A cycle-by-cycle trip condition is automatically cleared when the time-base counter reaches zero"]
pub type CbcnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OSHTN` reader - 15:8\\]
Trip-zone n \\[TZn\\]
select One-Shot \\[OSHT\\]
trip-zone enable/disable When any of the enabled pins go low, a one-shot trip event occurs for this ePWM module When the event occurs, the action defined in the TZCTL register is taken on the EPWMxA and EPWMxB outputs The one-shot trip condition remains latched until you clear the condition via the TZCLR register"]
pub type OshtnR = crate::FieldReader;
#[doc = "Field `OSHTN` writer - 15:8\\]
Trip-zone n \\[TZn\\]
select One-Shot \\[OSHT\\]
trip-zone enable/disable When any of the enabled pins go low, a one-shot trip event occurs for this ePWM module When the event occurs, the action defined in the TZCTL register is taken on the EPWMxA and EPWMxB outputs The one-shot trip condition remains latched until you clear the condition via the TZCLR register"]
pub type OshtnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Trip-zone n \\[TZn\\]
select Cycle-by-Cycle \\[CBC\\]
trip-zone enable/disable When any of the enabled pins go low, a cycle-by-cycle trip event occurs for this ePWM module When the event occurs, the action defined in the TZCTL register is taken on the EPWMxA and EPWMxB outputs A cycle-by-cycle trip condition is automatically cleared when the time-base counter reaches zero"]
    #[inline(always)]
    pub fn cbcn(&self) -> CbcnR {
        CbcnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Trip-zone n \\[TZn\\]
select One-Shot \\[OSHT\\]
trip-zone enable/disable When any of the enabled pins go low, a one-shot trip event occurs for this ePWM module When the event occurs, the action defined in the TZCTL register is taken on the EPWMxA and EPWMxB outputs The one-shot trip condition remains latched until you clear the condition via the TZCLR register"]
    #[inline(always)]
    pub fn oshtn(&self) -> OshtnR {
        OshtnR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Trip-zone n \\[TZn\\]
select Cycle-by-Cycle \\[CBC\\]
trip-zone enable/disable When any of the enabled pins go low, a cycle-by-cycle trip event occurs for this ePWM module When the event occurs, the action defined in the TZCTL register is taken on the EPWMxA and EPWMxB outputs A cycle-by-cycle trip condition is automatically cleared when the time-base counter reaches zero"]
    #[inline(always)]
    #[must_use]
    pub fn cbcn(&mut self) -> CbcnW<EpwmRegsTzselSpec> {
        CbcnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Trip-zone n \\[TZn\\]
select One-Shot \\[OSHT\\]
trip-zone enable/disable When any of the enabled pins go low, a one-shot trip event occurs for this ePWM module When the event occurs, the action defined in the TZCTL register is taken on the EPWMxA and EPWMxB outputs The one-shot trip condition remains latched until you clear the condition via the TZCLR register"]
    #[inline(always)]
    #[must_use]
    pub fn oshtn(&mut self) -> OshtnW<EpwmRegsTzselSpec> {
        OshtnW::new(self, 8)
    }
}
#[doc = "Trip Zone Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTzselSpec;
impl crate::RegisterSpec for EpwmRegsTzselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tzsel::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTzselSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tzsel::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTzselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TZSEL to value 0"]
impl crate::Resettable for EpwmRegsTzselSpec {
    const RESET_VALUE: u16 = 0;
}
