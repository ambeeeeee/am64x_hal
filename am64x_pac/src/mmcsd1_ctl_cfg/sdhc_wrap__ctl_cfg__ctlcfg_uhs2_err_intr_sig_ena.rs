#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sig_ena` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sig_ena` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec>;
#[doc = "Field `HEADER` reader - 0:0\\]
Setting this bit to 1 enables generating interrupt signal when Header Error bit is set in the UHS-II Error Interrupt Status Register."]
pub type HeaderR = crate::BitReader;
#[doc = "Field `HEADER` writer - 0:0\\]
Setting this bit to 1 enables generating interrupt signal when Header Error bit is set in the UHS-II Error Interrupt Status Register."]
pub type HeaderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_PKT` reader - 1:1\\]
Setting this bit to 1 enables generating interrupt signal when RES Packet Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type RespPktR = crate::BitReader;
#[doc = "Field `RESP_PKT` writer - 1:1\\]
Setting this bit to 1 enables generating interrupt signal when RES Packet Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type RespPktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY_EXPIRED_SIG_ENA` reader - 2:2\\]
Setting this bit to 1 enables generating interrupt signal when Retry Expired bit is set in the UHS-II Error InterruptStatus Register."]
pub type RetryExpiredSigEnaR = crate::BitReader;
#[doc = "Field `RETRY_EXPIRED_SIG_ENA` writer - 2:2\\]
Setting this bit to 1 enables generating interrupt signal when Retry Expired bit is set in the UHS-II Error InterruptStatus Register."]
pub type RetryExpiredSigEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - 3:3\\]
Setting this bit to 1 enables generating interrupt signal when CRC Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - 3:3\\]
Setting this bit to 1 enables generating interrupt signal when CRC Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAMING` reader - 4:4\\]
Setting this bit to 1 enables generating interrupt signal when Framing Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type FramingR = crate::BitReader;
#[doc = "Field `FRAMING` writer - 4:4\\]
Setting this bit to 1 enables generating interrupt signal when Framing Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type FramingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TID` reader - 5:5\\]
Setting this bit to 1 enables generating interrupt signal when TID Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type TidR = crate::BitReader;
#[doc = "Field `TID` writer - 5:5\\]
Setting this bit to 1 enables generating interrupt signal when TID Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type TidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRECOVERABLE` reader - 7:7\\]
Setting this bit to 1 enables generating interrupt signal when Unrecoverable Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type UnrecoverableR = crate::BitReader;
#[doc = "Field `UNRECOVERABLE` writer - 7:7\\]
Setting this bit to 1 enables generating interrupt signal when Unrecoverable Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type UnrecoverableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBSY` reader - 8:8\\]
Setting this bit to 1 enables generating interrupt signal when EBSY Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type EbsyR = crate::BitReader;
#[doc = "Field `EBSY` writer - 8:8\\]
Setting this bit to 1 enables generating interrupt signal when EBSY Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type EbsyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA2_ADMA3` reader - 15:15\\]
Setting this bit to 1 enables generating interrupt signal when ADMA2/3 Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type Adma2Adma3R = crate::BitReader;
#[doc = "Field `ADMA2_ADMA3` writer - 15:15\\]
Setting this bit to 1 enables generating interrupt signal when ADMA2/3 Error bit is set in the UHS-II Error InterruptStatus Register."]
pub type Adma2Adma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_RESP_TIMEOUT` reader - 16:16\\]
Setting this bit to 1 enables generating interrupt signal when Timeout for CMD_RES bit is set in the UHS-II Error InterruptStatus Register."]
pub type CmdRespTimeoutR = crate::BitReader;
#[doc = "Field `CMD_RESP_TIMEOUT` writer - 16:16\\]
Setting this bit to 1 enables generating interrupt signal when Timeout for CMD_RES bit is set in the UHS-II Error InterruptStatus Register."]
pub type CmdRespTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEADLOCK_TIMEOUT` reader - 17:17\\]
Setting this bit to 1 enables generating interrupt signal when Timeout for Dead lock bit is set in the UHS-II Error InterruptStatus Register."]
pub type DeadlockTimeoutR = crate::BitReader;
#[doc = "Field `DEADLOCK_TIMEOUT` writer - 17:17\\]
Setting this bit to 1 enables generating interrupt signal when Timeout for Dead lock bit is set in the UHS-II Error InterruptStatus Register."]
pub type DeadlockTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_SPECFIC` reader - 31:27\\]
Setting of a bit to 1 in this field enables generating interrupt signal when corre-spondent bit of Vendor Specific Error is set in the UHS-II Error Interrupt Status Register. 0h - Interrupt Signal is Disabled 1h - Interrupt Signal is Enabled"]
pub type VendorSpecficR = crate::FieldReader;
#[doc = "Field `VENDOR_SPECFIC` writer - 31:27\\]
Setting of a bit to 1 in this field enables generating interrupt signal when corre-spondent bit of Vendor Specific Error is set in the UHS-II Error Interrupt Status Register. 0h - Interrupt Signal is Disabled 1h - Interrupt Signal is Enabled"]
pub type VendorSpecficW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Setting this bit to 1 enables generating interrupt signal when Header Error bit is set in the UHS-II Error Interrupt Status Register."]
    #[inline(always)]
    pub fn header(&self) -> HeaderR {
        HeaderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Setting this bit to 1 enables generating interrupt signal when RES Packet Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn resp_pkt(&self) -> RespPktR {
        RespPktR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Setting this bit to 1 enables generating interrupt signal when Retry Expired bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn retry_expired_sig_ena(&self) -> RetryExpiredSigEnaR {
        RetryExpiredSigEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Setting this bit to 1 enables generating interrupt signal when CRC Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Setting this bit to 1 enables generating interrupt signal when Framing Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn framing(&self) -> FramingR {
        FramingR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Setting this bit to 1 enables generating interrupt signal when TID Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Setting this bit to 1 enables generating interrupt signal when Unrecoverable Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn unrecoverable(&self) -> UnrecoverableR {
        UnrecoverableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Setting this bit to 1 enables generating interrupt signal when EBSY Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn ebsy(&self) -> EbsyR {
        EbsyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Setting this bit to 1 enables generating interrupt signal when ADMA2/3 Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn adma2_adma3(&self) -> Adma2Adma3R {
        Adma2Adma3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Setting this bit to 1 enables generating interrupt signal when Timeout for CMD_RES bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn cmd_resp_timeout(&self) -> CmdRespTimeoutR {
        CmdRespTimeoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Setting this bit to 1 enables generating interrupt signal when Timeout for Dead lock bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    pub fn deadlock_timeout(&self) -> DeadlockTimeoutR {
        DeadlockTimeoutR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Setting of a bit to 1 in this field enables generating interrupt signal when corre-spondent bit of Vendor Specific Error is set in the UHS-II Error Interrupt Status Register. 0h - Interrupt Signal is Disabled 1h - Interrupt Signal is Enabled"]
    #[inline(always)]
    pub fn vendor_specfic(&self) -> VendorSpecficR {
        VendorSpecficR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Setting this bit to 1 enables generating interrupt signal when Header Error bit is set in the UHS-II Error Interrupt Status Register."]
    #[inline(always)]
    #[must_use]
    pub fn header(&mut self) -> HeaderW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        HeaderW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Setting this bit to 1 enables generating interrupt signal when RES Packet Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn resp_pkt(&mut self) -> RespPktW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        RespPktW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Setting this bit to 1 enables generating interrupt signal when Retry Expired bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn retry_expired_sig_ena(
        &mut self,
    ) -> RetryExpiredSigEnaW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        RetryExpiredSigEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Setting this bit to 1 enables generating interrupt signal when CRC Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        CrcW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Setting this bit to 1 enables generating interrupt signal when Framing Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn framing(&mut self) -> FramingW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        FramingW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Setting this bit to 1 enables generating interrupt signal when TID Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TidW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        TidW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Setting this bit to 1 enables generating interrupt signal when Unrecoverable Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn unrecoverable(&mut self) -> UnrecoverableW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        UnrecoverableW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Setting this bit to 1 enables generating interrupt signal when EBSY Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn ebsy(&mut self) -> EbsyW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        EbsyW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Setting this bit to 1 enables generating interrupt signal when ADMA2/3 Error bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn adma2_adma3(&mut self) -> Adma2Adma3W<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        Adma2Adma3W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Setting this bit to 1 enables generating interrupt signal when Timeout for CMD_RES bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_resp_timeout(
        &mut self,
    ) -> CmdRespTimeoutW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        CmdRespTimeoutW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Setting this bit to 1 enables generating interrupt signal when Timeout for Dead lock bit is set in the UHS-II Error InterruptStatus Register."]
    #[inline(always)]
    #[must_use]
    pub fn deadlock_timeout(
        &mut self,
    ) -> DeadlockTimeoutW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        DeadlockTimeoutW::new(self, 17)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Setting of a bit to 1 in this field enables generating interrupt signal when corre-spondent bit of Vendor Specific Error is set in the UHS-II Error Interrupt Status Register. 0h - Interrupt Signal is Disabled 1h - Interrupt Signal is Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_specfic(
        &mut self,
    ) -> VendorSpecficW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec> {
        VendorSpecficW::new(self, 27)
    }
}
#[doc = "This register is used to generate UHS-II Interrupt signals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sig_ena to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec {
    const RESET_VALUE: u32 = 0;
}
