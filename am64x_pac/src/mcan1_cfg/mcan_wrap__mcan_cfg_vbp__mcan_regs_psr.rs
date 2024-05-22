#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_PSR` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsPsrSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_PSR` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsPsrSpec>;
#[doc = "Field `LEC` reader - 2:0\\]
Last Error Code"]
pub type LecR = crate::FieldReader;
#[doc = "Field `LEC` writer - 2:0\\]
Last Error Code"]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ACT` reader - 4:3\\]
Activity"]
pub type ActR = crate::FieldReader;
#[doc = "Field `ACT` writer - 4:3\\]
Activity"]
pub type ActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP` reader - 5:5\\]
Error Passive"]
pub type EpR = crate::BitReader;
#[doc = "Field `EP` writer - 5:5\\]
Error Passive"]
pub type EpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW` reader - 6:6\\]
Warning Status"]
pub type EwR = crate::BitReader;
#[doc = "Field `EW` writer - 6:6\\]
Warning Status"]
pub type EwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BO` reader - 7:7\\]
Bus_Off status"]
pub type BoR = crate::BitReader;
#[doc = "Field `BO` writer - 7:7\\]
Bus_Off status"]
pub type BoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLEC` reader - 10:8\\]
Data Phase Last Error Code"]
pub type DlecR = crate::FieldReader;
#[doc = "Field `DLEC` writer - 10:8\\]
Data Phase Last Error Code"]
pub type DlecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESI` reader - 11:11\\]
ESI flag of last received CAN FD Message"]
pub type ResiR = crate::BitReader;
#[doc = "Field `RESI` writer - 11:11\\]
ESI flag of last received CAN FD Message"]
pub type ResiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBRS` reader - 12:12\\]
BRS flag of last received CAN FD Message"]
pub type RbrsR = crate::BitReader;
#[doc = "Field `RBRS` writer - 12:12\\]
BRS flag of last received CAN FD Message"]
pub type RbrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFDF` reader - 13:13\\]
Received a CAN FD Message"]
pub type RfdfR = crate::BitReader;
#[doc = "Field `RFDF` writer - 13:13\\]
Received a CAN FD Message"]
pub type RfdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXE` reader - 14:14\\]
Protocol Exception Event"]
pub type PxeR = crate::BitReader;
#[doc = "Field `PXE` writer - 14:14\\]
Protocol Exception Event"]
pub type PxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCV` reader - 22:16\\]
Transmitter Delay Compensation Value"]
pub type TdcvR = crate::FieldReader;
#[doc = "Field `TDCV` writer - 22:16\\]
Transmitter Delay Compensation Value"]
pub type TdcvW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Activity"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Bus_Off status"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Data Phase Last Error Code"]
    #[inline(always)]
    pub fn dlec(&self) -> DlecR {
        DlecR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
ESI flag of last received CAN FD Message"]
    #[inline(always)]
    pub fn resi(&self) -> ResiR {
        ResiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
BRS flag of last received CAN FD Message"]
    #[inline(always)]
    pub fn rbrs(&self) -> RbrsR {
        RbrsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Received a CAN FD Message"]
    #[inline(always)]
    pub fn rfdf(&self) -> RfdfR {
        RfdfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Protocol Exception Event"]
    #[inline(always)]
    pub fn pxe(&self) -> PxeR {
        PxeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TdcvR {
        TdcvR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LecW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        LecW::new(self, 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Activity"]
    #[inline(always)]
    #[must_use]
    pub fn act(&mut self) -> ActW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        ActW::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EpW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        EpW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Warning Status"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EwW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        EwW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Bus_Off status"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BoW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        BoW::new(self, 7)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Data Phase Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn dlec(&mut self) -> DlecW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        DlecW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
ESI flag of last received CAN FD Message"]
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> ResiW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        ResiW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
BRS flag of last received CAN FD Message"]
    #[inline(always)]
    #[must_use]
    pub fn rbrs(&mut self) -> RbrsW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        RbrsW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Received a CAN FD Message"]
    #[inline(always)]
    #[must_use]
    pub fn rfdf(&mut self) -> RfdfW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        RfdfW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Protocol Exception Event"]
    #[inline(always)]
    #[must_use]
    pub fn pxe(&mut self) -> PxeW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        PxeW::new(self, 14)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Transmitter Delay Compensation Value"]
    #[inline(always)]
    #[must_use]
    pub fn tdcv(&mut self) -> TdcvW<McanWrap_McanCfgVbp_McanRegsPsrSpec> {
        TdcvW::new(self, 16)
    }
}
#[doc = "CAN protocol controller status, transmitter delay compensation value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsPsrSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsPsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_psr::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsPsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_psr::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsPsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_PSR to value 0x0707"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsPsrSpec {
    const RESET_VALUE: u32 = 0x0707;
}
