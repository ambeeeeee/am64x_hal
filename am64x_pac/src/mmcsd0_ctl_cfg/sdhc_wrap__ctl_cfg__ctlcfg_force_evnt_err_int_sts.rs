#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_Err_Int_Sts` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_Err_Int_Sts` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec>;
#[doc = "Field `CMD_TIMEOUT` reader - 0:0\\]
Force Event for CMD Timeout Error."]
pub type CmdTimeoutR = crate::BitReader;
#[doc = "Field `CMD_TIMEOUT` writer - 0:0\\]
Force Event for CMD Timeout Error."]
pub type CmdTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC` reader - 1:1\\]
Force Event for Command CRC Error."]
pub type CmdCrcR = crate::BitReader;
#[doc = "Field `CMD_CRC` writer - 1:1\\]
Force Event for Command CRC Error."]
pub type CmdCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_ENDBIT` reader - 2:2\\]
Force Event for Command End Bit Error."]
pub type CmdEndbitR = crate::BitReader;
#[doc = "Field `CMD_ENDBIT` writer - 2:2\\]
Force Event for Command End Bit Error."]
pub type CmdEndbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_INDEX` reader - 3:3\\]
Force Event for Command Index Error"]
pub type CmdIndexR = crate::BitReader;
#[doc = "Field `CMD_INDEX` writer - 3:3\\]
Force Event for Command Index Error"]
pub type CmdIndexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_TIMEOUT` reader - 4:4\\]
Force Event for Data Timeout Error."]
pub type DatTimeoutR = crate::BitReader;
#[doc = "Field `DAT_TIMEOUT` writer - 4:4\\]
Force Event for Data Timeout Error."]
pub type DatTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_CRC` reader - 5:5\\]
Force Event for Data CRC Error."]
pub type DatCrcR = crate::BitReader;
#[doc = "Field `DAT_CRC` writer - 5:5\\]
Force Event for Data CRC Error."]
pub type DatCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_ENDBIT` reader - 6:6\\]
Force Event for Data End Bit Error."]
pub type DatEndbitR = crate::BitReader;
#[doc = "Field `DAT_ENDBIT` writer - 6:6\\]
Force Event for Data End Bit Error."]
pub type DatEndbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURR_LIM` reader - 7:7\\]
Force Event for Current Limit Error."]
pub type CurrLimR = crate::BitReader;
#[doc = "Field `CURR_LIM` writer - 7:7\\]
Force Event for Current Limit Error."]
pub type CurrLimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD` reader - 8:8\\]
Force Event for Auto CMD Error."]
pub type AutoCmdR = crate::BitReader;
#[doc = "Field `AUTO_CMD` writer - 8:8\\]
Force Event for Auto CMD Error."]
pub type AutoCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA` reader - 9:9\\]
Force Event for ADMA Error."]
pub type AdmaR = crate::BitReader;
#[doc = "Field `ADMA` writer - 9:9\\]
Force Event for ADMA Error."]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING` reader - 10:10\\]
Force Event for Tuning Error."]
pub type TuningR = crate::BitReader;
#[doc = "Field `TUNING` writer - 10:10\\]
Force Event for Tuning Error."]
pub type TuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP` reader - 11:11\\]
Force Event for Response Error"]
pub type RespR = crate::BitReader;
#[doc = "Field `RESP` writer - 11:11\\]
Force Event for Response Error"]
pub type RespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST` reader - 12:12\\]
Force Event for Host Error"]
pub type HostR = crate::BitReader;
#[doc = "Field `HOST` writer - 12:12\\]
Force Event for Host Error"]
pub type HostW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Force Event for CMD Timeout Error."]
    #[inline(always)]
    pub fn cmd_timeout(&self) -> CmdTimeoutR {
        CmdTimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force Event for Command CRC Error."]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CmdCrcR {
        CmdCrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Force Event for Command End Bit Error."]
    #[inline(always)]
    pub fn cmd_endbit(&self) -> CmdEndbitR {
        CmdEndbitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Event for Command Index Error"]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Event for Data Timeout Error."]
    #[inline(always)]
    pub fn dat_timeout(&self) -> DatTimeoutR {
        DatTimeoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Event for Data CRC Error."]
    #[inline(always)]
    pub fn dat_crc(&self) -> DatCrcR {
        DatCrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Force Event for Data End Bit Error."]
    #[inline(always)]
    pub fn dat_endbit(&self) -> DatEndbitR {
        DatEndbitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Force Event for Current Limit Error."]
    #[inline(always)]
    pub fn curr_lim(&self) -> CurrLimR {
        CurrLimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Force Event for Auto CMD Error."]
    #[inline(always)]
    pub fn auto_cmd(&self) -> AutoCmdR {
        AutoCmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Force Event for ADMA Error."]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Force Event for Tuning Error."]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Force Event for Response Error"]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Force Event for Host Error"]
    #[inline(always)]
    pub fn host(&self) -> HostR {
        HostR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Force Event for CMD Timeout Error."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_timeout(&mut self) -> CmdTimeoutW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        CmdTimeoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force Event for Command CRC Error."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc(&mut self) -> CmdCrcW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        CmdCrcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Force Event for Command End Bit Error."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_endbit(&mut self) -> CmdEndbitW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        CmdEndbitW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Event for Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CmdIndexW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        CmdIndexW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Event for Data Timeout Error."]
    #[inline(always)]
    #[must_use]
    pub fn dat_timeout(&mut self) -> DatTimeoutW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        DatTimeoutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Event for Data CRC Error."]
    #[inline(always)]
    #[must_use]
    pub fn dat_crc(&mut self) -> DatCrcW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        DatCrcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Force Event for Data End Bit Error."]
    #[inline(always)]
    #[must_use]
    pub fn dat_endbit(&mut self) -> DatEndbitW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        DatEndbitW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Force Event for Current Limit Error."]
    #[inline(always)]
    #[must_use]
    pub fn curr_lim(&mut self) -> CurrLimW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        CurrLimW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Force Event for Auto CMD Error."]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd(&mut self) -> AutoCmdW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        AutoCmdW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Force Event for ADMA Error."]
    #[inline(always)]
    #[must_use]
    pub fn adma(&mut self) -> AdmaW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Force Event for Tuning Error."]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TuningW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        TuningW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Force Event for Response Error"]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RespW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        RespW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Force Event for Host Error"]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HostW<SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec> {
        HostW::new(self, 12)
    }
}
#[doc = "This register is not physically implemented, rather it is an address where Error Interrupt Status register can be written.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_Err_Int_Sts to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec {
    const RESET_VALUE: u16 = 0;
}
