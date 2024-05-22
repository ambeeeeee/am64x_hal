#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_ACMD_Err_Sts` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_ACMD_Err_Sts` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec>;
#[doc = "Field `ACMD_NOT_EXEC` reader - 0:0\\]
Force Event for AUTO CMD12 Not Executed."]
pub type AcmdNotExecR = crate::BitReader;
#[doc = "Field `ACMD_NOT_EXEC` writer - 0:0\\]
Force Event for AUTO CMD12 Not Executed."]
pub type AcmdNotExecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - 1:1\\]
Force Event for AUTO CMD Timeout Error."]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - 1:1\\]
Force Event for AUTO CMD Timeout Error."]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - 2:2\\]
Force Event for AUTO CMD Timeout Error."]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - 2:2\\]
Force Event for AUTO CMD Timeout Error."]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDBIT` reader - 3:3\\]
Force Event for AUTO CMD End Bit Error."]
pub type EndbitR = crate::BitReader;
#[doc = "Field `ENDBIT` writer - 3:3\\]
Force Event for AUTO CMD End Bit Error."]
pub type EndbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDEX` reader - 4:4\\]
Force Event for AUTO CMD Index Error.."]
pub type IndexR = crate::BitReader;
#[doc = "Field `INDEX` writer - 4:4\\]
Force Event for AUTO CMD Index Error.."]
pub type IndexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP` reader - 5:5\\]
Force Event for AUTO CMD Response Error.."]
pub type RespR = crate::BitReader;
#[doc = "Field `RESP` writer - 5:5\\]
Force Event for AUTO CMD Response Error.."]
pub type RespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_NOT_ISS` reader - 7:7\\]
Force Event for Command Not Issued by AUTO CMD12 Error."]
pub type CmdNotIssR = crate::BitReader;
#[doc = "Field `CMD_NOT_ISS` writer - 7:7\\]
Force Event for Command Not Issued by AUTO CMD12 Error."]
pub type CmdNotIssW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Force Event for AUTO CMD12 Not Executed."]
    #[inline(always)]
    pub fn acmd_not_exec(&self) -> AcmdNotExecR {
        AcmdNotExecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force Event for AUTO CMD Timeout Error."]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Force Event for AUTO CMD Timeout Error."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Event for AUTO CMD End Bit Error."]
    #[inline(always)]
    pub fn endbit(&self) -> EndbitR {
        EndbitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Event for AUTO CMD Index Error.."]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Event for AUTO CMD Response Error.."]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Force Event for Command Not Issued by AUTO CMD12 Error."]
    #[inline(always)]
    pub fn cmd_not_iss(&self) -> CmdNotIssR {
        CmdNotIssR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Force Event for AUTO CMD12 Not Executed."]
    #[inline(always)]
    #[must_use]
    pub fn acmd_not_exec(&mut self) -> AcmdNotExecW<SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec> {
        AcmdNotExecW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force Event for AUTO CMD Timeout Error."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec> {
        TimeoutW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Force Event for AUTO CMD Timeout Error."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec> {
        CrcW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Event for AUTO CMD End Bit Error."]
    #[inline(always)]
    #[must_use]
    pub fn endbit(&mut self) -> EndbitW<SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec> {
        EndbitW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Event for AUTO CMD Index Error.."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> IndexW<SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec> {
        IndexW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Event for AUTO CMD Response Error.."]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RespW<SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec> {
        RespW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Force Event for Command Not Issued by AUTO CMD12 Error."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_not_iss(&mut self) -> CmdNotIssW<SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec> {
        CmdNotIssW::new(self, 7)
    }
}
#[doc = "This register is not physically implemented, rather it is an address where Auto CMD Error Status register can be written. Writing 1 : set each bit of the Auto CMD12 Error Status Register Writing 0 : no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_ACMD_Err_Sts to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec {
    const RESET_VALUE: u16 = 0;
}
