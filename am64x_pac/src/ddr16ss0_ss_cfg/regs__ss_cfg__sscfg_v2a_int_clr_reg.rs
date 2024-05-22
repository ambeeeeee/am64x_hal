#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_INT_CLR_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aIntClrRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_INT_CLR_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aIntClrRegSpec>;
#[doc = "Field `AERR_EN` reader - 1:1\\]
Enable clear for VBUSM2AXI interrupt for VBUSM.C address outside the programmed range. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type AerrEnR = crate::BitReader;
#[doc = "Field `AERR_EN` writer - 1:1\\]
Enable clear for VBUSM2AXI interrupt for VBUSM.C address outside the programmed range. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type AerrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOERR_EN` reader - 2:2\\]
Enable clear for VBUSM2AXI interrupt for controller AXI interface timeout. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type ToerrEnR = crate::BitReader;
#[doc = "Field `TOERR_EN` writer - 2:2\\]
Enable clear for VBUSM2AXI interrupt for controller AXI interface timeout. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type ToerrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC1BERR_EN` reader - 3:3\\]
Enable clear for SDRAM ECC 1-bit error. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type Ecc1berrEnR = crate::BitReader;
#[doc = "Field `ECC1BERR_EN` writer - 3:3\\]
Enable clear for SDRAM ECC 1-bit error. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type Ecc1berrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC2BERR_EN` reader - 4:4\\]
Enable clear for SDRAM ECC 2-bit error. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type Ecc2berrEnR = crate::BitReader;
#[doc = "Field `ECC2BERR_EN` writer - 4:4\\]
Enable clear for SDRAM ECC 2-bit error. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type Ecc2berrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCM1BERR_EN` reader - 5:5\\]
Enable clear for SDRAM ECC multi 1-bit errors in same SDRAM burst. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type Eccm1berrEnR = crate::BitReader;
#[doc = "Field `ECCM1BERR_EN` writer - 5:5\\]
Enable clear for SDRAM ECC multi 1-bit errors in same SDRAM burst. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
pub type Eccm1berrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Enable clear for VBUSM2AXI interrupt for VBUSM.C address outside the programmed range. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn aerr_en(&self) -> AerrEnR {
        AerrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clear for VBUSM2AXI interrupt for controller AXI interface timeout. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn toerr_en(&self) -> ToerrEnR {
        ToerrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clear for SDRAM ECC 1-bit error. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn ecc1berr_en(&self) -> Ecc1berrEnR {
        Ecc1berrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clear for SDRAM ECC 2-bit error. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn ecc2berr_en(&self) -> Ecc2berrEnR {
        Ecc2berrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clear for SDRAM ECC multi 1-bit errors in same SDRAM burst. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn eccm1berr_en(&self) -> Eccm1berrEnR {
        Eccm1berrEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Enable clear for VBUSM2AXI interrupt for VBUSM.C address outside the programmed range. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn aerr_en(&mut self) -> AerrEnW<Regs_SsCfg_SscfgV2aIntClrRegSpec> {
        AerrEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clear for VBUSM2AXI interrupt for controller AXI interface timeout. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn toerr_en(&mut self) -> ToerrEnW<Regs_SsCfg_SscfgV2aIntClrRegSpec> {
        ToerrEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clear for SDRAM ECC 1-bit error. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ecc1berr_en(&mut self) -> Ecc1berrEnW<Regs_SsCfg_SscfgV2aIntClrRegSpec> {
        Ecc1berrEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clear for SDRAM ECC 2-bit error. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ecc2berr_en(&mut self) -> Ecc2berrEnW<Regs_SsCfg_SscfgV2aIntClrRegSpec> {
        Ecc2berrEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clear for SDRAM ECC multi 1-bit errors in same SDRAM burst. Writing a 1 will disable the interrupt, and clear this bit as well as the corresponding Interrupt Enable Set Register. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn eccm1berr_en(&mut self) -> Eccm1berrEnW<Regs_SsCfg_SscfgV2aIntClrRegSpec> {
        Eccm1berrEnW::new(self, 5)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_CLR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_int_clr_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_int_clr_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aIntClrRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aIntClrRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_int_clr_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aIntClrRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_int_clr_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aIntClrRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_INT_CLR_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aIntClrRegSpec {
    const RESET_VALUE: u32 = 0;
}
