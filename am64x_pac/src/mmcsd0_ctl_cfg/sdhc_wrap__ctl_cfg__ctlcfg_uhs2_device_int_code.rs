#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_int_code` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCodeSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_int_code` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCodeSpec>;
#[doc = "Field `DEV_INTR` reader - 7:0\\]
This register is effective when INT MSG Enable is set to 1 in the UHS-II Device Select register. Host Controller holds an INT MSG packet per device. One of INT MSGs \\[Code length is 1 byte\\]
up to 15 can be read from this register by select-ing UHS-II Device Select. The number of the registers to hold INT MSGs is determined by Number of Devices Sup-ported in the UHS-II General Capabili-ties register. Device ID is supposed to be assigned from 1 sequentially at the UHS-II Initialization."]
pub type DevIntrR = crate::FieldReader;
#[doc = "Field `DEV_INTR` writer - 7:0\\]
This register is effective when INT MSG Enable is set to 1 in the UHS-II Device Select register. Host Controller holds an INT MSG packet per device. One of INT MSGs \\[Code length is 1 byte\\]
up to 15 can be read from this register by select-ing UHS-II Device Select. The number of the registers to hold INT MSGs is determined by Number of Devices Sup-ported in the UHS-II General Capabili-ties register. Device ID is supposed to be assigned from 1 sequentially at the UHS-II Initialization."]
pub type DevIntrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This register is effective when INT MSG Enable is set to 1 in the UHS-II Device Select register. Host Controller holds an INT MSG packet per device. One of INT MSGs \\[Code length is 1 byte\\]
up to 15 can be read from this register by select-ing UHS-II Device Select. The number of the registers to hold INT MSGs is determined by Number of Devices Sup-ported in the UHS-II General Capabili-ties register. Device ID is supposed to be assigned from 1 sequentially at the UHS-II Initialization."]
    #[inline(always)]
    pub fn dev_intr(&self) -> DevIntrR {
        DevIntrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This register is effective when INT MSG Enable is set to 1 in the UHS-II Device Select register. Host Controller holds an INT MSG packet per device. One of INT MSGs \\[Code length is 1 byte\\]
up to 15 can be read from this register by select-ing UHS-II Device Select. The number of the registers to hold INT MSGs is determined by Number of Devices Sup-ported in the UHS-II General Capabili-ties register. Device ID is supposed to be assigned from 1 sequentially at the UHS-II Initialization."]
    #[inline(always)]
    #[must_use]
    pub fn dev_intr(&mut self) -> DevIntrW<SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCodeSpec> {
        DevIntrW::new(self, 0)
    }
}
#[doc = "This register is effective when INT MSG Enable is set to 1 in the UHS-II Device Select register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCodeSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCodeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCodeSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_int_code to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCodeSpec {
    const RESET_VALUE: u8 = 0;
}
