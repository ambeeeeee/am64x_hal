#[doc = "Register `EPWM_REGS_TBCTL` reader"]
pub type R = crate::R<EpwmRegsTbctlSpec>;
#[doc = "Register `EPWM_REGS_TBCTL` writer"]
pub type W = crate::W<EpwmRegsTbctlSpec>;
#[doc = "Field `CTRMODE` reader - 1:0\\]
Counter Mode The time-base counter mode is normally configured once and not changed during normal operation If you change the mode of the counter, the change will take effect at the next TBCLK edge and the current counter value shall increment or decrement from the value before the mode change These bits set the time-base counter mode of operation as follows:"]
pub type CtrmodeR = crate::FieldReader;
#[doc = "Field `CTRMODE` writer - 1:0\\]
Counter Mode The time-base counter mode is normally configured once and not changed during normal operation If you change the mode of the counter, the change will take effect at the next TBCLK edge and the current counter value shall increment or decrement from the value before the mode change These bits set the time-base counter mode of operation as follows:"]
pub type CtrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHSEN` reader - 2:2\\]
Counter Register Load From Phase Register Enable"]
pub type PhsenR = crate::BitReader;
#[doc = "Field `PHSEN` writer - 2:2\\]
Counter Register Load From Phase Register Enable"]
pub type PhsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDLD` reader - 3:3\\]
Active Period Register Load From Shadow Register Select"]
pub type PrdldR = crate::BitReader;
#[doc = "Field `PRDLD` writer - 3:3\\]
Active Period Register Load From Shadow Register Select"]
pub type PrdldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCOSEL` reader - 5:4\\]
Synchronization Output Select These bits select the source of the EPWMxSYNCO signal"]
pub type SyncoselR = crate::FieldReader;
#[doc = "Field `SYNCOSEL` writer - 5:4\\]
Synchronization Output Select These bits select the source of the EPWMxSYNCO signal"]
pub type SyncoselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWFSYNC` reader - 6:6\\]
Software Forced Synchronization Pulse"]
pub type SwfsyncR = crate::BitReader;
#[doc = "Field `SWFSYNC` writer - 6:6\\]
Software Forced Synchronization Pulse"]
pub type SwfsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPCLKDIV` reader - 9:7\\]
High-Speed Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value TBCLK = SYSCLKOUT/\\[HSPCLKDIV - CLKDIV\\]
This divisor emulates the HSPCLK in the TMS320x281x system as used on the Event Manager \\[EV\\]
peripheral"]
pub type HspclkdivR = crate::FieldReader;
#[doc = "Field `HSPCLKDIV` writer - 9:7\\]
High-Speed Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value TBCLK = SYSCLKOUT/\\[HSPCLKDIV - CLKDIV\\]
This divisor emulates the HSPCLK in the TMS320x281x system as used on the Event Manager \\[EV\\]
peripheral"]
pub type HspclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLKDIV` reader - 12:10\\]
Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value TBCLK = SYSCLKOUT/\\[HSPCLKDIV - CLKDIV\\]"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - 12:10\\]
Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value TBCLK = SYSCLKOUT/\\[HSPCLKDIV - CLKDIV\\]"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHSDIR` reader - 13:13\\]
Phase Direction Bit This bit is only used when the time-base counter is configured in the up-down-count mode The PHSDIR bit indicates the direction the time-base counter \\[TBCNT\\]
will count after a synchronization event occurs and a new phase value is loaded from the phase \\[TBPHS\\]
register This is irrespective of the direction of the counter before the synchronization event In the up-count and down-count modes this bit is ignored"]
pub type PhsdirR = crate::BitReader;
#[doc = "Field `PHSDIR` writer - 13:13\\]
Phase Direction Bit This bit is only used when the time-base counter is configured in the up-down-count mode The PHSDIR bit indicates the direction the time-base counter \\[TBCNT\\]
will count after a synchronization event occurs and a new phase value is loaded from the phase \\[TBPHS\\]
register This is irrespective of the direction of the counter before the synchronization event In the up-count and down-count modes this bit is ignored"]
pub type PhsdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREE_SOFT` reader - 15:14\\]
Emulation Mode Bits These bits select the behavior of the ePWM time-base counter during emulation events:"]
pub type FreeSoftR = crate::FieldReader;
#[doc = "Field `FREE_SOFT` writer - 15:14\\]
Emulation Mode Bits These bits select the behavior of the ePWM time-base counter during emulation events:"]
pub type FreeSoftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Counter Mode The time-base counter mode is normally configured once and not changed during normal operation If you change the mode of the counter, the change will take effect at the next TBCLK edge and the current counter value shall increment or decrement from the value before the mode change These bits set the time-base counter mode of operation as follows:"]
    #[inline(always)]
    pub fn ctrmode(&self) -> CtrmodeR {
        CtrmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Counter Register Load From Phase Register Enable"]
    #[inline(always)]
    pub fn phsen(&self) -> PhsenR {
        PhsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Active Period Register Load From Shadow Register Select"]
    #[inline(always)]
    pub fn prdld(&self) -> PrdldR {
        PrdldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Synchronization Output Select These bits select the source of the EPWMxSYNCO signal"]
    #[inline(always)]
    pub fn syncosel(&self) -> SyncoselR {
        SyncoselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Software Forced Synchronization Pulse"]
    #[inline(always)]
    pub fn swfsync(&self) -> SwfsyncR {
        SwfsyncR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - 9:7\\]
High-Speed Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value TBCLK = SYSCLKOUT/\\[HSPCLKDIV - CLKDIV\\]
This divisor emulates the HSPCLK in the TMS320x281x system as used on the Event Manager \\[EV\\]
peripheral"]
    #[inline(always)]
    pub fn hspclkdiv(&self) -> HspclkdivR {
        HspclkdivR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value TBCLK = SYSCLKOUT/\\[HSPCLKDIV - CLKDIV\\]"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Phase Direction Bit This bit is only used when the time-base counter is configured in the up-down-count mode The PHSDIR bit indicates the direction the time-base counter \\[TBCNT\\]
will count after a synchronization event occurs and a new phase value is loaded from the phase \\[TBPHS\\]
register This is irrespective of the direction of the counter before the synchronization event In the up-count and down-count modes this bit is ignored"]
    #[inline(always)]
    pub fn phsdir(&self) -> PhsdirR {
        PhsdirR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Emulation Mode Bits These bits select the behavior of the ePWM time-base counter during emulation events:"]
    #[inline(always)]
    pub fn free_soft(&self) -> FreeSoftR {
        FreeSoftR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Counter Mode The time-base counter mode is normally configured once and not changed during normal operation If you change the mode of the counter, the change will take effect at the next TBCLK edge and the current counter value shall increment or decrement from the value before the mode change These bits set the time-base counter mode of operation as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn ctrmode(&mut self) -> CtrmodeW<EpwmRegsTbctlSpec> {
        CtrmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Counter Register Load From Phase Register Enable"]
    #[inline(always)]
    #[must_use]
    pub fn phsen(&mut self) -> PhsenW<EpwmRegsTbctlSpec> {
        PhsenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Active Period Register Load From Shadow Register Select"]
    #[inline(always)]
    #[must_use]
    pub fn prdld(&mut self) -> PrdldW<EpwmRegsTbctlSpec> {
        PrdldW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Synchronization Output Select These bits select the source of the EPWMxSYNCO signal"]
    #[inline(always)]
    #[must_use]
    pub fn syncosel(&mut self) -> SyncoselW<EpwmRegsTbctlSpec> {
        SyncoselW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Software Forced Synchronization Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn swfsync(&mut self) -> SwfsyncW<EpwmRegsTbctlSpec> {
        SwfsyncW::new(self, 6)
    }
    #[doc = "Bits 7:9 - 9:7\\]
High-Speed Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value TBCLK = SYSCLKOUT/\\[HSPCLKDIV - CLKDIV\\]
This divisor emulates the HSPCLK in the TMS320x281x system as used on the Event Manager \\[EV\\]
peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn hspclkdiv(&mut self) -> HspclkdivW<EpwmRegsTbctlSpec> {
        HspclkdivW::new(self, 7)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Time-base Clock Prescale Bits These bits determine part of the time-base clock prescale value TBCLK = SYSCLKOUT/\\[HSPCLKDIV - CLKDIV\\]"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<EpwmRegsTbctlSpec> {
        ClkdivW::new(self, 10)
    }
    #[doc = "Bit 13 - 13:13\\]
Phase Direction Bit This bit is only used when the time-base counter is configured in the up-down-count mode The PHSDIR bit indicates the direction the time-base counter \\[TBCNT\\]
will count after a synchronization event occurs and a new phase value is loaded from the phase \\[TBPHS\\]
register This is irrespective of the direction of the counter before the synchronization event In the up-count and down-count modes this bit is ignored"]
    #[inline(always)]
    #[must_use]
    pub fn phsdir(&mut self) -> PhsdirW<EpwmRegsTbctlSpec> {
        PhsdirW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Emulation Mode Bits These bits select the behavior of the ePWM time-base counter during emulation events:"]
    #[inline(always)]
    #[must_use]
    pub fn free_soft(&mut self) -> FreeSoftW<EpwmRegsTbctlSpec> {
        FreeSoftW::new(self, 14)
    }
}
#[doc = "Time-Base Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsTbctlSpec;
impl crate::RegisterSpec for EpwmRegsTbctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_tbctl::R`](R) reader structure"]
impl crate::Readable for EpwmRegsTbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_tbctl::W`](W) writer structure"]
impl crate::Writable for EpwmRegsTbctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_TBCTL to value 0x83"]
impl crate::Resettable for EpwmRegsTbctlSpec {
    const RESET_VALUE: u16 = 0x83;
}
