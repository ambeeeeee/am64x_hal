#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_231` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_231` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec>;
#[doc = "Field `PI_RD_DBI_LEVEL_EN_F2` reader - 1:0\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiRdDbiLevelEnF2R = crate::FieldReader;
#[doc = "Field `PI_RD_DBI_LEVEL_EN_F2` writer - 1:0\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiRdDbiLevelEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_TRTP_F0` reader - 15:8\\]
DRAM tRTP value in cycles for frequency set 0."]
pub type PiTrtpF0R = crate::FieldReader;
#[doc = "Field `PI_TRTP_F0` writer - 15:8\\]
DRAM tRTP value in cycles for frequency set 0."]
pub type PiTrtpF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRP_F0` reader - 23:16\\]
DRAM tRP value in cycles for frequency set 0."]
pub type PiTrpF0R = crate::FieldReader;
#[doc = "Field `PI_TRP_F0` writer - 23:16\\]
DRAM tRP value in cycles for frequency set 0."]
pub type PiTrpF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRCD_F0` reader - 31:24\\]
DRAM tRCD value in cycles for frequency set 0."]
pub type PiTrcdF0R = crate::FieldReader;
#[doc = "Field `PI_TRCD_F0` writer - 31:24\\]
DRAM tRCD value in cycles for frequency set 0."]
pub type PiTrcdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    pub fn pi_rd_dbi_level_en_f2(&self) -> PiRdDbiLevelEnF2R {
        PiRdDbiLevelEnF2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tRTP value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_trtp_f0(&self) -> PiTrtpF0R {
        PiTrtpF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tRP value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_trp_f0(&self) -> PiTrpF0R {
        PiTrpF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tRCD value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_trcd_f0(&self) -> PiTrcdF0R {
        PiTrcdF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rd_dbi_level_en_f2(
        &mut self,
    ) -> PiRdDbiLevelEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec> {
        PiRdDbiLevelEnF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tRTP value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trtp_f0(&mut self) -> PiTrtpF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec> {
        PiTrtpF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tRP value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trp_f0(&mut self) -> PiTrpF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec> {
        PiTrpF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tRCD value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trcd_f0(&mut self) -> PiTrcdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec> {
        PiTrcdF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_231\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_231::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_231::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_231::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_231::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_231 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi231Spec {
    const RESET_VALUE: u32 = 0;
}
