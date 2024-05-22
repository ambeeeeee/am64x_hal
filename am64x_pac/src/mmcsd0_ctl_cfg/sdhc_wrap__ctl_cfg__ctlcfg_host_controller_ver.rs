#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_host_controller_ver` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_host_controller_ver` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec>;
#[doc = "Field `SPEC_VER_NUM` reader - 7:0\\]
This status indicates the Host Controller Spec. Version. The upper and lower 4-bits indicate the version. 00h - SD Host Controller Specification Version 1.00 01h - SD Host Controller Specification Version 2.00 Including the feature of the ADMA and Test Register 02h - SD Host Controller Specification Version 3.00 03h - SD Host Controller Specification Version 4.00 04h - SD Host Controller Specification Version 4.10 'others' Reserved"]
pub type SpecVerNumR = crate::FieldReader;
#[doc = "Field `SPEC_VER_NUM` writer - 7:0\\]
This status indicates the Host Controller Spec. Version. The upper and lower 4-bits indicate the version. 00h - SD Host Controller Specification Version 1.00 01h - SD Host Controller Specification Version 2.00 Including the feature of the ADMA and Test Register 02h - SD Host Controller Specification Version 3.00 03h - SD Host Controller Specification Version 4.00 04h - SD Host Controller Specification Version 4.10 'others' Reserved"]
pub type SpecVerNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VEN_VER_NUM` reader - 15:8\\]
The Vendor Version Number is set to 0x10 \\[1.0\\]"]
pub type VenVerNumR = crate::FieldReader;
#[doc = "Field `VEN_VER_NUM` writer - 15:8\\]
The Vendor Version Number is set to 0x10 \\[1.0\\]"]
pub type VenVerNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This status indicates the Host Controller Spec. Version. The upper and lower 4-bits indicate the version. 00h - SD Host Controller Specification Version 1.00 01h - SD Host Controller Specification Version 2.00 Including the feature of the ADMA and Test Register 02h - SD Host Controller Specification Version 3.00 03h - SD Host Controller Specification Version 4.00 04h - SD Host Controller Specification Version 4.10 'others' Reserved"]
    #[inline(always)]
    pub fn spec_ver_num(&self) -> SpecVerNumR {
        SpecVerNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The Vendor Version Number is set to 0x10 \\[1.0\\]"]
    #[inline(always)]
    pub fn ven_ver_num(&self) -> VenVerNumR {
        VenVerNumR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This status indicates the Host Controller Spec. Version. The upper and lower 4-bits indicate the version. 00h - SD Host Controller Specification Version 1.00 01h - SD Host Controller Specification Version 2.00 Including the feature of the ADMA and Test Register 02h - SD Host Controller Specification Version 3.00 03h - SD Host Controller Specification Version 4.00 04h - SD Host Controller Specification Version 4.10 'others' Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn spec_ver_num(&mut self) -> SpecVerNumW<SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec> {
        SpecVerNumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The Vendor Version Number is set to 0x10 \\[1.0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ven_ver_num(&mut self) -> VenVerNumW<SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec> {
        VenVerNumW::new(self, 8)
    }
}
#[doc = "This register is used to read the vendor version number and specification version number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_host_controller_ver to value 0x1604"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec {
    const RESET_VALUE: u16 = 0x1604;
}
