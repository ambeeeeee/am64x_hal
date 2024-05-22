#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_settings` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_settings` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec>;
#[doc = "Field `POWER_MODE` reader - 0:0\\]
This field determines either Fast mode or Low Power mode.Host and all devices connected to the host shall be set to the same mode."]
pub type PowerModeR = crate::BitReader;
#[doc = "Field `POWER_MODE` writer - 0:0\\]
This field determines either Fast mode or Low Power mode.Host and all devices connected to the host shall be set to the same mode."]
pub type PowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMLANES` reader - 13:8\\]
The lane configuration of a Host System is set to this field depends on the capability among Host Controller and connected devices. 2 Lanes FD mode is mandatory and the others modes are optional. 0000b - 2 Lanes FD or 2L-HD 0001b - Not Used 0010b - 3 Lanes 2D1U-FD \\[Embedded\\]
0011b - 3 Lanes 1D2U-FD \\[Embedded\\]
0100b - 4 Lanes 2D2U-FD \\[Embedded\\]
others - Reserved"]
pub type NumlanesR = crate::FieldReader;
#[doc = "Field `NUMLANES` writer - 13:8\\]
The lane configuration of a Host System is set to this field depends on the capability among Host Controller and connected devices. 2 Lanes FD mode is mandatory and the others modes are optional. 0000b - 2 Lanes FD or 2L-HD 0001b - Not Used 0010b - 3 Lanes 2D1U-FD \\[Embedded\\]
0011b - 3 Lanes 1D2U-FD \\[Embedded\\]
0100b - 4 Lanes 2D2U-FD \\[Embedded\\]
others - Reserved"]
pub type NumlanesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This field determines either Fast mode or Low Power mode.Host and all devices connected to the host shall be set to the same mode."]
    #[inline(always)]
    pub fn power_mode(&self) -> PowerModeR {
        PowerModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
The lane configuration of a Host System is set to this field depends on the capability among Host Controller and connected devices. 2 Lanes FD mode is mandatory and the others modes are optional. 0000b - 2 Lanes FD or 2L-HD 0001b - Not Used 0010b - 3 Lanes 2D1U-FD \\[Embedded\\]
0011b - 3 Lanes 1D2U-FD \\[Embedded\\]
0100b - 4 Lanes 2D2U-FD \\[Embedded\\]
others - Reserved"]
    #[inline(always)]
    pub fn numlanes(&self) -> NumlanesR {
        NumlanesR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This field determines either Fast mode or Low Power mode.Host and all devices connected to the host shall be set to the same mode."]
    #[inline(always)]
    #[must_use]
    pub fn power_mode(&mut self) -> PowerModeW<SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec> {
        PowerModeW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
The lane configuration of a Host System is set to this field depends on the capability among Host Controller and connected devices. 2 Lanes FD mode is mandatory and the others modes are optional. 0000b - 2 Lanes FD or 2L-HD 0001b - Not Used 0010b - 3 Lanes 2D1U-FD \\[Embedded\\]
0011b - 3 Lanes 1D2U-FD \\[Embedded\\]
0100b - 4 Lanes 2D2U-FD \\[Embedded\\]
others - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn numlanes(&mut self) -> NumlanesW<SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec> {
        NumlanesW::new(self, 8)
    }
}
#[doc = "Start Address of General settings is pointed by Pointer for UHS-II Setting Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_settings to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec {
    const RESET_VALUE: u32 = 0;
}
