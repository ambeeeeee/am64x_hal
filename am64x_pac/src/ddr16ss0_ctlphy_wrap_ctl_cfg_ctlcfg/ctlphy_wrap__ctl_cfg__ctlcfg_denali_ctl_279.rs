#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_279` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_279` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec>;
#[doc = "Field `FSP_WR_CURRENT` reader - 0:0\\]
Reports which FSP set the memory will target with write commands."]
pub type FspWrCurrentR = crate::BitReader;
#[doc = "Field `FSP_WR_CURRENT` writer - 0:0\\]
Reports which FSP set the memory will target with write commands."]
pub type FspWrCurrentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP0_FRC_VALID` reader - 8:8\\]
Specifies whether the FSP set defined in the FSP0_FRC parameter reflects the frequency used to program the FSP0 registers."]
pub type Fsp0FrcValidR = crate::BitReader;
#[doc = "Field `FSP0_FRC_VALID` writer - 8:8\\]
Specifies whether the FSP set defined in the FSP0_FRC parameter reflects the frequency used to program the FSP0 registers."]
pub type Fsp0FrcValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP1_FRC_VALID` reader - 16:16\\]
Specifies whether the FSP set defined in the FSP1_FRC parameter reflects the frequency used to program the FSP1 registers."]
pub type Fsp1FrcValidR = crate::BitReader;
#[doc = "Field `FSP1_FRC_VALID` writer - 16:16\\]
Specifies whether the FSP set defined in the FSP1_FRC parameter reflects the frequency used to program the FSP1 registers."]
pub type Fsp1FrcValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP0_FRC` reader - 25:24\\]
Identifies which of the controller's frequency copy is associated with FSP0."]
pub type Fsp0FrcR = crate::FieldReader;
#[doc = "Field `FSP0_FRC` writer - 25:24\\]
Identifies which of the controller's frequency copy is associated with FSP0."]
pub type Fsp0FrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reports which FSP set the memory will target with write commands."]
    #[inline(always)]
    pub fn fsp_wr_current(&self) -> FspWrCurrentR {
        FspWrCurrentR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Specifies whether the FSP set defined in the FSP0_FRC parameter reflects the frequency used to program the FSP0 registers."]
    #[inline(always)]
    pub fn fsp0_frc_valid(&self) -> Fsp0FrcValidR {
        Fsp0FrcValidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Specifies whether the FSP set defined in the FSP1_FRC parameter reflects the frequency used to program the FSP1 registers."]
    #[inline(always)]
    pub fn fsp1_frc_valid(&self) -> Fsp1FrcValidR {
        Fsp1FrcValidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Identifies which of the controller's frequency copy is associated with FSP0."]
    #[inline(always)]
    pub fn fsp0_frc(&self) -> Fsp0FrcR {
        Fsp0FrcR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reports which FSP set the memory will target with write commands."]
    #[inline(always)]
    #[must_use]
    pub fn fsp_wr_current(&mut self) -> FspWrCurrentW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec> {
        FspWrCurrentW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Specifies whether the FSP set defined in the FSP0_FRC parameter reflects the frequency used to program the FSP0 registers."]
    #[inline(always)]
    #[must_use]
    pub fn fsp0_frc_valid(&mut self) -> Fsp0FrcValidW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec> {
        Fsp0FrcValidW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Specifies whether the FSP set defined in the FSP1_FRC parameter reflects the frequency used to program the FSP1 registers."]
    #[inline(always)]
    #[must_use]
    pub fn fsp1_frc_valid(&mut self) -> Fsp1FrcValidW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec> {
        Fsp1FrcValidW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Identifies which of the controller's frequency copy is associated with FSP0."]
    #[inline(always)]
    #[must_use]
    pub fn fsp0_frc(&mut self) -> Fsp0FrcW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec> {
        Fsp0FrcW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_279\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_279::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_279::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_279::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_279::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_279 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl279Spec {
    const RESET_VALUE: u32 = 0;
}
