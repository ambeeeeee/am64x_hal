#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ECR` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsEcrSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ECR` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsEcrSpec>;
#[doc = "Field `TEC` reader - 7:0\\]
Transmit Error Counter"]
pub type TecR = crate::FieldReader;
#[doc = "Field `TEC` writer - 7:0\\]
Transmit Error Counter"]
pub type TecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REC` reader - 14:8\\]
Receive Error Counter"]
pub type RecR = crate::FieldReader;
#[doc = "Field `REC` writer - 14:8\\]
Receive Error Counter"]
pub type RecW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RP` reader - 15:15\\]
Receive Error Passive"]
pub type RpR = crate::BitReader;
#[doc = "Field `RP` writer - 15:15\\]
Receive Error Passive"]
pub type RpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEL` reader - 23:16\\]
CAN Error Logging"]
pub type CelR = crate::FieldReader;
#[doc = "Field `CEL` writer - 23:16\\]
CAN Error Logging"]
pub type CelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&self) -> RpR {
        RpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
CAN Error Logging"]
    #[inline(always)]
    pub fn cel(&self) -> CelR {
        CelR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TecW<McanWrap_McanCfgVbp_McanRegsEcrSpec> {
        TecW::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Receive Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> RecW<McanWrap_McanCfgVbp_McanRegsEcrSpec> {
        RecW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Receive Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RpW<McanWrap_McanCfgVbp_McanRegsEcrSpec> {
        RpW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
CAN Error Logging"]
    #[inline(always)]
    #[must_use]
    pub fn cel(&mut self) -> CelW<McanWrap_McanCfgVbp_McanRegsEcrSpec> {
        CelW::new(self, 16)
    }
}
#[doc = "State of Rx/Tx Error Counter, CAN Error Logging\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsEcrSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsEcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsEcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ecr::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsEcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ECR to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsEcrSpec {
    const RESET_VALUE: u32 = 0;
}
