#[doc = "Register `CFG_WWDSIZECTRL` reader"]
pub type R = crate::R<CfgWwdsizectrlSpec>;
#[doc = "Register `CFG_WWDSIZECTRL` writer"]
pub type W = crate::W<CfgWwdsizectrlSpec>;
#[doc = "Field `WWDSIZE` reader - 31:0\\]
User and privilege mode (read), privileged mode (write): Table 3. Windowed Watchdog Window Size Configuration Value written to WWDSIZE Window Size 0x00000005 100% (The functionality is the same as the standard time-out digital watchdog.) 0x00000050 50% 0x00000500 25% 0x00005000 12.5% 0x00050000 6.25% 0x00500000 3.125% Any other 3.125% value Note: Incorrect value being written to watchdog window size control register If an incorerct value is written to the WWDSIZE field, or if a system disturbance causes the WWDSIZE field to have a value other than 0x5, 0x50, 0x500, 0x5000, 0x50000, or 0x500000, then the window size will be configured to be 3.125%. This increases the chances of getting a reset due to the windowed watchdog, which enables the system to handle the cause for the incorrect configuration. Note: Configuration of DWWD Window Size The DWWD window size can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDSIZE is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDSIZE is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
pub type WwdsizeR = crate::FieldReader<u32>;
#[doc = "Field `WWDSIZE` writer - 31:0\\]
User and privilege mode (read), privileged mode (write): Table 3. Windowed Watchdog Window Size Configuration Value written to WWDSIZE Window Size 0x00000005 100% (The functionality is the same as the standard time-out digital watchdog.) 0x00000050 50% 0x00000500 25% 0x00005000 12.5% 0x00050000 6.25% 0x00500000 3.125% Any other 3.125% value Note: Incorrect value being written to watchdog window size control register If an incorerct value is written to the WWDSIZE field, or if a system disturbance causes the WWDSIZE field to have a value other than 0x5, 0x50, 0x500, 0x5000, 0x50000, or 0x500000, then the window size will be configured to be 3.125%. This increases the chances of getting a reset due to the windowed watchdog, which enables the system to handle the cause for the incorrect configuration. Note: Configuration of DWWD Window Size The DWWD window size can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDSIZE is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDSIZE is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
pub type WwdsizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
User and privilege mode (read), privileged mode (write): Table 3. Windowed Watchdog Window Size Configuration Value written to WWDSIZE Window Size 0x00000005 100% (The functionality is the same as the standard time-out digital watchdog.) 0x00000050 50% 0x00000500 25% 0x00005000 12.5% 0x00050000 6.25% 0x00500000 3.125% Any other 3.125% value Note: Incorrect value being written to watchdog window size control register If an incorerct value is written to the WWDSIZE field, or if a system disturbance causes the WWDSIZE field to have a value other than 0x5, 0x50, 0x500, 0x5000, 0x50000, or 0x500000, then the window size will be configured to be 3.125%. This increases the chances of getting a reset due to the windowed watchdog, which enables the system to handle the cause for the incorrect configuration. Note: Configuration of DWWD Window Size The DWWD window size can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDSIZE is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDSIZE is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
    #[inline(always)]
    pub fn wwdsize(&self) -> WwdsizeR {
        WwdsizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
User and privilege mode (read), privileged mode (write): Table 3. Windowed Watchdog Window Size Configuration Value written to WWDSIZE Window Size 0x00000005 100% (The functionality is the same as the standard time-out digital watchdog.) 0x00000050 50% 0x00000500 25% 0x00005000 12.5% 0x00050000 6.25% 0x00500000 3.125% Any other 3.125% value Note: Incorrect value being written to watchdog window size control register If an incorerct value is written to the WWDSIZE field, or if a system disturbance causes the WWDSIZE field to have a value other than 0x5, 0x50, 0x500, 0x5000, 0x50000, or 0x500000, then the window size will be configured to be 3.125%. This increases the chances of getting a reset due to the windowed watchdog, which enables the system to handle the cause for the incorrect configuration. Note: Configuration of DWWD Window Size The DWWD window size can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDSIZE is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDSIZE is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
    #[inline(always)]
    #[must_use]
    pub fn wwdsize(&mut self) -> WwdsizeW<CfgWwdsizectrlSpec> {
        WwdsizeW::new(self, 0)
    }
}
#[doc = "CFG_WWDSIZECTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_wwdsizectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_wwdsizectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgWwdsizectrlSpec;
impl crate::RegisterSpec for CfgWwdsizectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_wwdsizectrl::R`](R) reader structure"]
impl crate::Readable for CfgWwdsizectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_wwdsizectrl::W`](W) writer structure"]
impl crate::Writable for CfgWwdsizectrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_WWDSIZECTRL to value 0x05"]
impl crate::Resettable for CfgWwdsizectrlSpec {
    const RESET_VALUE: u32 = 0x05;
}
