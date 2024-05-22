#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_cap` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_cap` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec>;
#[doc = "Field `DAP` reader - 3:0\\]
This field indicates the maximum capability of host power supply for a device configured by a Host system.This field is used to set the argument of DEVICE_INIT CCMD 0h -360 mW \\[Default\\]
1h - 360 mW 2h - 720 mW ..... ....... Fh - 360 x15 mW"]
pub type DapR = crate::FieldReader;
#[doc = "Field `DAP` writer - 3:0\\]
This field indicates the maximum capability of host power supply for a device configured by a Host system.This field is used to set the argument of DEVICE_INIT CCMD 0h -360 mW \\[Default\\]
1h - 360 mW 2h - 720 mW ..... ....... Fh - 360 x15 mW"]
pub type DapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GAP` reader - 7:4\\]
This field indicates the maximum capability of host power supply for a group configured by a Host system.This field is used to set the argument of DEVICE_INIT CCMD 0h -Not used 1h - 360 mW 2h - 720 mW ..... ....... Fh - 360x15 mW"]
pub type GapR = crate::FieldReader;
#[doc = "Field `GAP` writer - 7:4\\]
This field indicates the maximum capability of host power supply for a group configured by a Host system.This field is used to set the argument of DEVICE_INIT CCMD 0h -Not used 1h - 360 mW 2h - 720 mW ..... ....... Fh - 360x15 mW"]
pub type GapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUM_LANES` reader - 13:8\\]
This field indicates support of lanes by the Host Controller.0 mean not supported and 1 means supported. D08 - 2L-HD D09 - 2D1U-FD D10 - 1D2U-FD D11 - 2D2U-FD D12 - Reserved D13 - Reserved"]
pub type NumLanesR = crate::FieldReader;
#[doc = "Field `NUM_LANES` writer - 13:8\\]
This field indicates support of lanes by the Host Controller.0 mean not supported and 1 means supported. D08 - 2L-HD D09 - 2D1U-FD D10 - 1D2U-FD D11 - 2D2U-FD D12 - Reserved D13 - Reserved"]
pub type NumLanesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CFG_64BIT_ADDRESSING` reader - 14:14\\]
This field indicates support of 64-bit addressing by the Host Controller. '0' 32-bit Addressing is supported '1' 32-bit and 64-bit Addressing is supported"]
pub type Cfg64bitAddressingR = crate::BitReader;
#[doc = "Field `CFG_64BIT_ADDRESSING` writer - 14:14\\]
This field indicates support of 64-bit addressing by the Host Controller. '0' 32-bit Addressing is supported '1' 32-bit and 64-bit Addressing is supported"]
pub type Cfg64bitAddressingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_TYPE` reader - 17:16\\]
This field indicates device type configured by a Host system. '00' Removable Card\\[P2P\\]
'01' Embedded Devices '10' Embedded Devices+Removable Card '11' Reserved"]
pub type DeviceTypeR = crate::FieldReader;
#[doc = "Field `DEVICE_TYPE` writer - 17:16\\]
This field indicates device type configured by a Host system. '00' Removable Card\\[P2P\\]
'01' Embedded Devices '10' Embedded Devices+Removable Card '11' Reserved"]
pub type DeviceTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CORECFG_UHS2_MAX_DEVICES` reader - 21:18\\]
This field indicates the maximum number of devices supported by the Host Controller. 0h - Not used 1h - 1 Devices 2h - 2 Devices ..... ....... Fh - 15 Devices"]
pub type CorecfgUhs2MaxDevicesR = crate::FieldReader;
#[doc = "Field `CORECFG_UHS2_MAX_DEVICES` writer - 21:18\\]
This field indicates the maximum number of devices supported by the Host Controller. 0h - Not used 1h - 1 Devices 2h - 2 Devices ..... ....... Fh - 15 Devices"]
pub type CorecfgUhs2MaxDevicesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CORECFG_UHS2_BUS_TOPLOGY` reader - 23:22\\]
This field indicates one of bus topologies configured by a Host system. '00' P2P Connection '01' Ring Connection '10' HUB Connection '11' HUB is connected in Ring"]
pub type CorecfgUhs2BusToplogyR = crate::FieldReader;
#[doc = "Field `CORECFG_UHS2_BUS_TOPLOGY` writer - 23:22\\]
This field indicates one of bus topologies configured by a Host system. '00' P2P Connection '01' Ring Connection '10' HUB Connection '11' HUB is connected in Ring"]
pub type CorecfgUhs2BusToplogyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This field indicates the maximum capability of host power supply for a device configured by a Host system.This field is used to set the argument of DEVICE_INIT CCMD 0h -360 mW \\[Default\\]
1h - 360 mW 2h - 720 mW ..... ....... Fh - 360 x15 mW"]
    #[inline(always)]
    pub fn dap(&self) -> DapR {
        DapR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This field indicates the maximum capability of host power supply for a group configured by a Host system.This field is used to set the argument of DEVICE_INIT CCMD 0h -Not used 1h - 360 mW 2h - 720 mW ..... ....... Fh - 360x15 mW"]
    #[inline(always)]
    pub fn gap(&self) -> GapR {
        GapR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
This field indicates support of lanes by the Host Controller.0 mean not supported and 1 means supported. D08 - 2L-HD D09 - 2D1U-FD D10 - 1D2U-FD D11 - 2D2U-FD D12 - Reserved D13 - Reserved"]
    #[inline(always)]
    pub fn num_lanes(&self) -> NumLanesR {
        NumLanesR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
This field indicates support of 64-bit addressing by the Host Controller. '0' 32-bit Addressing is supported '1' 32-bit and 64-bit Addressing is supported"]
    #[inline(always)]
    pub fn cfg_64bit_addressing(&self) -> Cfg64bitAddressingR {
        Cfg64bitAddressingR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
This field indicates device type configured by a Host system. '00' Removable Card\\[P2P\\]
'01' Embedded Devices '10' Embedded Devices+Removable Card '11' Reserved"]
    #[inline(always)]
    pub fn device_type(&self) -> DeviceTypeR {
        DeviceTypeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - 21:18\\]
This field indicates the maximum number of devices supported by the Host Controller. 0h - Not used 1h - 1 Devices 2h - 2 Devices ..... ....... Fh - 15 Devices"]
    #[inline(always)]
    pub fn corecfg_uhs2_max_devices(&self) -> CorecfgUhs2MaxDevicesR {
        CorecfgUhs2MaxDevicesR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
This field indicates one of bus topologies configured by a Host system. '00' P2P Connection '01' Ring Connection '10' HUB Connection '11' HUB is connected in Ring"]
    #[inline(always)]
    pub fn corecfg_uhs2_bus_toplogy(&self) -> CorecfgUhs2BusToplogyR {
        CorecfgUhs2BusToplogyR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This field indicates the maximum capability of host power supply for a device configured by a Host system.This field is used to set the argument of DEVICE_INIT CCMD 0h -360 mW \\[Default\\]
1h - 360 mW 2h - 720 mW ..... ....... Fh - 360 x15 mW"]
    #[inline(always)]
    #[must_use]
    pub fn dap(&mut self) -> DapW<SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec> {
        DapW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This field indicates the maximum capability of host power supply for a group configured by a Host system.This field is used to set the argument of DEVICE_INIT CCMD 0h -Not used 1h - 360 mW 2h - 720 mW ..... ....... Fh - 360x15 mW"]
    #[inline(always)]
    #[must_use]
    pub fn gap(&mut self) -> GapW<SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec> {
        GapW::new(self, 4)
    }
    #[doc = "Bits 8:13 - 13:8\\]
This field indicates support of lanes by the Host Controller.0 mean not supported and 1 means supported. D08 - 2L-HD D09 - 2D1U-FD D10 - 1D2U-FD D11 - 2D2U-FD D12 - Reserved D13 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn num_lanes(&mut self) -> NumLanesW<SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec> {
        NumLanesW::new(self, 8)
    }
    #[doc = "Bit 14 - 14:14\\]
This field indicates support of 64-bit addressing by the Host Controller. '0' 32-bit Addressing is supported '1' 32-bit and 64-bit Addressing is supported"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_64bit_addressing(
        &mut self,
    ) -> Cfg64bitAddressingW<SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec> {
        Cfg64bitAddressingW::new(self, 14)
    }
    #[doc = "Bits 16:17 - 17:16\\]
This field indicates device type configured by a Host system. '00' Removable Card\\[P2P\\]
'01' Embedded Devices '10' Embedded Devices+Removable Card '11' Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn device_type(&mut self) -> DeviceTypeW<SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec> {
        DeviceTypeW::new(self, 16)
    }
    #[doc = "Bits 18:21 - 21:18\\]
This field indicates the maximum number of devices supported by the Host Controller. 0h - Not used 1h - 1 Devices 2h - 2 Devices ..... ....... Fh - 15 Devices"]
    #[inline(always)]
    #[must_use]
    pub fn corecfg_uhs2_max_devices(
        &mut self,
    ) -> CorecfgUhs2MaxDevicesW<SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec> {
        CorecfgUhs2MaxDevicesW::new(self, 18)
    }
    #[doc = "Bits 22:23 - 23:22\\]
This field indicates one of bus topologies configured by a Host system. '00' P2P Connection '01' Ring Connection '10' HUB Connection '11' HUB is connected in Ring"]
    #[inline(always)]
    #[must_use]
    pub fn corecfg_uhs2_bus_toplogy(
        &mut self,
    ) -> CorecfgUhs2BusToplogyW<SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec> {
        CorecfgUhs2BusToplogyW::new(self, 22)
    }
}
#[doc = "Start Address of General Capabilities is pointed by Pointer for UHS-II Host Capabilities Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_cap to value 0x0004_5511"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec {
    const RESET_VALUE: u32 = 0x0004_5511;
}
