#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_cap` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_cap` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec>;
#[doc = "Field `N_FCU` reader - 15:8\\]
This field indicates maximum the number of blocks in a Flow Control unit by the Host Controller.This value is determined by supported buffer size. 00h - 256 Blocks 01h - 1 Block 02h - 2 Block 03h - 3 Block ....... FFh - 255 Blocks"]
pub type NFcuR = crate::FieldReader;
#[doc = "Field `N_FCU` writer - 15:8\\]
This field indicates maximum the number of blocks in a Flow Control unit by the Host Controller.This value is determined by supported buffer size. 00h - 256 Blocks 01h - 1 Block 02h - 2 Block 03h - 3 Block ....... FFh - 255 Blocks"]
pub type NFcuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAX_BLK_LENGTH` reader - 31:20\\]
This field indicates maximum block length by the Host Controller. 000h - Not Used 001h - 1 byte 002h - 2 bytes ...... ...... 200h - 512 bytes ...... ...... 800h - 2048 bytes 801h-FFFh -Not Used"]
pub type MaxBlkLengthR = crate::FieldReader<u16>;
#[doc = "Field `MAX_BLK_LENGTH` writer - 31:20\\]
This field indicates maximum block length by the Host Controller. 000h - Not Used 001h - 1 byte 002h - 2 bytes ...... ...... 200h - 512 bytes ...... ...... 800h - 2048 bytes 801h-FFFh -Not Used"]
pub type MaxBlkLengthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `N_DATA_GAP` reader - 39:32\\]
This field indicates the minimum number of data gap\\[DIDL\\]
supported by the Host Controller. 00h - No Gap 01h - 1 LSS 02h - 2 LSS 03h - 3 LSS ...... ...... FFh - 255 LSS"]
pub type NDataGapR = crate::FieldReader;
#[doc = "Field `N_DATA_GAP` writer - 39:32\\]
This field indicates the minimum number of data gap\\[DIDL\\]
supported by the Host Controller. 00h - No Gap 01h - 1 LSS 02h - 2 LSS 03h - 3 LSS ...... ...... FFh - 255 LSS"]
pub type NDataGapW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
This field indicates maximum the number of blocks in a Flow Control unit by the Host Controller.This value is determined by supported buffer size. 00h - 256 Blocks 01h - 1 Block 02h - 2 Block 03h - 3 Block ....... FFh - 255 Blocks"]
    #[inline(always)]
    pub fn n_fcu(&self) -> NFcuR {
        NFcuR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
This field indicates maximum block length by the Host Controller. 000h - Not Used 001h - 1 byte 002h - 2 bytes ...... ...... 200h - 512 bytes ...... ...... 800h - 2048 bytes 801h-FFFh -Not Used"]
    #[inline(always)]
    pub fn max_blk_length(&self) -> MaxBlkLengthR {
        MaxBlkLengthR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 32:39 - 39:32\\]
This field indicates the minimum number of data gap\\[DIDL\\]
supported by the Host Controller. 00h - No Gap 01h - 1 LSS 02h - 2 LSS 03h - 3 LSS ...... ...... FFh - 255 LSS"]
    #[inline(always)]
    pub fn n_data_gap(&self) -> NDataGapR {
        NDataGapR::new(((self.bits >> 32) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
This field indicates maximum the number of blocks in a Flow Control unit by the Host Controller.This value is determined by supported buffer size. 00h - 256 Blocks 01h - 1 Block 02h - 2 Block 03h - 3 Block ....... FFh - 255 Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn n_fcu(&mut self) -> NFcuW<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec> {
        NFcuW::new(self, 8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
This field indicates maximum block length by the Host Controller. 000h - Not Used 001h - 1 byte 002h - 2 bytes ...... ...... 200h - 512 bytes ...... ...... 800h - 2048 bytes 801h-FFFh -Not Used"]
    #[inline(always)]
    #[must_use]
    pub fn max_blk_length(&mut self) -> MaxBlkLengthW<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec> {
        MaxBlkLengthW::new(self, 20)
    }
    #[doc = "Bits 32:39 - 39:32\\]
This field indicates the minimum number of data gap\\[DIDL\\]
supported by the Host Controller. 00h - No Gap 01h - 1 LSS 02h - 2 LSS 03h - 3 LSS ...... ...... FFh - 255 LSS"]
    #[inline(always)]
    #[must_use]
    pub fn n_data_gap(&mut self) -> NDataGapW<SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec> {
        NDataGapW::new(self, 32)
    }
}
#[doc = "Start Address of LINK/TRAN settings is pointed by Pointer for UHS-II Capabilities Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_cap to value 0x0129_5120_0100"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec {
    const RESET_VALUE: u64 = 0x0129_5120_0100;
}
