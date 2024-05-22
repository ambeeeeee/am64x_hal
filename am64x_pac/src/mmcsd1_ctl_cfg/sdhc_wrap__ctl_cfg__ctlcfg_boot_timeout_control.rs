#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_boot_timeout_control` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgBootTimeoutControlSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_boot_timeout_control` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgBootTimeoutControlSpec>;
#[doc = "Field `DATA_TIMEOUT_CNT` reader - 31:0\\]
This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC4.4 card.The value is in number of sd clock."]
pub type DataTimeoutCntR = crate::FieldReader<u32>;
#[doc = "Field `DATA_TIMEOUT_CNT` writer - 31:0\\]
This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC4.4 card.The value is in number of sd clock."]
pub type DataTimeoutCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC4.4 card.The value is in number of sd clock."]
    #[inline(always)]
    pub fn data_timeout_cnt(&self) -> DataTimeoutCntR {
        DataTimeoutCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC4.4 card.The value is in number of sd clock."]
    #[inline(always)]
    #[must_use]
    pub fn data_timeout_cnt(
        &mut self,
    ) -> DataTimeoutCntW<SdhcWrap_CtlCfg_CtlcfgBootTimeoutControlSpec> {
        DataTimeoutCntW::new(self, 0)
    }
}
#[doc = "This is used to program the boot timeout value counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgBootTimeoutControlSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgBootTimeoutControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgBootTimeoutControlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgBootTimeoutControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_boot_timeout_control to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgBootTimeoutControlSpec {
    const RESET_VALUE: u32 = 0;
}
