#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sts` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sts` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec>;
#[doc = "Field `HEADER` reader - 0:0\\]
Setting of this bit means that Header Error occurs in a received packet."]
pub type HeaderR = crate::BitReader;
#[doc = "Field `HEADER` writer - 0:0\\]
Setting of this bit means that Header Error occurs in a received packet."]
pub type HeaderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_PKT` reader - 1:1\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to1 in the UHS- II Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response,this bit is set to 1."]
pub type RespPktR = crate::BitReader;
#[doc = "Field `RESP_PKT` writer - 1:1\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to1 in the UHS- II Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response,this bit is set to 1."]
pub type RespPktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY_EXPIRED` reader - 2:2\\]
Setting of this bit means that Retry Counter Expired Error occurs during data transfer.If this bit is set,either Framing Error or CRC Error in this register shall be set."]
pub type RetryExpiredR = crate::BitReader;
#[doc = "Field `RETRY_EXPIRED` writer - 2:2\\]
Setting of this bit means that Retry Counter Expired Error occurs during data transfer.If this bit is set,either Framing Error or CRC Error in this register shall be set."]
pub type RetryExpiredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - 3:3\\]
Setting of this bit means that CRC Error occurs during a packet receiving."]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - 3:3\\]
Setting of this bit means that CRC Error occurs during a packet receiving."]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAMING` reader - 4:4\\]
Setting of this bit means that Framing Error occurs during a packet receiving."]
pub type FramingR = crate::BitReader;
#[doc = "Field `FRAMING` writer - 4:4\\]
Setting of this bit means that Framing Error occurs during a packet receiving."]
pub type FramingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TID` reader - 5:5\\]
Setting of this bit means that TID Error occurs."]
pub type TidR = crate::BitReader;
#[doc = "Field `TID` writer - 5:5\\]
Setting of this bit means that TID Error occurs."]
pub type TidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRECOVERABLE` reader - 7:7\\]
Setting of this bit means that Unrecoverable Error is set in a packet from a device."]
pub type UnrecoverableR = crate::BitReader;
#[doc = "Field `UNRECOVERABLE` writer - 7:7\\]
Setting of this bit means that Unrecoverable Error is set in a packet from a device."]
pub type UnrecoverableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBSY` reader - 8:8\\]
On receiving EBSY packet, if the packet indicates an error, this bit is set to 1. Setting of this bit also sets Error Interrupt and Transfer Completer together in the Normal Interrupt Status register. This error check is effective for a command with setting EBSY Wait in the UHS-II Transfer Mode register."]
pub type EbsyR = crate::BitReader;
#[doc = "Field `EBSY` writer - 8:8\\]
On receiving EBSY packet, if the packet indicates an error, this bit is set to 1. Setting of this bit also sets Error Interrupt and Transfer Completer together in the Normal Interrupt Status register. This error check is effective for a command with setting EBSY Wait in the UHS-II Transfer Mode register."]
pub type EbsyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA2_ADMA3` reader - 15:15\\]
Setting of this bit means that ADMA2/3 Error occurs in UHS-II mode. ADMA2/3 Error Status is indicated to the ADMA Error Status \\[054h\\], which is defined in the Host spec 3.00."]
pub type Adma2Adma3R = crate::BitReader;
#[doc = "Field `ADMA2_ADMA3` writer - 15:15\\]
Setting of this bit means that ADMA2/3 Error occurs in UHS-II mode. ADMA2/3 Error Status is indicated to the ADMA Error Status \\[054h\\], which is defined in the Host spec 3.00."]
pub type Adma2Adma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_RESP_TIMEOUT` reader - 16:16\\]
Setting of this bit means that RES Packet timeout occurs. Host expects to receive RES packet but not received in a specified timeout \\[5ms\\]. Timeout value is determined by the setting of Timeout Counter Value for CMD_RES in UHS-II Timer Control register."]
pub type CmdRespTimeoutR = crate::BitReader;
#[doc = "Field `CMD_RESP_TIMEOUT` writer - 16:16\\]
Setting of this bit means that RES Packet timeout occurs. Host expects to receive RES packet but not received in a specified timeout \\[5ms\\]. Timeout value is determined by the setting of Timeout Counter Value for CMD_RES in UHS-II Timer Control register."]
pub type CmdRespTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEADLOCK_TIMEOUT` reader - 17:17\\]
Setting of this bit means that deadlock timeout occurs. Host expects to receive a packet but not received in a specified timeout \\[1 second\\]. Timeout value is determined by the setting of Timeout Counter Value for Deadlock in UHS-II Timer Control register."]
pub type DeadlockTimeoutR = crate::BitReader;
#[doc = "Field `DEADLOCK_TIMEOUT` writer - 17:17\\]
Setting of this bit means that deadlock timeout occurs. Host expects to receive a packet but not received in a specified timeout \\[1 second\\]. Timeout value is determined by the setting of Timeout Counter Value for Deadlock in UHS-II Timer Control register."]
pub type DeadlockTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_SPECFIC_ERR` reader - 31:27\\]
Vendor may use this field for vendor specific error status. '0' Interrupt is not generated '1' Vendor Specific Error"]
pub type VendorSpecficErrR = crate::FieldReader;
#[doc = "Field `VENDOR_SPECFIC_ERR` writer - 31:27\\]
Vendor may use this field for vendor specific error status. '0' Interrupt is not generated '1' Vendor Specific Error"]
pub type VendorSpecficErrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Setting of this bit means that Header Error occurs in a received packet."]
    #[inline(always)]
    pub fn header(&self) -> HeaderR {
        HeaderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to1 in the UHS- II Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response,this bit is set to 1."]
    #[inline(always)]
    pub fn resp_pkt(&self) -> RespPktR {
        RespPktR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Setting of this bit means that Retry Counter Expired Error occurs during data transfer.If this bit is set,either Framing Error or CRC Error in this register shall be set."]
    #[inline(always)]
    pub fn retry_expired(&self) -> RetryExpiredR {
        RetryExpiredR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Setting of this bit means that CRC Error occurs during a packet receiving."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Setting of this bit means that Framing Error occurs during a packet receiving."]
    #[inline(always)]
    pub fn framing(&self) -> FramingR {
        FramingR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Setting of this bit means that TID Error occurs."]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Setting of this bit means that Unrecoverable Error is set in a packet from a device."]
    #[inline(always)]
    pub fn unrecoverable(&self) -> UnrecoverableR {
        UnrecoverableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
On receiving EBSY packet, if the packet indicates an error, this bit is set to 1. Setting of this bit also sets Error Interrupt and Transfer Completer together in the Normal Interrupt Status register. This error check is effective for a command with setting EBSY Wait in the UHS-II Transfer Mode register."]
    #[inline(always)]
    pub fn ebsy(&self) -> EbsyR {
        EbsyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Setting of this bit means that ADMA2/3 Error occurs in UHS-II mode. ADMA2/3 Error Status is indicated to the ADMA Error Status \\[054h\\], which is defined in the Host spec 3.00."]
    #[inline(always)]
    pub fn adma2_adma3(&self) -> Adma2Adma3R {
        Adma2Adma3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Setting of this bit means that RES Packet timeout occurs. Host expects to receive RES packet but not received in a specified timeout \\[5ms\\]. Timeout value is determined by the setting of Timeout Counter Value for CMD_RES in UHS-II Timer Control register."]
    #[inline(always)]
    pub fn cmd_resp_timeout(&self) -> CmdRespTimeoutR {
        CmdRespTimeoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Setting of this bit means that deadlock timeout occurs. Host expects to receive a packet but not received in a specified timeout \\[1 second\\]. Timeout value is determined by the setting of Timeout Counter Value for Deadlock in UHS-II Timer Control register."]
    #[inline(always)]
    pub fn deadlock_timeout(&self) -> DeadlockTimeoutR {
        DeadlockTimeoutR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Vendor may use this field for vendor specific error status. '0' Interrupt is not generated '1' Vendor Specific Error"]
    #[inline(always)]
    pub fn vendor_specfic_err(&self) -> VendorSpecficErrR {
        VendorSpecficErrR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Setting of this bit means that Header Error occurs in a received packet."]
    #[inline(always)]
    #[must_use]
    pub fn header(&mut self) -> HeaderW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        HeaderW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to1 in the UHS- II Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response,this bit is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn resp_pkt(&mut self) -> RespPktW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        RespPktW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Setting of this bit means that Retry Counter Expired Error occurs during data transfer.If this bit is set,either Framing Error or CRC Error in this register shall be set."]
    #[inline(always)]
    #[must_use]
    pub fn retry_expired(&mut self) -> RetryExpiredW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        RetryExpiredW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Setting of this bit means that CRC Error occurs during a packet receiving."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        CrcW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Setting of this bit means that Framing Error occurs during a packet receiving."]
    #[inline(always)]
    #[must_use]
    pub fn framing(&mut self) -> FramingW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        FramingW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Setting of this bit means that TID Error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TidW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        TidW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Setting of this bit means that Unrecoverable Error is set in a packet from a device."]
    #[inline(always)]
    #[must_use]
    pub fn unrecoverable(&mut self) -> UnrecoverableW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        UnrecoverableW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
On receiving EBSY packet, if the packet indicates an error, this bit is set to 1. Setting of this bit also sets Error Interrupt and Transfer Completer together in the Normal Interrupt Status register. This error check is effective for a command with setting EBSY Wait in the UHS-II Transfer Mode register."]
    #[inline(always)]
    #[must_use]
    pub fn ebsy(&mut self) -> EbsyW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        EbsyW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Setting of this bit means that ADMA2/3 Error occurs in UHS-II mode. ADMA2/3 Error Status is indicated to the ADMA Error Status \\[054h\\], which is defined in the Host spec 3.00."]
    #[inline(always)]
    #[must_use]
    pub fn adma2_adma3(&mut self) -> Adma2Adma3W<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        Adma2Adma3W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Setting of this bit means that RES Packet timeout occurs. Host expects to receive RES packet but not received in a specified timeout \\[5ms\\]. Timeout value is determined by the setting of Timeout Counter Value for CMD_RES in UHS-II Timer Control register."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_resp_timeout(
        &mut self,
    ) -> CmdRespTimeoutW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        CmdRespTimeoutW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Setting of this bit means that deadlock timeout occurs. Host expects to receive a packet but not received in a specified timeout \\[1 second\\]. Timeout value is determined by the setting of Timeout Counter Value for Deadlock in UHS-II Timer Control register."]
    #[inline(always)]
    #[must_use]
    pub fn deadlock_timeout(
        &mut self,
    ) -> DeadlockTimeoutW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        DeadlockTimeoutW::new(self, 17)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Vendor may use this field for vendor specific error status. '0' Interrupt is not generated '1' Vendor Specific Error"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_specfic_err(
        &mut self,
    ) -> VendorSpecficErrW<SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec> {
        VendorSpecficErrW::new(self, 27)
    }
}
#[doc = "This register gives the status of all UHS-II interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sts to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec {
    const RESET_VALUE: u32 = 0;
}
