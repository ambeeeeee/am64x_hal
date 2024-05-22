#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_INT_RAW_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aIntRawRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_INT_RAW_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aIntRawRegSpec>;
#[doc = "Field `AERR` reader - 1:1\\]
Raw status of VBUSM2AXI interrupt for VBUSM.C address outside the programmed range. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type AerrR = crate::BitReader;
#[doc = "Field `AERR` writer - 1:1\\]
Raw status of VBUSM2AXI interrupt for VBUSM.C address outside the programmed range. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type AerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOERR` reader - 2:2\\]
Raw status of VBUSM2AXI interrupt for controller AXI interface timeout. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type ToerrR = crate::BitReader;
#[doc = "Field `TOERR` writer - 2:2\\]
Raw status of VBUSM2AXI interrupt for controller AXI interface timeout. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type ToerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC1BERR` reader - 3:3\\]
Raw status of SDRAM ECC 1-bit error. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type Ecc1berrR = crate::BitReader;
#[doc = "Field `ECC1BERR` writer - 3:3\\]
Raw status of SDRAM ECC 1-bit error. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type Ecc1berrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC2BERR` reader - 4:4\\]
Raw status of SDRAM ECC 2-bit error. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type Ecc2berrR = crate::BitReader;
#[doc = "Field `ECC2BERR` writer - 4:4\\]
Raw status of SDRAM ECC 2-bit error. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type Ecc2berrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCM1BERR` reader - 5:5\\]
Raw status of SDRAM ECC multi 1-bit errors in same SDRAM burst. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type Eccm1berrR = crate::BitReader;
#[doc = "Field `ECCM1BERR` writer - 5:5\\]
Raw status of SDRAM ECC multi 1-bit errors in same SDRAM burst. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
pub type Eccm1berrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Raw status of VBUSM2AXI interrupt for VBUSM.C address outside the programmed range. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn aerr(&self) -> AerrR {
        AerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw status of VBUSM2AXI interrupt for controller AXI interface timeout. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn toerr(&self) -> ToerrR {
        ToerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw status of SDRAM ECC 1-bit error. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn ecc1berr(&self) -> Ecc1berrR {
        Ecc1berrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Raw status of SDRAM ECC 2-bit error. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn ecc2berr(&self) -> Ecc2berrR {
        Ecc2berrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Raw status of SDRAM ECC multi 1-bit errors in same SDRAM burst. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn eccm1berr(&self) -> Eccm1berrR {
        Eccm1berrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Raw status of VBUSM2AXI interrupt for VBUSM.C address outside the programmed range. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AerrW<Regs_SsCfg_SscfgV2aIntRawRegSpec> {
        AerrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw status of VBUSM2AXI interrupt for controller AXI interface timeout. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn toerr(&mut self) -> ToerrW<Regs_SsCfg_SscfgV2aIntRawRegSpec> {
        ToerrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw status of SDRAM ECC 1-bit error. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ecc1berr(&mut self) -> Ecc1berrW<Regs_SsCfg_SscfgV2aIntRawRegSpec> {
        Ecc1berrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Raw status of SDRAM ECC 2-bit error. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ecc2berr(&mut self) -> Ecc2berrW<Regs_SsCfg_SscfgV2aIntRawRegSpec> {
        Ecc2berrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Raw status of SDRAM ECC multi 1-bit errors in same SDRAM burst. Write 1 to set the (raw) status, mostly for debug. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn eccm1berr(&mut self) -> Eccm1berrW<Regs_SsCfg_SscfgV2aIntRawRegSpec> {
        Eccm1berrW::new(self, 5)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_RAW_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_int_raw_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_int_raw_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aIntRawRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aIntRawRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_int_raw_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aIntRawRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_int_raw_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aIntRawRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_INT_RAW_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aIntRawRegSpec {
    const RESET_VALUE: u32 = 0;
}
