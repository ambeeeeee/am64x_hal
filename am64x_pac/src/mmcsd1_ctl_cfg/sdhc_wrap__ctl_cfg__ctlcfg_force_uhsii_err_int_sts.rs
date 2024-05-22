#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_force_UHSII_Err_Int_Sts` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_force_UHSII_Err_Int_Sts` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec>;
#[doc = "Field `HEADER` reader - 0:0\\]
Setting this bit forces the Host Controller to set Header Error in the UHS-II Error Interrupt Status register."]
pub type HeaderR = crate::BitReader;
#[doc = "Field `HEADER` writer - 0:0\\]
Setting this bit forces the Host Controller to set Header Error in the UHS-II Error Interrupt Status register."]
pub type HeaderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES_PKT` reader - 1:1\\]
Setting this bit forces the Host Controller to set RES Packet Error in the UHS-II Error Interrupt Status register."]
pub type ResPktR = crate::BitReader;
#[doc = "Field `RES_PKT` writer - 1:1\\]
Setting this bit forces the Host Controller to set RES Packet Error in the UHS-II Error Interrupt Status register."]
pub type ResPktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY_EXPIRED` reader - 2:2\\]
Setting this bit forces the Host Controller to set Retry Expired in the UHS-II Error Interrupt Status register."]
pub type RetryExpiredR = crate::BitReader;
#[doc = "Field `RETRY_EXPIRED` writer - 2:2\\]
Setting this bit forces the Host Controller to set Retry Expired in the UHS-II Error Interrupt Status register."]
pub type RetryExpiredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - 3:3\\]
Setting this bit forces the Host Controller to set CRC Error in the UHS-II Error Interrupt Status register."]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - 3:3\\]
Setting this bit forces the Host Controller to set CRC Error in the UHS-II Error Interrupt Status register."]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAMING` reader - 4:4\\]
Setting this bit forces the Host Controller to set Framing Error in the UHS-II Error Interrupt Status register."]
pub type FramingR = crate::BitReader;
#[doc = "Field `FRAMING` writer - 4:4\\]
Setting this bit forces the Host Controller to set Framing Error in the UHS-II Error Interrupt Status register."]
pub type FramingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TID` reader - 5:5\\]
Setting this bit forces the Host Controller to set TID Error in the UHS-II Error Interrupt Status register."]
pub type TidR = crate::BitReader;
#[doc = "Field `TID` writer - 5:5\\]
Setting this bit forces the Host Controller to set TID Error in the UHS-II Error Interrupt Status register."]
pub type TidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRECOVERABLE` reader - 7:7\\]
Setting this bit forces the Host Controller to set Unrecover-able Error in the UHS-II Error Interrupt Status register."]
pub type UnrecoverableR = crate::BitReader;
#[doc = "Field `UNRECOVERABLE` writer - 7:7\\]
Setting this bit forces the Host Controller to set Unrecover-able Error in the UHS-II Error Interrupt Status register."]
pub type UnrecoverableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBSY` reader - 8:8\\]
Setting this bit forces the Host Controller to set EBSY Error in the UHS-II Error Interrupt Status register."]
pub type EbsyR = crate::BitReader;
#[doc = "Field `EBSY` writer - 8:8\\]
Setting this bit forces the Host Controller to set EBSY Error in the UHS-II Error Interrupt Status register."]
pub type EbsyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA` reader - 15:15\\]
Setting this bit forces the Host Controller to set ADMA Error in the UHS-II Error Interrupt Status register."]
pub type AdmaR = crate::BitReader;
#[doc = "Field `ADMA` writer - 15:15\\]
Setting this bit forces the Host Controller to set ADMA Error in the UHS-II Error Interrupt Status register."]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_CMD_RES` reader - 16:16\\]
Setting this bit forces the Host Controller to set Timeout for CMD_RES in the UHS-II Error Interrupt Status register."]
pub type TimeoutCmdResR = crate::BitReader;
#[doc = "Field `TIMEOUT_CMD_RES` writer - 16:16\\]
Setting this bit forces the Host Controller to set Timeout for CMD_RES in the UHS-II Error Interrupt Status register."]
pub type TimeoutCmdResW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_DEADLOCK` reader - 17:17\\]
Setting this bit forces the Host Controller to set Timeout for Deadlock in the UHS-II Error Interrupt Status register."]
pub type TimeoutDeadlockR = crate::BitReader;
#[doc = "Field `TIMEOUT_DEADLOCK` writer - 17:17\\]
Setting this bit forces the Host Controller to set Timeout for Deadlock in the UHS-II Error Interrupt Status register."]
pub type TimeoutDeadlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_SPECIFIC` reader - 31:27\\]
Force Event for Vendor Specific Error 0h - Not Affected 1h - Vendor Specific Error Status is set"]
pub type VendorSpecificR = crate::FieldReader;
#[doc = "Field `VENDOR_SPECIFIC` writer - 31:27\\]
Force Event for Vendor Specific Error 0h - Not Affected 1h - Vendor Specific Error Status is set"]
pub type VendorSpecificW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Setting this bit forces the Host Controller to set Header Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn header(&self) -> HeaderR {
        HeaderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Setting this bit forces the Host Controller to set RES Packet Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn res_pkt(&self) -> ResPktR {
        ResPktR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Setting this bit forces the Host Controller to set Retry Expired in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn retry_expired(&self) -> RetryExpiredR {
        RetryExpiredR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Setting this bit forces the Host Controller to set CRC Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Setting this bit forces the Host Controller to set Framing Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn framing(&self) -> FramingR {
        FramingR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Setting this bit forces the Host Controller to set TID Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Setting this bit forces the Host Controller to set Unrecover-able Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn unrecoverable(&self) -> UnrecoverableR {
        UnrecoverableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Setting this bit forces the Host Controller to set EBSY Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn ebsy(&self) -> EbsyR {
        EbsyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Setting this bit forces the Host Controller to set ADMA Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Setting this bit forces the Host Controller to set Timeout for CMD_RES in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn timeout_cmd_res(&self) -> TimeoutCmdResR {
        TimeoutCmdResR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Setting this bit forces the Host Controller to set Timeout for Deadlock in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn timeout_deadlock(&self) -> TimeoutDeadlockR {
        TimeoutDeadlockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Force Event for Vendor Specific Error 0h - Not Affected 1h - Vendor Specific Error Status is set"]
    #[inline(always)]
    pub fn vendor_specific(&self) -> VendorSpecificR {
        VendorSpecificR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Setting this bit forces the Host Controller to set Header Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn header(&mut self) -> HeaderW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        HeaderW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Setting this bit forces the Host Controller to set RES Packet Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn res_pkt(&mut self) -> ResPktW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        ResPktW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Setting this bit forces the Host Controller to set Retry Expired in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn retry_expired(
        &mut self,
    ) -> RetryExpiredW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        RetryExpiredW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Setting this bit forces the Host Controller to set CRC Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        CrcW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Setting this bit forces the Host Controller to set Framing Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn framing(&mut self) -> FramingW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        FramingW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Setting this bit forces the Host Controller to set TID Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TidW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        TidW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Setting this bit forces the Host Controller to set Unrecover-able Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn unrecoverable(
        &mut self,
    ) -> UnrecoverableW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        UnrecoverableW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Setting this bit forces the Host Controller to set EBSY Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn ebsy(&mut self) -> EbsyW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        EbsyW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Setting this bit forces the Host Controller to set ADMA Error in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn adma(&mut self) -> AdmaW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        AdmaW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Setting this bit forces the Host Controller to set Timeout for CMD_RES in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn timeout_cmd_res(
        &mut self,
    ) -> TimeoutCmdResW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        TimeoutCmdResW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Setting this bit forces the Host Controller to set Timeout for Deadlock in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn timeout_deadlock(
        &mut self,
    ) -> TimeoutDeadlockW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        TimeoutDeadlockW::new(self, 17)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Force Event for Vendor Specific Error 0h - Not Affected 1h - Vendor Specific Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_specific(
        &mut self,
    ) -> VendorSpecificW<SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec> {
        VendorSpecificW::new(self, 27)
    }
}
#[doc = "This register is not physically implemented, rather it is an address where UHS-II Error Interrupt Status register can be written.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_force_UHSII_Err_Int_Sts to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec {
    const RESET_VALUE: u32 = 0;
}
