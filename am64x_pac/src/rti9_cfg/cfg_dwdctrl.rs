#[doc = "Register `CFG_DWDCTRL` reader"]
pub type R = crate::R<CfgDwdctrlSpec>;
#[doc = "Register `CFG_DWDCTRL` writer"]
pub type W = crate::W<CfgDwdctrlSpec>;
#[doc = "Field `DWDCTRL` reader - 31:0\\]
User and priviledge mode (read): 0x5312ACED = DWD counter is disabled. This is the default value. 0xA98559DA = DWD counter is enabled Any other value = DWD counter state is unchanged (enabled or disabled) Priviledge mode (write): 0xA98559DA = DWD counter is enabled Any other value = State of DWD counter is unchanged (stays enabled or disabled) Note: One-Write Functionality of DWDCTRL Register The RTIDWDCTRL register implements a one-write functionality, such that the application cannot write to this registermore than once. Writing the default value will not enable the watchdog as described above. Writing the enable value will start the watchdog counters. A write to RTIDWDCTRL will only be enabled after a system reset again."]
pub type DwdctrlR = crate::FieldReader<u32>;
#[doc = "Field `DWDCTRL` writer - 31:0\\]
User and priviledge mode (read): 0x5312ACED = DWD counter is disabled. This is the default value. 0xA98559DA = DWD counter is enabled Any other value = DWD counter state is unchanged (enabled or disabled) Priviledge mode (write): 0xA98559DA = DWD counter is enabled Any other value = State of DWD counter is unchanged (stays enabled or disabled) Note: One-Write Functionality of DWDCTRL Register The RTIDWDCTRL register implements a one-write functionality, such that the application cannot write to this registermore than once. Writing the default value will not enable the watchdog as described above. Writing the enable value will start the watchdog counters. A write to RTIDWDCTRL will only be enabled after a system reset again."]
pub type DwdctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
User and priviledge mode (read): 0x5312ACED = DWD counter is disabled. This is the default value. 0xA98559DA = DWD counter is enabled Any other value = DWD counter state is unchanged (enabled or disabled) Priviledge mode (write): 0xA98559DA = DWD counter is enabled Any other value = State of DWD counter is unchanged (stays enabled or disabled) Note: One-Write Functionality of DWDCTRL Register The RTIDWDCTRL register implements a one-write functionality, such that the application cannot write to this registermore than once. Writing the default value will not enable the watchdog as described above. Writing the enable value will start the watchdog counters. A write to RTIDWDCTRL will only be enabled after a system reset again."]
    #[inline(always)]
    pub fn dwdctrl(&self) -> DwdctrlR {
        DwdctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
User and priviledge mode (read): 0x5312ACED = DWD counter is disabled. This is the default value. 0xA98559DA = DWD counter is enabled Any other value = DWD counter state is unchanged (enabled or disabled) Priviledge mode (write): 0xA98559DA = DWD counter is enabled Any other value = State of DWD counter is unchanged (stays enabled or disabled) Note: One-Write Functionality of DWDCTRL Register The RTIDWDCTRL register implements a one-write functionality, such that the application cannot write to this registermore than once. Writing the default value will not enable the watchdog as described above. Writing the enable value will start the watchdog counters. A write to RTIDWDCTRL will only be enabled after a system reset again."]
    #[inline(always)]
    #[must_use]
    pub fn dwdctrl(&mut self) -> DwdctrlW<CfgDwdctrlSpec> {
        DwdctrlW::new(self, 0)
    }
}
#[doc = "CFG_DWDCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dwdctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dwdctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDwdctrlSpec;
impl crate::RegisterSpec for CfgDwdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dwdctrl::R`](R) reader structure"]
impl crate::Readable for CfgDwdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dwdctrl::W`](W) writer structure"]
impl crate::Writable for CfgDwdctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DWDCTRL to value 0"]
impl crate::Resettable for CfgDwdctrlSpec {
    const RESET_VALUE: u32 = 0;
}
