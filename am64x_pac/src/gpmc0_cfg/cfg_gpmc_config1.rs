#[doc = "Register `CFG_GPMC_CONFIG1` reader"]
pub type R = crate::R<CfgGpmcConfig1Spec>;
#[doc = "Register `CFG_GPMC_CONFIG1` writer"]
pub type W = crate::W<CfgGpmcConfig1Spec>;
#[doc = "Field `GPMCFCLKDIVIDER` reader - 1:0\\]
Divides the GPMC.FCLK clock"]
pub type GpmcfclkdividerR = crate::FieldReader;
#[doc = "Field `GPMCFCLKDIVIDER` writer - 1:0\\]
Divides the GPMC.FCLK clock"]
pub type GpmcfclkdividerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMEPARAGRANULARITY` reader - 4:4\\]
Signals timing latencies scalar factor \\[Rd/WRCycleTime, AccessTime, PageBurstAccessTime, CSOnTime, CSRd/WrOffTime, ADVOnTime, ADVRd/WrOffTime, OEOnTime, OEOffTime, WEOnTime, WEOffTime, Cycle2CycleDelay, BusTurnAround, TimeOutStartValue\\]"]
pub type TimeparagranularityR = crate::BitReader;
#[doc = "Field `TIMEPARAGRANULARITY` writer - 4:4\\]
Signals timing latencies scalar factor \\[Rd/WRCycleTime, AccessTime, PageBurstAccessTime, CSOnTime, CSRd/WrOffTime, ADVOnTime, ADVRd/WrOffTime, OEOnTime, OEOffTime, WEOnTime, WEOffTime, Cycle2CycleDelay, BusTurnAround, TimeOutStartValue\\]"]
pub type TimeparagranularityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUXADDDATA` reader - 9:8\\]
Enables the Address and data multiplexed protocol \\[Reset value is CS0MUXDEVICE input pin sampled at IC reset for CS0 and 0 for CS1-7\\]"]
pub type MuxadddataR = crate::FieldReader;
#[doc = "Field `MUXADDDATA` writer - 9:8\\]
Enables the Address and data multiplexed protocol \\[Reset value is CS0MUXDEVICE input pin sampled at IC reset for CS0 and 0 for CS1-7\\]"]
pub type MuxadddataW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVICETYPE` reader - 11:10\\]
Selects the attached device type"]
pub type DevicetypeR = crate::FieldReader;
#[doc = "Field `DEVICETYPE` writer - 11:10\\]
Selects the attached device type"]
pub type DevicetypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVICESIZE` reader - 13:12\\]
Selects the device size attached \\[Reset value is BOOTDEVICESIZE input pin sampled at IC reset for CS0 and 01 for CS1-7\\]"]
pub type DevicesizeR = crate::FieldReader;
#[doc = "Field `DEVICESIZE` writer - 13:12\\]
Selects the device size attached \\[Reset value is BOOTDEVICESIZE input pin sampled at IC reset for CS0 and 01 for CS1-7\\]"]
pub type DevicesizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAITPINSELECT` reader - 17:16\\]
Selects the input WAIT pin for this chip select \\[Reset value is BOOTWAITSELECT input pin sampled at IC reset for CS0 and 0 for CS1-7\\]"]
pub type WaitpinselectR = crate::FieldReader;
#[doc = "Field `WAITPINSELECT` writer - 17:16\\]
Selects the input WAIT pin for this chip select \\[Reset value is BOOTWAITSELECT input pin sampled at IC reset for CS0 and 0 for CS1-7\\]"]
pub type WaitpinselectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAITMONITORINGTIME` reader - 19:18\\]
Selects input pin Wait monitoring time"]
pub type WaitmonitoringtimeR = crate::FieldReader;
#[doc = "Field `WAITMONITORINGTIME` writer - 19:18\\]
Selects input pin Wait monitoring time"]
pub type WaitmonitoringtimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAITWRITEMONITORING` reader - 21:21\\]
Selects the Wait monitoring configuration for Write accesses"]
pub type WaitwritemonitoringR = crate::BitReader;
#[doc = "Field `WAITWRITEMONITORING` writer - 21:21\\]
Selects the Wait monitoring configuration for Write accesses"]
pub type WaitwritemonitoringW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITREADMONITORING` reader - 22:22\\]
Selects the Wait monitoring configuration for Read accesses \\[Reset value is BOOTWAITEN input pin sampled at IC reset\\]"]
pub type WaitreadmonitoringR = crate::BitReader;
#[doc = "Field `WAITREADMONITORING` writer - 22:22\\]
Selects the Wait monitoring configuration for Read accesses \\[Reset value is BOOTWAITEN input pin sampled at IC reset\\]"]
pub type WaitreadmonitoringW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTACHEDDEVICEPAGELENGTH` reader - 24:23\\]
Specifies the attached device page \\[burst\\]
length"]
pub type AttacheddevicepagelengthR = crate::FieldReader;
#[doc = "Field `ATTACHEDDEVICEPAGELENGTH` writer - 24:23\\]
Specifies the attached device page \\[burst\\]
length"]
pub type AttacheddevicepagelengthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKACTIVATIONTIME` reader - 26:25\\]
Output GPMC.CLK activation time"]
pub type ClkactivationtimeR = crate::FieldReader;
#[doc = "Field `CLKACTIVATIONTIME` writer - 26:25\\]
Output GPMC.CLK activation time"]
pub type ClkactivationtimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITETYPE` reader - 27:27\\]
Selects the write mode operation"]
pub type WritetypeR = crate::BitReader;
#[doc = "Field `WRITETYPE` writer - 27:27\\]
Selects the write mode operation"]
pub type WritetypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEMULTIPLE` reader - 28:28\\]
Selects the write single or multiple access"]
pub type WritemultipleR = crate::BitReader;
#[doc = "Field `WRITEMULTIPLE` writer - 28:28\\]
Selects the write single or multiple access"]
pub type WritemultipleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READTYPE` reader - 29:29\\]
Selects the read mode operation"]
pub type ReadtypeR = crate::BitReader;
#[doc = "Field `READTYPE` writer - 29:29\\]
Selects the read mode operation"]
pub type ReadtypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READMULTIPLE` reader - 30:30\\]
Selects the read single or multiple access"]
pub type ReadmultipleR = crate::BitReader;
#[doc = "Field `READMULTIPLE` writer - 30:30\\]
Selects the read single or multiple access"]
pub type ReadmultipleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRAPBURST` reader - 31:31\\]
Enables the wrapping burst capability. Must be set if the attached device is configured in wrapping burst"]
pub type WrapburstR = crate::BitReader;
#[doc = "Field `WRAPBURST` writer - 31:31\\]
Enables the wrapping burst capability. Must be set if the attached device is configured in wrapping burst"]
pub type WrapburstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Divides the GPMC.FCLK clock"]
    #[inline(always)]
    pub fn gpmcfclkdivider(&self) -> GpmcfclkdividerR {
        GpmcfclkdividerR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Signals timing latencies scalar factor \\[Rd/WRCycleTime, AccessTime, PageBurstAccessTime, CSOnTime, CSRd/WrOffTime, ADVOnTime, ADVRd/WrOffTime, OEOnTime, OEOffTime, WEOnTime, WEOffTime, Cycle2CycleDelay, BusTurnAround, TimeOutStartValue\\]"]
    #[inline(always)]
    pub fn timeparagranularity(&self) -> TimeparagranularityR {
        TimeparagranularityR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enables the Address and data multiplexed protocol \\[Reset value is CS0MUXDEVICE input pin sampled at IC reset for CS0 and 0 for CS1-7\\]"]
    #[inline(always)]
    pub fn muxadddata(&self) -> MuxadddataR {
        MuxadddataR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects the attached device type"]
    #[inline(always)]
    pub fn devicetype(&self) -> DevicetypeR {
        DevicetypeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects the device size attached \\[Reset value is BOOTDEVICESIZE input pin sampled at IC reset for CS0 and 01 for CS1-7\\]"]
    #[inline(always)]
    pub fn devicesize(&self) -> DevicesizeR {
        DevicesizeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Selects the input WAIT pin for this chip select \\[Reset value is BOOTWAITSELECT input pin sampled at IC reset for CS0 and 0 for CS1-7\\]"]
    #[inline(always)]
    pub fn waitpinselect(&self) -> WaitpinselectR {
        WaitpinselectR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Selects input pin Wait monitoring time"]
    #[inline(always)]
    pub fn waitmonitoringtime(&self) -> WaitmonitoringtimeR {
        WaitmonitoringtimeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Selects the Wait monitoring configuration for Write accesses"]
    #[inline(always)]
    pub fn waitwritemonitoring(&self) -> WaitwritemonitoringR {
        WaitwritemonitoringR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Selects the Wait monitoring configuration for Read accesses \\[Reset value is BOOTWAITEN input pin sampled at IC reset\\]"]
    #[inline(always)]
    pub fn waitreadmonitoring(&self) -> WaitreadmonitoringR {
        WaitreadmonitoringR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Specifies the attached device page \\[burst\\]
length"]
    #[inline(always)]
    pub fn attacheddevicepagelength(&self) -> AttacheddevicepagelengthR {
        AttacheddevicepagelengthR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Output GPMC.CLK activation time"]
    #[inline(always)]
    pub fn clkactivationtime(&self) -> ClkactivationtimeR {
        ClkactivationtimeR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Selects the write mode operation"]
    #[inline(always)]
    pub fn writetype(&self) -> WritetypeR {
        WritetypeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Selects the write single or multiple access"]
    #[inline(always)]
    pub fn writemultiple(&self) -> WritemultipleR {
        WritemultipleR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Selects the read mode operation"]
    #[inline(always)]
    pub fn readtype(&self) -> ReadtypeR {
        ReadtypeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Selects the read single or multiple access"]
    #[inline(always)]
    pub fn readmultiple(&self) -> ReadmultipleR {
        ReadmultipleR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Enables the wrapping burst capability. Must be set if the attached device is configured in wrapping burst"]
    #[inline(always)]
    pub fn wrapburst(&self) -> WrapburstR {
        WrapburstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Divides the GPMC.FCLK clock"]
    #[inline(always)]
    #[must_use]
    pub fn gpmcfclkdivider(&mut self) -> GpmcfclkdividerW<CfgGpmcConfig1Spec> {
        GpmcfclkdividerW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Signals timing latencies scalar factor \\[Rd/WRCycleTime, AccessTime, PageBurstAccessTime, CSOnTime, CSRd/WrOffTime, ADVOnTime, ADVRd/WrOffTime, OEOnTime, OEOffTime, WEOnTime, WEOffTime, Cycle2CycleDelay, BusTurnAround, TimeOutStartValue\\]"]
    #[inline(always)]
    #[must_use]
    pub fn timeparagranularity(&mut self) -> TimeparagranularityW<CfgGpmcConfig1Spec> {
        TimeparagranularityW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enables the Address and data multiplexed protocol \\[Reset value is CS0MUXDEVICE input pin sampled at IC reset for CS0 and 0 for CS1-7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn muxadddata(&mut self) -> MuxadddataW<CfgGpmcConfig1Spec> {
        MuxadddataW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects the attached device type"]
    #[inline(always)]
    #[must_use]
    pub fn devicetype(&mut self) -> DevicetypeW<CfgGpmcConfig1Spec> {
        DevicetypeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Selects the device size attached \\[Reset value is BOOTDEVICESIZE input pin sampled at IC reset for CS0 and 01 for CS1-7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn devicesize(&mut self) -> DevicesizeW<CfgGpmcConfig1Spec> {
        DevicesizeW::new(self, 12)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Selects the input WAIT pin for this chip select \\[Reset value is BOOTWAITSELECT input pin sampled at IC reset for CS0 and 0 for CS1-7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn waitpinselect(&mut self) -> WaitpinselectW<CfgGpmcConfig1Spec> {
        WaitpinselectW::new(self, 16)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Selects input pin Wait monitoring time"]
    #[inline(always)]
    #[must_use]
    pub fn waitmonitoringtime(&mut self) -> WaitmonitoringtimeW<CfgGpmcConfig1Spec> {
        WaitmonitoringtimeW::new(self, 18)
    }
    #[doc = "Bit 21 - 21:21\\]
Selects the Wait monitoring configuration for Write accesses"]
    #[inline(always)]
    #[must_use]
    pub fn waitwritemonitoring(&mut self) -> WaitwritemonitoringW<CfgGpmcConfig1Spec> {
        WaitwritemonitoringW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Selects the Wait monitoring configuration for Read accesses \\[Reset value is BOOTWAITEN input pin sampled at IC reset\\]"]
    #[inline(always)]
    #[must_use]
    pub fn waitreadmonitoring(&mut self) -> WaitreadmonitoringW<CfgGpmcConfig1Spec> {
        WaitreadmonitoringW::new(self, 22)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Specifies the attached device page \\[burst\\]
length"]
    #[inline(always)]
    #[must_use]
    pub fn attacheddevicepagelength(&mut self) -> AttacheddevicepagelengthW<CfgGpmcConfig1Spec> {
        AttacheddevicepagelengthW::new(self, 23)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Output GPMC.CLK activation time"]
    #[inline(always)]
    #[must_use]
    pub fn clkactivationtime(&mut self) -> ClkactivationtimeW<CfgGpmcConfig1Spec> {
        ClkactivationtimeW::new(self, 25)
    }
    #[doc = "Bit 27 - 27:27\\]
Selects the write mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn writetype(&mut self) -> WritetypeW<CfgGpmcConfig1Spec> {
        WritetypeW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Selects the write single or multiple access"]
    #[inline(always)]
    #[must_use]
    pub fn writemultiple(&mut self) -> WritemultipleW<CfgGpmcConfig1Spec> {
        WritemultipleW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Selects the read mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn readtype(&mut self) -> ReadtypeW<CfgGpmcConfig1Spec> {
        ReadtypeW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Selects the read single or multiple access"]
    #[inline(always)]
    #[must_use]
    pub fn readmultiple(&mut self) -> ReadmultipleW<CfgGpmcConfig1Spec> {
        ReadmultipleW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Enables the wrapping burst capability. Must be set if the attached device is configured in wrapping burst"]
    #[inline(always)]
    #[must_use]
    pub fn wrapburst(&mut self) -> WrapburstW<CfgGpmcConfig1Spec> {
        WrapburstW::new(self, 31)
    }
}
#[doc = "The configuration 1 register sets signal control parameters per chip select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcConfig1Spec;
impl crate::RegisterSpec for CfgGpmcConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_config1::R`](R) reader structure"]
impl crate::Readable for CfgGpmcConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_config1::W`](W) writer structure"]
impl crate::Writable for CfgGpmcConfig1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_CONFIG1 to value 0"]
impl crate::Resettable for CfgGpmcConfig1Spec {
    const RESET_VALUE: u32 = 0;
}
