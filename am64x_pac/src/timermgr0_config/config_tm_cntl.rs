#[doc = "Register `CONFIG_TM_CNTL` reader"]
pub type R = crate::R<ConfigTmCntlSpec>;
#[doc = "Register `CONFIG_TM_CNTL` writer"]
pub type W = crate::W<ConfigTmCntlSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Enables the timer manager. When this bit is zero, the timers will all be halted and will not count"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Enables the timer manager. When this bit is zero, the timers will all be halted and will not count"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_TIMER` reader - 10:1\\]
The maximum timer that will be checked - e.g. if only using 512 timers, set this to 511. All timers above this number will be ignored. Should be set once during initialization"]
pub type MaxTimerR = crate::FieldReader<u16>;
#[doc = "Field `MAX_TIMER` writer - 10:1\\]
The maximum timer that will be checked - e.g. if only using 512 timers, set this to 511. All timers above this number will be ignored. Should be set once during initialization"]
pub type MaxTimerW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MASS_ENABLE` reader - 12:12\\]
Always reads zero. When a 1 is written to this bit, all timers from 0 to the TM_CNTL.max_timer will be enabled. Useful for initial programming, to not need to loop over every TIMER_CONTROL register to enable every timer if many or all are being used. This should only be used during initialization, or when TM_CNTL.enable is set to 0, as this does not set the timers, only enable them"]
pub type MassEnableR = crate::BitReader;
#[doc = "Field `MASS_ENABLE` writer - 12:12\\]
Always reads zero. When a 1 is written to this bit, all timers from 0 to the TM_CNTL.max_timer will be enabled. Useful for initial programming, to not need to loop over every TIMER_CONTROL register to enable every timer if many or all are being used. This should only be used during initialization, or when TM_CNTL.enable is set to 0, as this does not set the timers, only enable them"]
pub type MassEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the timer manager. When this bit is zero, the timers will all be halted and will not count"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - 10:1\\]
The maximum timer that will be checked - e.g. if only using 512 timers, set this to 511. All timers above this number will be ignored. Should be set once during initialization"]
    #[inline(always)]
    pub fn max_timer(&self) -> MaxTimerR {
        MaxTimerR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - 12:12\\]
Always reads zero. When a 1 is written to this bit, all timers from 0 to the TM_CNTL.max_timer will be enabled. Useful for initial programming, to not need to loop over every TIMER_CONTROL register to enable every timer if many or all are being used. This should only be used during initialization, or when TM_CNTL.enable is set to 0, as this does not set the timers, only enable them"]
    #[inline(always)]
    pub fn mass_enable(&self) -> MassEnableR {
        MassEnableR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the timer manager. When this bit is zero, the timers will all be halted and will not count"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ConfigTmCntlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 1:10 - 10:1\\]
The maximum timer that will be checked - e.g. if only using 512 timers, set this to 511. All timers above this number will be ignored. Should be set once during initialization"]
    #[inline(always)]
    #[must_use]
    pub fn max_timer(&mut self) -> MaxTimerW<ConfigTmCntlSpec> {
        MaxTimerW::new(self, 1)
    }
    #[doc = "Bit 12 - 12:12\\]
Always reads zero. When a 1 is written to this bit, all timers from 0 to the TM_CNTL.max_timer will be enabled. Useful for initial programming, to not need to loop over every TIMER_CONTROL register to enable every timer if many or all are being used. This should only be used during initialization, or when TM_CNTL.enable is set to 0, as this does not set the timers, only enable them"]
    #[inline(always)]
    #[must_use]
    pub fn mass_enable(&mut self) -> MassEnableW<ConfigTmCntlSpec> {
        MassEnableW::new(self, 12)
    }
}
#[doc = "This register controls the overall behavior of the timer manager module\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_tm_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_tm_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigTmCntlSpec;
impl crate::RegisterSpec for ConfigTmCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_tm_cntl::R`](R) reader structure"]
impl crate::Readable for ConfigTmCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`config_tm_cntl::W`](W) writer structure"]
impl crate::Writable for ConfigTmCntlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_TM_CNTL to value 0x2046"]
impl crate::Resettable for ConfigTmCntlSpec {
    const RESET_VALUE: u32 = 0x2046;
}
