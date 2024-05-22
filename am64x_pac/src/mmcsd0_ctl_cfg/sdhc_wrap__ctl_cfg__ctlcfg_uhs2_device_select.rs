#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_select` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_select` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec>;
#[doc = "Field `DEV_SEL` reader - 3:0\\]
Host Controller holds an INT MSG packet per device. One of INT MSGs \\[up to 15\\]
can be selected by this field and read from the UHS-II Device Interrupt Code Register \\[0BFh\\]. This field is effective when INT MSG Enable is set to 1. The number of devices implemented in the Host Controller is indicated by Number of Devices supported in the UHS-II General Capabilities register. 0h Unselected \\[Default\\]
1h INT MSG of Device ID 1 is selected 2h INT MSG of Device ID 2 is selected ..... ..... Fh INT MSG of Device ID 15 is selected"]
pub type DevSelR = crate::FieldReader;
#[doc = "Field `DEV_SEL` writer - 3:0\\]
Host Controller holds an INT MSG packet per device. One of INT MSGs \\[up to 15\\]
can be selected by this field and read from the UHS-II Device Interrupt Code Register \\[0BFh\\]. This field is effective when INT MSG Enable is set to 1. The number of devices implemented in the Host Controller is indicated by Number of Devices supported in the UHS-II General Capabilities register. 0h Unselected \\[Default\\]
1h INT MSG of Device ID 1 is selected 2h INT MSG of Device ID 2 is selected ..... ..... Fh INT MSG of Device ID 15 is selected"]
pub type DevSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INT_MSG_ENA` reader - 7:7\\]
This bit enables receipt of INT MSG. If this bit is set to 1,receipt of INT MSG is informed by Card Interrupt in the Nor-mal Interrupt Status register. If this bit is set to 0, Host Con-troller ignores receipt of INT MSG and may not set the UHS-II Device Interrupt Code register. Support of INT MSG Interrupt is optional. If trying to set this bit to 1 but still this bit is read 0, INT MSG Interrupt is not sup-ported by the Host Controller. In this case, UHS-II Device Interrupt Status register always shall be read 0 and UHS-II Device Interrupt Code register may not be implemented. '0' Disabled '1' Enabled"]
pub type IntMsgEnaR = crate::BitReader;
#[doc = "Field `INT_MSG_ENA` writer - 7:7\\]
This bit enables receipt of INT MSG. If this bit is set to 1,receipt of INT MSG is informed by Card Interrupt in the Nor-mal Interrupt Status register. If this bit is set to 0, Host Con-troller ignores receipt of INT MSG and may not set the UHS-II Device Interrupt Code register. Support of INT MSG Interrupt is optional. If trying to set this bit to 1 but still this bit is read 0, INT MSG Interrupt is not sup-ported by the Host Controller. In this case, UHS-II Device Interrupt Status register always shall be read 0 and UHS-II Device Interrupt Code register may not be implemented. '0' Disabled '1' Enabled"]
pub type IntMsgEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Host Controller holds an INT MSG packet per device. One of INT MSGs \\[up to 15\\]
can be selected by this field and read from the UHS-II Device Interrupt Code Register \\[0BFh\\]. This field is effective when INT MSG Enable is set to 1. The number of devices implemented in the Host Controller is indicated by Number of Devices supported in the UHS-II General Capabilities register. 0h Unselected \\[Default\\]
1h INT MSG of Device ID 1 is selected 2h INT MSG of Device ID 2 is selected ..... ..... Fh INT MSG of Device ID 15 is selected"]
    #[inline(always)]
    pub fn dev_sel(&self) -> DevSelR {
        DevSelR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit enables receipt of INT MSG. If this bit is set to 1,receipt of INT MSG is informed by Card Interrupt in the Nor-mal Interrupt Status register. If this bit is set to 0, Host Con-troller ignores receipt of INT MSG and may not set the UHS-II Device Interrupt Code register. Support of INT MSG Interrupt is optional. If trying to set this bit to 1 but still this bit is read 0, INT MSG Interrupt is not sup-ported by the Host Controller. In this case, UHS-II Device Interrupt Status register always shall be read 0 and UHS-II Device Interrupt Code register may not be implemented. '0' Disabled '1' Enabled"]
    #[inline(always)]
    pub fn int_msg_ena(&self) -> IntMsgEnaR {
        IntMsgEnaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Host Controller holds an INT MSG packet per device. One of INT MSGs \\[up to 15\\]
can be selected by this field and read from the UHS-II Device Interrupt Code Register \\[0BFh\\]. This field is effective when INT MSG Enable is set to 1. The number of devices implemented in the Host Controller is indicated by Number of Devices supported in the UHS-II General Capabilities register. 0h Unselected \\[Default\\]
1h INT MSG of Device ID 1 is selected 2h INT MSG of Device ID 2 is selected ..... ..... Fh INT MSG of Device ID 15 is selected"]
    #[inline(always)]
    #[must_use]
    pub fn dev_sel(&mut self) -> DevSelW<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec> {
        DevSelW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit enables receipt of INT MSG. If this bit is set to 1,receipt of INT MSG is informed by Card Interrupt in the Nor-mal Interrupt Status register. If this bit is set to 0, Host Con-troller ignores receipt of INT MSG and may not set the UHS-II Device Interrupt Code register. Support of INT MSG Interrupt is optional. If trying to set this bit to 1 but still this bit is read 0, INT MSG Interrupt is not sup-ported by the Host Controller. In this case, UHS-II Device Interrupt Status register always shall be read 0 and UHS-II Device Interrupt Code register may not be implemented. '0' Disabled '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn int_msg_ena(&mut self) -> IntMsgEnaW<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec> {
        IntMsgEnaW::new(self, 7)
    }
}
#[doc = "UHS-II Device Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_select to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec {
    const RESET_VALUE: u8 = 0;
}
