#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_313` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_313` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec>;
#[doc = "Field `ZQ_REQ` reader - 3:0\\]
User request to initiate a ZQ calibration. Program to 3 for ZQ Start, program to 4 for ZQ Initialization \\[ZQINIT\\], program to 5 for ZQ Latch, or program to 8 for ZQ Reset. Clearing to 0 will not trigger any ZQ command. This parameter should only be written when the ZQ_REQ_PENDING parameter is cleared to 0. WRITE-ONLY"]
pub type ZqReqR = crate::FieldReader;
#[doc = "Field `ZQ_REQ` writer - 3:0\\]
User request to initiate a ZQ calibration. Program to 3 for ZQ Start, program to 4 for ZQ Initialization \\[ZQINIT\\], program to 5 for ZQ Latch, or program to 8 for ZQ Reset. Clearing to 0 will not trigger any ZQ command. This parameter should only be written when the ZQ_REQ_PENDING parameter is cleared to 0. WRITE-ONLY"]
pub type ZqReqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ZQ_REQ_PENDING` reader - 8:8\\]
Indicates that a ZQ command is currently in progress or waiting to run. Value of 1 indicates command in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur. READ-ONLY"]
pub type ZqReqPendingR = crate::BitReader;
#[doc = "Field `ZQ_REQ_PENDING` writer - 8:8\\]
Indicates that a ZQ command is currently in progress or waiting to run. Value of 1 indicates command in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur. READ-ONLY"]
pub type ZqReqPendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZQRESET_F0` reader - 27:16\\]
Number of cycles needed for a ZQRESET command. FC=0"]
pub type ZqresetF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQRESET_F0` writer - 27:16\\]
Number of cycles needed for a ZQRESET command. FC=0"]
pub type ZqresetF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
User request to initiate a ZQ calibration. Program to 3 for ZQ Start, program to 4 for ZQ Initialization \\[ZQINIT\\], program to 5 for ZQ Latch, or program to 8 for ZQ Reset. Clearing to 0 will not trigger any ZQ command. This parameter should only be written when the ZQ_REQ_PENDING parameter is cleared to 0. WRITE-ONLY"]
    #[inline(always)]
    pub fn zq_req(&self) -> ZqReqR {
        ZqReqR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates that a ZQ command is currently in progress or waiting to run. Value of 1 indicates command in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur. READ-ONLY"]
    #[inline(always)]
    pub fn zq_req_pending(&self) -> ZqReqPendingR {
        ZqReqPendingR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Number of cycles needed for a ZQRESET command. FC=0"]
    #[inline(always)]
    pub fn zqreset_f0(&self) -> ZqresetF0R {
        ZqresetF0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
User request to initiate a ZQ calibration. Program to 3 for ZQ Start, program to 4 for ZQ Initialization \\[ZQINIT\\], program to 5 for ZQ Latch, or program to 8 for ZQ Reset. Clearing to 0 will not trigger any ZQ command. This parameter should only be written when the ZQ_REQ_PENDING parameter is cleared to 0. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn zq_req(&mut self) -> ZqReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec> {
        ZqReqW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates that a ZQ command is currently in progress or waiting to run. Value of 1 indicates command in progress or waiting to run. When this is asserted, no writes to ZQ_REQ should occur. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn zq_req_pending(&mut self) -> ZqReqPendingW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec> {
        ZqReqPendingW::new(self, 8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Number of cycles needed for a ZQRESET command. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn zqreset_f0(&mut self) -> ZqresetF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec> {
        ZqresetF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_313\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_313::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_313::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_313::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_313::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_313 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl313Spec {
    const RESET_VALUE: u32 = 0;
}
