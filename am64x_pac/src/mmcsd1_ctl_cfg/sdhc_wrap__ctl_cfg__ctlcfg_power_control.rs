#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_power_control` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgPowerControlSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_power_control` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgPowerControlSpec>;
#[doc = "Field `SD_BUS_POWER` reader - 0:0\\]
Before setting this bit, the SD host driver shall set SD Bus Voltage Select. If the HC detects the No Card State, this bit shall be cleared. If this bit is cleared, the Host Control-ler should immediately stop driving CMD and DAT\\[3:0\\]
\\[tri-state\\], and drive SDCLK to low level \\[Refer to Section 2.2.14\\]. If card is connectedto Host Controller, Host Controller shall set these lines to low before stopping to supply VDD1.In UHS-II mode, before clearing this bit, Host Driver shall clear SD Clock Enable and before stopping to sup-ply VDD1, Host Controller shall set DAT\\[2\\]
to low if DAT\\[2\\]
is used as out-of band interrupt."]
pub type SdBusPowerR = crate::BitReader;
#[doc = "Field `SD_BUS_POWER` writer - 0:0\\]
Before setting this bit, the SD host driver shall set SD Bus Voltage Select. If the HC detects the No Card State, this bit shall be cleared. If this bit is cleared, the Host Control-ler should immediately stop driving CMD and DAT\\[3:0\\]
\\[tri-state\\], and drive SDCLK to low level \\[Refer to Section 2.2.14\\]. If card is connectedto Host Controller, Host Controller shall set these lines to low before stopping to supply VDD1.In UHS-II mode, before clearing this bit, Host Driver shall clear SD Clock Enable and before stopping to sup-ply VDD1, Host Controller shall set DAT\\[2\\]
to low if DAT\\[2\\]
is used as out-of band interrupt."]
pub type SdBusPowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD_BUS_VOLTAGE` reader - 3:1\\]
By setting these bits, the HD selects the voltage level for the SD card. Before setting this register, the HD shall check the voltage support bits in the capabilities register. If an unsupported voltage is selected, the Host System shall not supply SD bus voltage. '000'- '100' Reserved '101' 1.8V '110' 3.0V '111' 3.3V"]
pub type SdBusVoltageR = crate::FieldReader;
#[doc = "Field `SD_BUS_VOLTAGE` writer - 3:1\\]
By setting these bits, the HD selects the voltage level for the SD card. Before setting this register, the HD shall check the voltage support bits in the capabilities register. If an unsupported voltage is selected, the Host System shall not supply SD bus voltage. '000'- '100' Reserved '101' 1.8V '110' 3.0V '111' 3.3V"]
pub type SdBusVoltageW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UHS2_POWER` reader - 4:4\\]
Setting this bit enables providing VDD2. '0' Power Off '1' Power On"]
pub type Uhs2PowerR = crate::BitReader;
#[doc = "Field `UHS2_POWER` writer - 4:4\\]
Setting this bit enables providing VDD2. '0' Power Off '1' Power On"]
pub type Uhs2PowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHS2_VOLTAGE` reader - 7:5\\]
This field determines supply voltage range to VDD2. This field can be set to 101b if 1.8V VDD2 Support in the Capabilities register is set to 1. '000' VDD2 Not supported '001'- '011' Reserved '100' Reserved for 1.2V '101' 1.8V '110' Not Used '111' Not Used"]
pub type Uhs2VoltageR = crate::FieldReader;
#[doc = "Field `UHS2_VOLTAGE` writer - 7:5\\]
This field determines supply voltage range to VDD2. This field can be set to 101b if 1.8V VDD2 Support in the Capabilities register is set to 1. '000' VDD2 Not supported '001'- '011' Reserved '100' Reserved for 1.2V '101' 1.8V '110' Not Used '111' Not Used"]
pub type Uhs2VoltageW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Before setting this bit, the SD host driver shall set SD Bus Voltage Select. If the HC detects the No Card State, this bit shall be cleared. If this bit is cleared, the Host Control-ler should immediately stop driving CMD and DAT\\[3:0\\]
\\[tri-state\\], and drive SDCLK to low level \\[Refer to Section 2.2.14\\]. If card is connectedto Host Controller, Host Controller shall set these lines to low before stopping to supply VDD1.In UHS-II mode, before clearing this bit, Host Driver shall clear SD Clock Enable and before stopping to sup-ply VDD1, Host Controller shall set DAT\\[2\\]
to low if DAT\\[2\\]
is used as out-of band interrupt."]
    #[inline(always)]
    pub fn sd_bus_power(&self) -> SdBusPowerR {
        SdBusPowerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
By setting these bits, the HD selects the voltage level for the SD card. Before setting this register, the HD shall check the voltage support bits in the capabilities register. If an unsupported voltage is selected, the Host System shall not supply SD bus voltage. '000'- '100' Reserved '101' 1.8V '110' 3.0V '111' 3.3V"]
    #[inline(always)]
    pub fn sd_bus_voltage(&self) -> SdBusVoltageR {
        SdBusVoltageR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 4 - 4:4\\]
Setting this bit enables providing VDD2. '0' Power Off '1' Power On"]
    #[inline(always)]
    pub fn uhs2_power(&self) -> Uhs2PowerR {
        Uhs2PowerR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
This field determines supply voltage range to VDD2. This field can be set to 101b if 1.8V VDD2 Support in the Capabilities register is set to 1. '000' VDD2 Not supported '001'- '011' Reserved '100' Reserved for 1.2V '101' 1.8V '110' Not Used '111' Not Used"]
    #[inline(always)]
    pub fn uhs2_voltage(&self) -> Uhs2VoltageR {
        Uhs2VoltageR::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Before setting this bit, the SD host driver shall set SD Bus Voltage Select. If the HC detects the No Card State, this bit shall be cleared. If this bit is cleared, the Host Control-ler should immediately stop driving CMD and DAT\\[3:0\\]
\\[tri-state\\], and drive SDCLK to low level \\[Refer to Section 2.2.14\\]. If card is connectedto Host Controller, Host Controller shall set these lines to low before stopping to supply VDD1.In UHS-II mode, before clearing this bit, Host Driver shall clear SD Clock Enable and before stopping to sup-ply VDD1, Host Controller shall set DAT\\[2\\]
to low if DAT\\[2\\]
is used as out-of band interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_power(&mut self) -> SdBusPowerW<SdhcWrap_CtlCfg_CtlcfgPowerControlSpec> {
        SdBusPowerW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
By setting these bits, the HD selects the voltage level for the SD card. Before setting this register, the HD shall check the voltage support bits in the capabilities register. If an unsupported voltage is selected, the Host System shall not supply SD bus voltage. '000'- '100' Reserved '101' 1.8V '110' 3.0V '111' 3.3V"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_voltage(&mut self) -> SdBusVoltageW<SdhcWrap_CtlCfg_CtlcfgPowerControlSpec> {
        SdBusVoltageW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Setting this bit enables providing VDD2. '0' Power Off '1' Power On"]
    #[inline(always)]
    #[must_use]
    pub fn uhs2_power(&mut self) -> Uhs2PowerW<SdhcWrap_CtlCfg_CtlcfgPowerControlSpec> {
        Uhs2PowerW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
This field determines supply voltage range to VDD2. This field can be set to 101b if 1.8V VDD2 Support in the Capabilities register is set to 1. '000' VDD2 Not supported '001'- '011' Reserved '100' Reserved for 1.2V '101' 1.8V '110' Not Used '111' Not Used"]
    #[inline(always)]
    #[must_use]
    pub fn uhs2_voltage(&mut self) -> Uhs2VoltageW<SdhcWrap_CtlCfg_CtlcfgPowerControlSpec> {
        Uhs2VoltageW::new(self, 5)
    }
}
#[doc = "This register is used to program the SD Bus power and voltage level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_power_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_power_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgPowerControlSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgPowerControlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_power_control::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgPowerControlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_power_control::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgPowerControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_power_control to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgPowerControlSpec {
    const RESET_VALUE: u8 = 0;
}
