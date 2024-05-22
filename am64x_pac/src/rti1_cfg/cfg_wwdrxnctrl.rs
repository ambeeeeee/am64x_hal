#[doc = "Register `CFG_WWDRXNCTRL` reader"]
pub type R = crate::R<CfgWwdrxnctrlSpec>;
#[doc = "Register `CFG_WWDRXNCTRL` writer"]
pub type W = crate::W<CfgWwdrxnctrlSpec>;
#[doc = "Field `WWDRXN` reader - 3:0\\]
User and privilege mode (read), privileged mode (write): 0x5 = This is the default value. The windowed watchdog will cause a reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. 0xA = The windowed watchdog will generate a non-maskable interrupt to the CPU if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Writing any other value will cause a system reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Note: Configuration of DWWD Reaction The DWWD reaction can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDRXN is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDRXN is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
pub type WwdrxnR = crate::FieldReader;
#[doc = "Field `WWDRXN` writer - 3:0\\]
User and privilege mode (read), privileged mode (write): 0x5 = This is the default value. The windowed watchdog will cause a reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. 0xA = The windowed watchdog will generate a non-maskable interrupt to the CPU if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Writing any other value will cause a system reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Note: Configuration of DWWD Reaction The DWWD reaction can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDRXN is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDRXN is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
pub type WwdrxnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
User and privilege mode (read), privileged mode (write): 0x5 = This is the default value. The windowed watchdog will cause a reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. 0xA = The windowed watchdog will generate a non-maskable interrupt to the CPU if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Writing any other value will cause a system reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Note: Configuration of DWWD Reaction The DWWD reaction can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDRXN is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDRXN is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
    #[inline(always)]
    pub fn wwdrxn(&self) -> WwdrxnR {
        WwdrxnR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
User and privilege mode (read), privileged mode (write): 0x5 = This is the default value. The windowed watchdog will cause a reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. 0xA = The windowed watchdog will generate a non-maskable interrupt to the CPU if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Writing any other value will cause a system reset if the watchdog is serviced outside the time window defined by the configuration, or if the watchdog is not serviced at all. Note: Configuration of DWWD Reaction The DWWD reaction can be selected by the application even when the DWWD counter is already enabled. If a change to the WWDRXN is made before the watchdog service window is opened, then the change in the configuration takes effect immediately. If a change to the WWDRXN is made when the watchdog service window is already open, then the change in configuration takes effect only after the watchdog is serviced."]
    #[inline(always)]
    #[must_use]
    pub fn wwdrxn(&mut self) -> WwdrxnW<CfgWwdrxnctrlSpec> {
        WwdrxnW::new(self, 0)
    }
}
#[doc = "CFG_WWDRXNCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_wwdrxnctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_wwdrxnctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgWwdrxnctrlSpec;
impl crate::RegisterSpec for CfgWwdrxnctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_wwdrxnctrl::R`](R) reader structure"]
impl crate::Readable for CfgWwdrxnctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_wwdrxnctrl::W`](W) writer structure"]
impl crate::Writable for CfgWwdrxnctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_WWDRXNCTRL to value 0x05"]
impl crate::Resettable for CfgWwdrxnctrlSpec {
    const RESET_VALUE: u32 = 0x05;
}
