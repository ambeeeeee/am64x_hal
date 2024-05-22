#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_settings` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_settings` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec>;
#[doc = "Field `HOST_NFCU` reader - 15:8\\]
Host Driver sets the number of blocks in Data Burst \\[Flow Control\\]
to this field.The value shall be smaller than or equal to N_FCU capabilities among the Host Controller and connected card and devices. Setting 1 to 4 blocks is recommended considering buffer size. 00h - 256 Blocks 01h - 1 Block 02h - 2 Blocks 03h - 3 Blocks ...... ...... FFh - 255 Blocks"]
pub type HostNfcuR = crate::FieldReader;
#[doc = "Field `HOST_NFCU` writer - 15:8\\]
Host Driver sets the number of blocks in Data Burst \\[Flow Control\\]
to this field.The value shall be smaller than or equal to N_FCU capabilities among the Host Controller and connected card and devices. Setting 1 to 4 blocks is recommended considering buffer size. 00h - 256 Blocks 01h - 1 Block 02h - 2 Blocks 03h - 3 Blocks ...... ...... FFh - 255 Blocks"]
pub type HostNfcuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RETRY_COUNT` reader - 17:16\\]
Data Burst retry count is set to this field. '00' Retry Disabled '01' 1 time '10' 2 times '11' 3 times"]
pub type RetryCountR = crate::FieldReader;
#[doc = "Field `RETRY_COUNT` writer - 17:16\\]
Data Burst retry count is set to this field. '00' Retry Disabled '01' 1 time '10' 2 times '11' 3 times"]
pub type RetryCountW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `N_DATA_GAP` reader - 39:32\\]
The largest value of N_DATA_GAP capabilities among the Host Controller and Connected Devices is set to this field. 00h - No Gap 01h - 1 LSS 02h - 2 LSS 03h - 3 LSS ...... ...... FFh - 255 LSS"]
pub type NDataGapR = crate::FieldReader;
#[doc = "Field `N_DATA_GAP` writer - 39:32\\]
The largest value of N_DATA_GAP capabilities among the Host Controller and Connected Devices is set to this field. 00h - No Gap 01h - 1 LSS 02h - 2 LSS 03h - 3 LSS ...... ...... FFh - 255 LSS"]
pub type NDataGapW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
Host Driver sets the number of blocks in Data Burst \\[Flow Control\\]
to this field.The value shall be smaller than or equal to N_FCU capabilities among the Host Controller and connected card and devices. Setting 1 to 4 blocks is recommended considering buffer size. 00h - 256 Blocks 01h - 1 Block 02h - 2 Blocks 03h - 3 Blocks ...... ...... FFh - 255 Blocks"]
    #[inline(always)]
    pub fn host_nfcu(&self) -> HostNfcuR {
        HostNfcuR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Data Burst retry count is set to this field. '00' Retry Disabled '01' 1 time '10' 2 times '11' 3 times"]
    #[inline(always)]
    pub fn retry_count(&self) -> RetryCountR {
        RetryCountR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 32:39 - 39:32\\]
The largest value of N_DATA_GAP capabilities among the Host Controller and Connected Devices is set to this field. 00h - No Gap 01h - 1 LSS 02h - 2 LSS 03h - 3 LSS ...... ...... FFh - 255 LSS"]
    #[inline(always)]
    pub fn n_data_gap(&self) -> NDataGapR {
        NDataGapR::new(((self.bits >> 32) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
Host Driver sets the number of blocks in Data Burst \\[Flow Control\\]
to this field.The value shall be smaller than or equal to N_FCU capabilities among the Host Controller and connected card and devices. Setting 1 to 4 blocks is recommended considering buffer size. 00h - 256 Blocks 01h - 1 Block 02h - 2 Blocks 03h - 3 Blocks ...... ...... FFh - 255 Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn host_nfcu(&mut self) -> HostNfcuW<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec> {
        HostNfcuW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Data Burst retry count is set to this field. '00' Retry Disabled '01' 1 time '10' 2 times '11' 3 times"]
    #[inline(always)]
    #[must_use]
    pub fn retry_count(&mut self) -> RetryCountW<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec> {
        RetryCountW::new(self, 16)
    }
    #[doc = "Bits 32:39 - 39:32\\]
The largest value of N_DATA_GAP capabilities among the Host Controller and Connected Devices is set to this field. 00h - No Gap 01h - 1 LSS 02h - 2 LSS 03h - 3 LSS ...... ...... FFh - 255 LSS"]
    #[inline(always)]
    #[must_use]
    pub fn n_data_gap(&mut self) -> NDataGapW<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec> {
        NDataGapW::new(self, 32)
    }
}
#[doc = "Start Address of LINK/TRAN settings is pointed by Pointer for UHS-II Setting Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_settings to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec {
    const RESET_VALUE: u64 = 0;
}
