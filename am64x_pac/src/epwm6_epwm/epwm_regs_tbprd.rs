#[doc = "Register `EPWM_REGS_TBPRD` reader"]
pub type R = crate::R<EpwmRegsTbprdSpec>;
#[doc = "Register `EPWM_REGS_TBPRD` writer"]
pub type W = crate::W<EpwmRegsTbprdSpec>;
#[doc = "Field `TBPRD` reader - 15:0\\]
These bits determine the period of the time-base counter This sets the PWM frequency Shadowing of this register is enabled and disabled by the TBCTL\\[PRDLD\\]
bit By default this register is shadowed \\[a\\]
If TBCTL\\[PRDLD\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register In this case, the active register will be loaded from the shadow register when the time-base counter equals zero \\[b\\]
If TBCTL\\[PRDLD\\]
= 1, then the shadow is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware \\[c\\]
The active and shadow registers share the same memory map address"]
pub type TbprdR = crate::FieldReader<u16>;
#[doc = "Field `TBPRD` writer - 15:0\\]
These bits determine the period of the time-base counter This sets the PWM frequency Shadowing of this register is enabled and disabled by the TBCTL\\[PRDLD\\]
bit By default this register is shadowed \\[a\\]
If TBCTL\\[PRDLD\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register In this case, the active register will be loaded from the shadow register when the time-base counter equals zero \\[b\\]
If TBCTL\\[PRDLD\\]
= 1, then the shadow is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware \\[c\\]
The active and shadow registers share the same memory map address"]
pub type TbprdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
These bits determine the period of the time-base counter This sets the PWM frequency Shadowing of this register is enabled and disabled by the TBCTL\\[PRDLD\\]
bit By default this register is shadowed \\[a\\]
If TBCTL\\[PRDLD\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register In this case, the active register will be loaded from the shadow register when the time-base counter equals zero \\[b\\]
If TBCTL\\[PRDLD\\]
= 1, then the shadow is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware \\[c\\]
The active and shadow registers share the same memory map address"]
    #[inline(always)]
    pub fn tbprd(&self) -> TbprdR {
        TbprdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
These bits determine the period of the time-base counter This sets the PWM frequency Shadowing of this register is enabled and disabled by the TBCTL\\[PRDLD\\]
bit By default this register is shadowed \\[a\\]
If TBCTL\\[PRDLD\\]
= 0, then the shadow is enabled and any write or read will automatically go to the shadow register In this case, the active register will be loaded from the shadow register when the time-base counter equals zero \\[b\\]
If TBCTL\\[PRDLD\\]
= 1, then the shadow is disabled and any write or read will go directly to the active register, that is the register actively controlling the hardware \\[c\\]
The active and shadow registers share the same memory map address"]
    #[inline(always)]
    #[must_use]
    pub fn tbprd(&mut self) -> TbprdW<EpwmRegsTbprdSpec> {
        TbprdW::new(self, 0)
    }
}
#[doc = "Time Base Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbprd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbprd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTbprdSpec;
impl crate::RegisterSpec for EpwmRegsTbprdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tbprd::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTbprdSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tbprd::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTbprdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TBPRD to value 0"]
impl crate::Resettable for EpwmRegsTbprdSpec {
    const RESET_VALUE: u16 = 0;
}
