#[doc = "Register `EPWM_REGS_ETPS` reader"]
pub type R = crate::R<EpwmRegsEtpsSpec>;
#[doc = "Register `EPWM_REGS_ETPS` writer"]
pub type W = crate::W<EpwmRegsEtpsSpec>;
#[doc = "Field `INTPRD` reader - 1:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Period Select These bits determine how many selected ETSEL\\[INTSEL\\]
events need to occur before an interrupt is generated To be generated, the interrupt must be enabled \\[ETSEL\\[INT\\]
= 1\\]
If the interrupt status flag is set from a previous interrupt \\[ETFLG\\[INT\\]
= 1\\]
then no interrupt will be generated until the flag is cleared via the ETCLR\\[INT\\]
bit This allows for one interrupt to be pending while another is still being serviced Once the interrupt is generated, the ETPS\\[INTCNT\\]
bits will automatically be cleared Writing a INTPRD value that is the same as the current counter value will trigger an interrupt if it is enabled and the status flag is clear Writing a INTPRD value that is less than the current counter value will result in an undefined state If a counter event occurs at the same instant as a new zero or non-zero INTPRD value is written, the counter is incremented"]
pub type IntprdR = crate::FieldReader;
#[doc = "Field `INTPRD` writer - 1:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Period Select These bits determine how many selected ETSEL\\[INTSEL\\]
events need to occur before an interrupt is generated To be generated, the interrupt must be enabled \\[ETSEL\\[INT\\]
= 1\\]
If the interrupt status flag is set from a previous interrupt \\[ETFLG\\[INT\\]
= 1\\]
then no interrupt will be generated until the flag is cleared via the ETCLR\\[INT\\]
bit This allows for one interrupt to be pending while another is still being serviced Once the interrupt is generated, the ETPS\\[INTCNT\\]
bits will automatically be cleared Writing a INTPRD value that is the same as the current counter value will trigger an interrupt if it is enabled and the status flag is clear Writing a INTPRD value that is less than the current counter value will result in an undefined state If a counter event occurs at the same instant as a new zero or non-zero INTPRD value is written, the counter is incremented"]
pub type IntprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTCNT` reader - 3:2\\]
ePWM Interrupt Event \\[EPWMx_INT\\]
Counter Register These bits indicate how many selected ETSEL\\[INTSEL\\]
events have occurred These bits are automatically cleared when an interrupt pulse is generated If interrupts are disabled, ETSEL\\[INT\\]
= 0 or the interrupt flag is set, ETFLG\\[INT\\]
= 1, the counter will stop counting events when it reaches the period value ETPS\\[INTCNT\\]
= ETPS\\[INTPRD\\]"]
pub type IntcntR = crate::FieldReader;
#[doc = "Field `INTCNT` writer - 3:2\\]
ePWM Interrupt Event \\[EPWMx_INT\\]
Counter Register These bits indicate how many selected ETSEL\\[INTSEL\\]
events have occurred These bits are automatically cleared when an interrupt pulse is generated If interrupts are disabled, ETSEL\\[INT\\]
= 0 or the interrupt flag is set, ETFLG\\[INT\\]
= 1, the counter will stop counting events when it reaches the period value ETPS\\[INTCNT\\]
= ETPS\\[INTPRD\\]"]
pub type IntcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Period Select These bits determine how many selected ETSEL\\[INTSEL\\]
events need to occur before an interrupt is generated To be generated, the interrupt must be enabled \\[ETSEL\\[INT\\]
= 1\\]
If the interrupt status flag is set from a previous interrupt \\[ETFLG\\[INT\\]
= 1\\]
then no interrupt will be generated until the flag is cleared via the ETCLR\\[INT\\]
bit This allows for one interrupt to be pending while another is still being serviced Once the interrupt is generated, the ETPS\\[INTCNT\\]
bits will automatically be cleared Writing a INTPRD value that is the same as the current counter value will trigger an interrupt if it is enabled and the status flag is clear Writing a INTPRD value that is less than the current counter value will result in an undefined state If a counter event occurs at the same instant as a new zero or non-zero INTPRD value is written, the counter is incremented"]
    #[inline(always)]
    pub fn intprd(&self) -> IntprdR {
        IntprdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
ePWM Interrupt Event \\[EPWMx_INT\\]
Counter Register These bits indicate how many selected ETSEL\\[INTSEL\\]
events have occurred These bits are automatically cleared when an interrupt pulse is generated If interrupts are disabled, ETSEL\\[INT\\]
= 0 or the interrupt flag is set, ETFLG\\[INT\\]
= 1, the counter will stop counting events when it reaches the period value ETPS\\[INTCNT\\]
= ETPS\\[INTPRD\\]"]
    #[inline(always)]
    pub fn intcnt(&self) -> IntcntR {
        IntcntR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Period Select These bits determine how many selected ETSEL\\[INTSEL\\]
events need to occur before an interrupt is generated To be generated, the interrupt must be enabled \\[ETSEL\\[INT\\]
= 1\\]
If the interrupt status flag is set from a previous interrupt \\[ETFLG\\[INT\\]
= 1\\]
then no interrupt will be generated until the flag is cleared via the ETCLR\\[INT\\]
bit This allows for one interrupt to be pending while another is still being serviced Once the interrupt is generated, the ETPS\\[INTCNT\\]
bits will automatically be cleared Writing a INTPRD value that is the same as the current counter value will trigger an interrupt if it is enabled and the status flag is clear Writing a INTPRD value that is less than the current counter value will result in an undefined state If a counter event occurs at the same instant as a new zero or non-zero INTPRD value is written, the counter is incremented"]
    #[inline(always)]
    #[must_use]
    pub fn intprd(&mut self) -> IntprdW<EpwmRegsEtpsSpec> {
        IntprdW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
ePWM Interrupt Event \\[EPWMx_INT\\]
Counter Register These bits indicate how many selected ETSEL\\[INTSEL\\]
events have occurred These bits are automatically cleared when an interrupt pulse is generated If interrupts are disabled, ETSEL\\[INT\\]
= 0 or the interrupt flag is set, ETFLG\\[INT\\]
= 1, the counter will stop counting events when it reaches the period value ETPS\\[INTCNT\\]
= ETPS\\[INTPRD\\]"]
    #[inline(always)]
    #[must_use]
    pub fn intcnt(&mut self) -> IntcntW<EpwmRegsEtpsSpec> {
        IntcntW::new(self, 2)
    }
}
#[doc = "Event Trigger Pre-Scale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsEtpsSpec;
impl crate::RegisterSpec for EpwmRegsEtpsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_etps::R`](R) reader structure"]
impl crate::Readable for EpwmRegsEtpsSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_etps::W`](W) writer structure"]
impl crate::Writable for EpwmRegsEtpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_ETPS to value 0"]
impl crate::Resettable for EpwmRegsEtpsSpec {
    const RESET_VALUE: u16 = 0;
}
