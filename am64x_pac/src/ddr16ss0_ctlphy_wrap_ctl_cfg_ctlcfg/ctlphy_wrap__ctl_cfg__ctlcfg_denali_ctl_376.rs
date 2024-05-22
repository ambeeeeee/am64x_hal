#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_376` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_376` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec>;
#[doc = "Field `RD_TO_ODTH_F1` reader - 5:0\\]
Defines the delay from a read command to ODT assertion. FC=1"]
pub type RdToOdthF1R = crate::FieldReader;
#[doc = "Field `RD_TO_ODTH_F1` writer - 5:0\\]
Defines the delay from a read command to ODT assertion. FC=1"]
pub type RdToOdthF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RD_TO_ODTH_F2` reader - 13:8\\]
Defines the delay from a read command to ODT assertion. FC=2"]
pub type RdToOdthF2R = crate::FieldReader;
#[doc = "Field `RD_TO_ODTH_F2` writer - 13:8\\]
Defines the delay from a read command to ODT assertion. FC=2"]
pub type RdToOdthF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RW2MRW_DLY_F0` reader - 20:16\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=0"]
pub type Rw2mrwDlyF0R = crate::FieldReader;
#[doc = "Field `RW2MRW_DLY_F0` writer - 20:16\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=0"]
pub type Rw2mrwDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RW2MRW_DLY_F1` reader - 28:24\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=1"]
pub type Rw2mrwDlyF1R = crate::FieldReader;
#[doc = "Field `RW2MRW_DLY_F1` writer - 28:24\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=1"]
pub type Rw2mrwDlyF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Defines the delay from a read command to ODT assertion. FC=1"]
    #[inline(always)]
    pub fn rd_to_odth_f1(&self) -> RdToOdthF1R {
        RdToOdthF1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the delay from a read command to ODT assertion. FC=2"]
    #[inline(always)]
    pub fn rd_to_odth_f2(&self) -> RdToOdthF2R {
        RdToOdthF2R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=0"]
    #[inline(always)]
    pub fn rw2mrw_dly_f0(&self) -> Rw2mrwDlyF0R {
        Rw2mrwDlyF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=1"]
    #[inline(always)]
    pub fn rw2mrw_dly_f1(&self) -> Rw2mrwDlyF1R {
        Rw2mrwDlyF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Defines the delay from a read command to ODT assertion. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn rd_to_odth_f1(&mut self) -> RdToOdthF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec> {
        RdToOdthF1W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the delay from a read command to ODT assertion. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn rd_to_odth_f2(&mut self) -> RdToOdthF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec> {
        RdToOdthF2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn rw2mrw_dly_f0(&mut self) -> Rw2mrwDlyF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec> {
        Rw2mrwDlyF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn rw2mrw_dly_f1(&mut self) -> Rw2mrwDlyF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec> {
        Rw2mrwDlyF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_376\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_376::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_376::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_376::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_376::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_376 to value 0x0808_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl376Spec {
    const RESET_VALUE: u32 = 0x0808_0000;
}
