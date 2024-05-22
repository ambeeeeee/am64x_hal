#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_415` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_415` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec>;
#[doc = "Field `BANK_ACTIVATE_2TICK_COUNT` reader - 2:0\\]
NEED TO FiLL IN ."]
pub type BankActivate2tickCountR = crate::FieldReader;
#[doc = "Field `BANK_ACTIVATE_2TICK_COUNT` writer - 2:0\\]
NEED TO FiLL IN ."]
pub type BankActivate2tickCountW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRE_2TICK_COUNT` reader - 10:8\\]
NEED TO FiLL IN ."]
pub type Pre2tickCountR = crate::FieldReader;
#[doc = "Field `PRE_2TICK_COUNT` writer - 10:8\\]
NEED TO FiLL IN ."]
pub type Pre2tickCountW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STRATEGY_4TICK_COUNT` reader - 18:16\\]
NEED TO FiLL IN ."]
pub type Strategy4tickCountR = crate::FieldReader;
#[doc = "Field `STRATEGY_4TICK_COUNT` writer - 18:16\\]
NEED TO FiLL IN ."]
pub type Strategy4tickCountW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BANK_ACTIVATE_4TICK_COUNT` reader - 26:24\\]
NEED TO FiLL IN ."]
pub type BankActivate4tickCountR = crate::FieldReader;
#[doc = "Field `BANK_ACTIVATE_4TICK_COUNT` writer - 26:24\\]
NEED TO FiLL IN ."]
pub type BankActivate4tickCountW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn bank_activate_2tick_count(&self) -> BankActivate2tickCountR {
        BankActivate2tickCountR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn pre_2tick_count(&self) -> Pre2tickCountR {
        Pre2tickCountR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn strategy_4tick_count(&self) -> Strategy4tickCountR {
        Strategy4tickCountR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn bank_activate_4tick_count(&self) -> BankActivate4tickCountR {
        BankActivate4tickCountR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn bank_activate_2tick_count(
        &mut self,
    ) -> BankActivate2tickCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec> {
        BankActivate2tickCountW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn pre_2tick_count(&mut self) -> Pre2tickCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec> {
        Pre2tickCountW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn strategy_4tick_count(
        &mut self,
    ) -> Strategy4tickCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec> {
        Strategy4tickCountW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn bank_activate_4tick_count(
        &mut self,
    ) -> BankActivate4tickCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec> {
        BankActivate4tickCountW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_415\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_415::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_415::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_415::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_415::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_415 to value 0x0101_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl415Spec {
    const RESET_VALUE: u32 = 0x0101_0000;
}
