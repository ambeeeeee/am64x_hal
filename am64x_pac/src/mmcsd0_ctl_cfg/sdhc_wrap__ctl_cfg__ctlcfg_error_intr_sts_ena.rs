#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts_ena` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts_ena` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec>;
#[doc = "Field `CMD_TIMEOUT` reader - 0:0\\]
'0' Masked '1' Enabled"]
pub type CmdTimeoutR = crate::BitReader;
#[doc = "Field `CMD_TIMEOUT` writer - 0:0\\]
'0' Masked '1' Enabled"]
pub type CmdTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC` reader - 1:1\\]
'0' Masked '1' Enabled"]
pub type CmdCrcR = crate::BitReader;
#[doc = "Field `CMD_CRC` writer - 1:1\\]
'0' Masked '1' Enabled"]
pub type CmdCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_ENDBIT` reader - 2:2\\]
'0' Masked '1' Enabled"]
pub type CmdEndbitR = crate::BitReader;
#[doc = "Field `CMD_ENDBIT` writer - 2:2\\]
'0' Masked '1' Enabled"]
pub type CmdEndbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_INDEX` reader - 3:3\\]
'0' Masked '1' Enabled"]
pub type CmdIndexR = crate::BitReader;
#[doc = "Field `CMD_INDEX` writer - 3:3\\]
'0' Masked '1' Enabled"]
pub type CmdIndexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TIMEOUT` reader - 4:4\\]
'0' Masked '1' Enabled"]
pub type DataTimeoutR = crate::BitReader;
#[doc = "Field `DATA_TIMEOUT` writer - 4:4\\]
'0' Masked '1' Enabled"]
pub type DataTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CRC` reader - 5:5\\]
'0' Masked '1' Enabled"]
pub type DataCrcR = crate::BitReader;
#[doc = "Field `DATA_CRC` writer - 5:5\\]
'0' Masked '1' Enabled"]
pub type DataCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_ENDBIT` reader - 6:6\\]
'0' Masked '1' Enabled"]
pub type DataEndbitR = crate::BitReader;
#[doc = "Field `DATA_ENDBIT` writer - 6:6\\]
'0' Masked '1' Enabled"]
pub type DataEndbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURR_LIMIT` reader - 7:7\\]
'0' Masked '1' Enabled"]
pub type CurrLimitR = crate::BitReader;
#[doc = "Field `CURR_LIMIT` writer - 7:7\\]
'0' Masked '1' Enabled"]
pub type CurrLimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD` reader - 8:8\\]
'0' Masked '1' Enabled"]
pub type AutoCmdR = crate::BitReader;
#[doc = "Field `AUTO_CMD` writer - 8:8\\]
'0' Masked '1' Enabled"]
pub type AutoCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA` reader - 9:9\\]
'0' Masked '1' Enabled"]
pub type AdmaR = crate::BitReader;
#[doc = "Field `ADMA` writer - 9:9\\]
'0' Masked '1' Enabled"]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING` reader - 10:10\\]
'0' Masked '1' Enabled"]
pub type TuningR = crate::BitReader;
#[doc = "Field `TUNING` writer - 10:10\\]
'0' Masked '1' Enabled"]
pub type TuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP` reader - 11:11\\]
'0' Masked '1' Enabled"]
pub type RespR = crate::BitReader;
#[doc = "Field `RESP` writer - 11:11\\]
'0' Masked '1' Enabled"]
pub type RespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST` reader - 12:12\\]
'0' Masked '1' Enabled"]
pub type HostR = crate::BitReader;
#[doc = "Field `HOST` writer - 12:12\\]
'0' Masked '1' Enabled"]
pub type HostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_SPECIFIC` reader - 14:13\\]
N/A"]
pub type VendorSpecificR = crate::FieldReader;
#[doc = "Field `VENDOR_SPECIFIC` writer - 14:13\\]
N/A"]
pub type VendorSpecificW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn cmd_timeout(&self) -> CmdTimeoutR {
        CmdTimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CmdCrcR {
        CmdCrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn cmd_endbit(&self) -> CmdEndbitR {
        CmdEndbitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn data_timeout(&self) -> DataTimeoutR {
        DataTimeoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn data_crc(&self) -> DataCrcR {
        DataCrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn data_endbit(&self) -> DataEndbitR {
        DataEndbitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn curr_limit(&self) -> CurrLimitR {
        CurrLimitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn auto_cmd(&self) -> AutoCmdR {
        AutoCmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    pub fn host(&self) -> HostR {
        HostR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
N/A"]
    #[inline(always)]
    pub fn vendor_specific(&self) -> VendorSpecificR {
        VendorSpecificR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_timeout(&mut self) -> CmdTimeoutW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        CmdTimeoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc(&mut self) -> CmdCrcW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        CmdCrcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_endbit(&mut self) -> CmdEndbitW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        CmdEndbitW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CmdIndexW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        CmdIndexW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_timeout(&mut self) -> DataTimeoutW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        DataTimeoutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc(&mut self) -> DataCrcW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        DataCrcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_endbit(&mut self) -> DataEndbitW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        DataEndbitW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn curr_limit(&mut self) -> CurrLimitW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        CurrLimitW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd(&mut self) -> AutoCmdW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        AutoCmdW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn adma(&mut self) -> AdmaW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TuningW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        TuningW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RespW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        RespW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
'0' Masked '1' Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HostW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        HostW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 14:13\\]
N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_specific(
        &mut self,
    ) -> VendorSpecificW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec> {
        VendorSpecificW::new(self, 13)
    }
}
#[doc = "This register is used to enable the Error Interrupt Status register fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts_ena to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec {
    const RESET_VALUE: u16 = 0;
}
