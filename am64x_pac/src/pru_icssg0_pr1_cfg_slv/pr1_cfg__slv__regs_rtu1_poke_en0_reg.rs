#[doc = "Register `PR1_CFG__SLV__REGS_rtu1_poke_en0_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_rtu1_poke_en0_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec>;
#[doc = "Field `RTU1_POKE_R20_EN` reader - "]
pub type Rtu1PokeR20EnR = crate::FieldReader;
#[doc = "Field `RTU1_POKE_R20_EN` writer - "]
pub type Rtu1PokeR20EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTU1_POKE_R21_EN` reader - "]
pub type Rtu1PokeR21EnR = crate::FieldReader;
#[doc = "Field `RTU1_POKE_R21_EN` writer - "]
pub type Rtu1PokeR21EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTU1_POKE_R22_EN` reader - "]
pub type Rtu1PokeR22EnR = crate::FieldReader;
#[doc = "Field `RTU1_POKE_R22_EN` writer - "]
pub type Rtu1PokeR22EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTU1_POKE_R23_EN` reader - "]
pub type Rtu1PokeR23EnR = crate::FieldReader;
#[doc = "Field `RTU1_POKE_R23_EN` writer - "]
pub type Rtu1PokeR23EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTU1_POKE_R24_EN` reader - "]
pub type Rtu1PokeR24EnR = crate::FieldReader;
#[doc = "Field `RTU1_POKE_R24_EN` writer - "]
pub type Rtu1PokeR24EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTU1_POKE_R25_EN` reader - "]
pub type Rtu1PokeR25EnR = crate::FieldReader;
#[doc = "Field `RTU1_POKE_R25_EN` writer - "]
pub type Rtu1PokeR25EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTU1_POKE_R26_EN` reader - "]
pub type Rtu1PokeR26EnR = crate::FieldReader;
#[doc = "Field `RTU1_POKE_R26_EN` writer - "]
pub type Rtu1PokeR26EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTU1_POKE_R27_EN` reader - "]
pub type Rtu1PokeR27EnR = crate::FieldReader;
#[doc = "Field `RTU1_POKE_R27_EN` writer - "]
pub type Rtu1PokeR27EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rtu1_poke_r20_en(&self) -> Rtu1PokeR20EnR {
        Rtu1PokeR20EnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn rtu1_poke_r21_en(&self) -> Rtu1PokeR21EnR {
        Rtu1PokeR21EnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rtu1_poke_r22_en(&self) -> Rtu1PokeR22EnR {
        Rtu1PokeR22EnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rtu1_poke_r23_en(&self) -> Rtu1PokeR23EnR {
        Rtu1PokeR23EnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rtu1_poke_r24_en(&self) -> Rtu1PokeR24EnR {
        Rtu1PokeR24EnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn rtu1_poke_r25_en(&self) -> Rtu1PokeR25EnR {
        Rtu1PokeR25EnR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rtu1_poke_r26_en(&self) -> Rtu1PokeR26EnR {
        Rtu1PokeR26EnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn rtu1_poke_r27_en(&self) -> Rtu1PokeR27EnR {
        Rtu1PokeR27EnR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn rtu1_poke_r20_en(&mut self) -> Rtu1PokeR20EnW<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec> {
        Rtu1PokeR20EnW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn rtu1_poke_r21_en(&mut self) -> Rtu1PokeR21EnW<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec> {
        Rtu1PokeR21EnW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn rtu1_poke_r22_en(&mut self) -> Rtu1PokeR22EnW<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec> {
        Rtu1PokeR22EnW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn rtu1_poke_r23_en(&mut self) -> Rtu1PokeR23EnW<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec> {
        Rtu1PokeR23EnW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn rtu1_poke_r24_en(&mut self) -> Rtu1PokeR24EnW<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec> {
        Rtu1PokeR24EnW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn rtu1_poke_r25_en(&mut self) -> Rtu1PokeR25EnW<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec> {
        Rtu1PokeR25EnW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn rtu1_poke_r26_en(&mut self) -> Rtu1PokeR26EnW<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec> {
        Rtu1PokeR26EnW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn rtu1_poke_r27_en(&mut self) -> Rtu1PokeR27EnW<Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec> {
        Rtu1PokeR27EnW::new(self, 28)
    }
}
#[doc = "PR1_CFG__SLV__REGS_rtu1_poke_en0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_rtu1_poke_en0_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_rtu1_poke_en0_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_rtu1_poke_en0_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_rtu1_poke_en0_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_rtu1_poke_en0_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec {
    const RESET_VALUE: u32 = 0;
}
