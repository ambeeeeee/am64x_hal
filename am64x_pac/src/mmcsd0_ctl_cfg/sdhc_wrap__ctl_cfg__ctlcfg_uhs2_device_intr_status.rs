#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_intr_status` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatusSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_intr_status` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatusSpec>;
#[doc = "Field `DEV_INT_STS` reader - 15:0\\]
This register shows receipt of INT MSG from which device and is effective when INT MSG Enable is set to 1 in the UHS- II Device Select register. On receiving INT MSG from a device, Host Controller saves the INT MSG to UHS-II Device Interrupt Code register. A bit of this register, which is corre- spondent to Device ID, is set to 1 and generate Card Interrupt in Normal Interrupt Status register. Writing a bit to 1 clears the status bit \\[interrupt is treated\\]
and writing a bit to 0 keeps the status value \\[interrupt is untreated\\]. If INT MSG Enable is set to 0, this register is cleared to 0 and Host Controller ignores receipt of INT MSG. Effective bit range of this register is determined by Number of Devices Supported in the UHS-II General Capabilities regis-ter. If N devices are supported, bits 1 to N are effective. Then Device ID is supposed to be assigned from 1 sequentially at the UHS-II Initialization. A bit of unsupported Device ID in this register shall be indicated to 0. D00 - Not used \\[Reserved\\]
D01 - Setting 1 means INT MSG is received from Device ID 1 D02 - Setting 1 means INT MSG is received from Device ID 2 .... ..... D15 - Setting 1 means INT MSG is received from Device ID 15"]
pub type DevIntStsR = crate::FieldReader<u16>;
#[doc = "Field `DEV_INT_STS` writer - 15:0\\]
This register shows receipt of INT MSG from which device and is effective when INT MSG Enable is set to 1 in the UHS- II Device Select register. On receiving INT MSG from a device, Host Controller saves the INT MSG to UHS-II Device Interrupt Code register. A bit of this register, which is corre- spondent to Device ID, is set to 1 and generate Card Interrupt in Normal Interrupt Status register. Writing a bit to 1 clears the status bit \\[interrupt is treated\\]
and writing a bit to 0 keeps the status value \\[interrupt is untreated\\]. If INT MSG Enable is set to 0, this register is cleared to 0 and Host Controller ignores receipt of INT MSG. Effective bit range of this register is determined by Number of Devices Supported in the UHS-II General Capabilities regis-ter. If N devices are supported, bits 1 to N are effective. Then Device ID is supposed to be assigned from 1 sequentially at the UHS-II Initialization. A bit of unsupported Device ID in this register shall be indicated to 0. D00 - Not used \\[Reserved\\]
D01 - Setting 1 means INT MSG is received from Device ID 1 D02 - Setting 1 means INT MSG is received from Device ID 2 .... ..... D15 - Setting 1 means INT MSG is received from Device ID 15"]
pub type DevIntStsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This register shows receipt of INT MSG from which device and is effective when INT MSG Enable is set to 1 in the UHS- II Device Select register. On receiving INT MSG from a device, Host Controller saves the INT MSG to UHS-II Device Interrupt Code register. A bit of this register, which is corre- spondent to Device ID, is set to 1 and generate Card Interrupt in Normal Interrupt Status register. Writing a bit to 1 clears the status bit \\[interrupt is treated\\]
and writing a bit to 0 keeps the status value \\[interrupt is untreated\\]. If INT MSG Enable is set to 0, this register is cleared to 0 and Host Controller ignores receipt of INT MSG. Effective bit range of this register is determined by Number of Devices Supported in the UHS-II General Capabilities regis-ter. If N devices are supported, bits 1 to N are effective. Then Device ID is supposed to be assigned from 1 sequentially at the UHS-II Initialization. A bit of unsupported Device ID in this register shall be indicated to 0. D00 - Not used \\[Reserved\\]
D01 - Setting 1 means INT MSG is received from Device ID 1 D02 - Setting 1 means INT MSG is received from Device ID 2 .... ..... D15 - Setting 1 means INT MSG is received from Device ID 15"]
    #[inline(always)]
    pub fn dev_int_sts(&self) -> DevIntStsR {
        DevIntStsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This register shows receipt of INT MSG from which device and is effective when INT MSG Enable is set to 1 in the UHS- II Device Select register. On receiving INT MSG from a device, Host Controller saves the INT MSG to UHS-II Device Interrupt Code register. A bit of this register, which is corre- spondent to Device ID, is set to 1 and generate Card Interrupt in Normal Interrupt Status register. Writing a bit to 1 clears the status bit \\[interrupt is treated\\]
and writing a bit to 0 keeps the status value \\[interrupt is untreated\\]. If INT MSG Enable is set to 0, this register is cleared to 0 and Host Controller ignores receipt of INT MSG. Effective bit range of this register is determined by Number of Devices Supported in the UHS-II General Capabilities regis-ter. If N devices are supported, bits 1 to N are effective. Then Device ID is supposed to be assigned from 1 sequentially at the UHS-II Initialization. A bit of unsupported Device ID in this register shall be indicated to 0. D00 - Not used \\[Reserved\\]
D01 - Setting 1 means INT MSG is received from Device ID 1 D02 - Setting 1 means INT MSG is received from Device ID 2 .... ..... D15 - Setting 1 means INT MSG is received from Device ID 15"]
    #[inline(always)]
    #[must_use]
    pub fn dev_int_sts(&mut self) -> DevIntStsW<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatusSpec> {
        DevIntStsW::new(self, 0)
    }
}
#[doc = "This register shows receipt of INT MSG from which device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatusSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_intr_status to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatusSpec {
    const RESET_VALUE: u16 = 0;
}
