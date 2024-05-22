#[doc = "Register `EPWM_REGS_CMPB` reader"]
pub type R = crate::R<EpwmRegsCmpbSpec>;
#[doc = "Register `EPWM_REGS_CMPB` writer"]
pub type W = crate::W<EpwmRegsCmpbSpec>;
#[doc = "Field `CMPB` reader - 15:0\\]
The value in the active CMPB register is continuously compared to the time-base counter \\[TBCNT\\]
When the values are equal, the counter-compare module generates a time-base counter equal to counter compare B event This event is sent to the action-qualifier where it is qualified and converted it into one or more actions These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers The actions that can be defined in the AQCTLA and AQCTLB registers include the following \\[a\\]
Do nothing, the event is ignored \\[b\\]
Clear: Pull the EPWMxA and/or EPWMxB signal low \\[c\\]
Set: Pull the EPWMxA and/or EPWMxB signal high \\[d\\]
Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWBMODE\\]
bit By default this register is shadowed \\[a\\]
If CMPCTL\\[SHDWBMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register In this case, the CMPCTL\\[LOADBMODE\\]
bit field determines which event will load the active register from the shadow register: \\[b\\]
Before a write, the CMPCTL\\[SHDWBFULL\\]
bit can be read to determine if the shadow register is currently full \\[c\\]
If CMPCTL\\[SHDWBMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware \\[d\\]
In either mode, the active and shadow registers share the same memory map address"]
pub type CmpbR = crate::FieldReader<u16>;
#[doc = "Field `CMPB` writer - 15:0\\]
The value in the active CMPB register is continuously compared to the time-base counter \\[TBCNT\\]
When the values are equal, the counter-compare module generates a time-base counter equal to counter compare B event This event is sent to the action-qualifier where it is qualified and converted it into one or more actions These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers The actions that can be defined in the AQCTLA and AQCTLB registers include the following \\[a\\]
Do nothing, the event is ignored \\[b\\]
Clear: Pull the EPWMxA and/or EPWMxB signal low \\[c\\]
Set: Pull the EPWMxA and/or EPWMxB signal high \\[d\\]
Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWBMODE\\]
bit By default this register is shadowed \\[a\\]
If CMPCTL\\[SHDWBMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register In this case, the CMPCTL\\[LOADBMODE\\]
bit field determines which event will load the active register from the shadow register: \\[b\\]
Before a write, the CMPCTL\\[SHDWBFULL\\]
bit can be read to determine if the shadow register is currently full \\[c\\]
If CMPCTL\\[SHDWBMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware \\[d\\]
In either mode, the active and shadow registers share the same memory map address"]
pub type CmpbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The value in the active CMPB register is continuously compared to the time-base counter \\[TBCNT\\]
When the values are equal, the counter-compare module generates a time-base counter equal to counter compare B event This event is sent to the action-qualifier where it is qualified and converted it into one or more actions These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers The actions that can be defined in the AQCTLA and AQCTLB registers include the following \\[a\\]
Do nothing, the event is ignored \\[b\\]
Clear: Pull the EPWMxA and/or EPWMxB signal low \\[c\\]
Set: Pull the EPWMxA and/or EPWMxB signal high \\[d\\]
Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWBMODE\\]
bit By default this register is shadowed \\[a\\]
If CMPCTL\\[SHDWBMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register In this case, the CMPCTL\\[LOADBMODE\\]
bit field determines which event will load the active register from the shadow register: \\[b\\]
Before a write, the CMPCTL\\[SHDWBFULL\\]
bit can be read to determine if the shadow register is currently full \\[c\\]
If CMPCTL\\[SHDWBMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware \\[d\\]
In either mode, the active and shadow registers share the same memory map address"]
    #[inline(always)]
    pub fn cmpb(&self) -> CmpbR {
        CmpbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The value in the active CMPB register is continuously compared to the time-base counter \\[TBCNT\\]
When the values are equal, the counter-compare module generates a time-base counter equal to counter compare B event This event is sent to the action-qualifier where it is qualified and converted it into one or more actions These actions can be applied to either the EPWMxA or the EPWMxB output depending on the configuration of the AQCTLA and AQCTLB registers The actions that can be defined in the AQCTLA and AQCTLB registers include the following \\[a\\]
Do nothing, the event is ignored \\[b\\]
Clear: Pull the EPWMxA and/or EPWMxB signal low \\[c\\]
Set: Pull the EPWMxA and/or EPWMxB signal high \\[d\\]
Toggle the EPWMxA and/or EPWMxB signal Shadowing of this register is enabled and disabled by the CMPCTL\\[SHDWBMODE\\]
bit By default this register is shadowed \\[a\\]
If CMPCTL\\[SHDWBMODE\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register In this case, the CMPCTL\\[LOADBMODE\\]
bit field determines which event will load the active register from the shadow register: \\[b\\]
Before a write, the CMPCTL\\[SHDWBFULL\\]
bit can be read to determine if the shadow register is currently full \\[c\\]
If CMPCTL\\[SHDWBMODE\\]
= 1, then the shadow register is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware \\[d\\]
In either mode, the active and shadow registers share the same memory map address"]
    #[inline(always)]
    #[must_use]
    pub fn cmpb(&mut self) -> CmpbW<EpwmRegsCmpbSpec> {
        CmpbW::new(self, 0)
    }
}
#[doc = "Counter Compare B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_cmpb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_cmpb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsCmpbSpec;
impl crate::RegisterSpec for EpwmRegsCmpbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_cmpb::R`](R) reader structure"]
impl crate::Readable for EpwmRegsCmpbSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_cmpb::W`](W) writer structure"]
impl crate::Writable for EpwmRegsCmpbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_CMPB to value 0"]
impl crate::Resettable for EpwmRegsCmpbSpec {
    const RESET_VALUE: u16 = 0;
}
