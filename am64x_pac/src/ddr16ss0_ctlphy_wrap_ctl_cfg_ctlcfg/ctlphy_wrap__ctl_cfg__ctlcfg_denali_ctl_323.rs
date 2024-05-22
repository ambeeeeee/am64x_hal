#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_323` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_323` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec>;
#[doc = "Field `AGE_COUNT` reader - 7:0\\]
Initial value of master aging-rate counter for command aging."]
pub type AgeCountR = crate::FieldReader;
#[doc = "Field `AGE_COUNT` writer - 7:0\\]
Initial value of master aging-rate counter for command aging."]
pub type AgeCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COMMAND_AGE_COUNT` reader - 15:8\\]
Initial value of individual command aging counters for command aging."]
pub type CommandAgeCountR = crate::FieldReader;
#[doc = "Field `COMMAND_AGE_COUNT` writer - 15:8\\]
Initial value of individual command aging counters for command aging."]
pub type CommandAgeCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR_CMP_EN` reader - 16:16\\]
Enable address collision detection as a rule for command queue placement. Set to 1 to enable."]
pub type AddrCmpEnR = crate::BitReader;
#[doc = "Field `ADDR_CMP_EN` writer - 16:16\\]
Enable address collision detection as a rule for command queue placement. Set to 1 to enable."]
pub type AddrCmpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_COLLISION_MPM_DIS` reader - 24:24\\]
Disable address collision detection extension using micro page mask for command queue placement and selection. Set to 1 to disable."]
pub type AddrCollisionMpmDisR = crate::BitReader;
#[doc = "Field `ADDR_COLLISION_MPM_DIS` writer - 24:24\\]
Disable address collision detection extension using micro page mask for command queue placement and selection. Set to 1 to disable."]
pub type AddrCollisionMpmDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Initial value of master aging-rate counter for command aging."]
    #[inline(always)]
    pub fn age_count(&self) -> AgeCountR {
        AgeCountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Initial value of individual command aging counters for command aging."]
    #[inline(always)]
    pub fn command_age_count(&self) -> CommandAgeCountR {
        CommandAgeCountR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable address collision detection as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    pub fn addr_cmp_en(&self) -> AddrCmpEnR {
        AddrCmpEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable address collision detection extension using micro page mask for command queue placement and selection. Set to 1 to disable."]
    #[inline(always)]
    pub fn addr_collision_mpm_dis(&self) -> AddrCollisionMpmDisR {
        AddrCollisionMpmDisR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Initial value of master aging-rate counter for command aging."]
    #[inline(always)]
    #[must_use]
    pub fn age_count(&mut self) -> AgeCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec> {
        AgeCountW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Initial value of individual command aging counters for command aging."]
    #[inline(always)]
    #[must_use]
    pub fn command_age_count(
        &mut self,
    ) -> CommandAgeCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec> {
        CommandAgeCountW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable address collision detection as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn addr_cmp_en(&mut self) -> AddrCmpEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec> {
        AddrCmpEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable address collision detection extension using micro page mask for command queue placement and selection. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn addr_collision_mpm_dis(
        &mut self,
    ) -> AddrCollisionMpmDisW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec> {
        AddrCollisionMpmDisW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_323\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_323::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_323::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_323::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_323::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_323 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl323Spec {
    const RESET_VALUE: u32 = 0;
}
