#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_377` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_377` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec>;
#[doc = "Field `RW2MRW_DLY_F2` reader - 4:0\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=2"]
pub type Rw2mrwDlyF2R = crate::FieldReader;
#[doc = "Field `RW2MRW_DLY_F2` writer - 4:0\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=2"]
pub type Rw2mrwDlyF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `R2R_DIFFCS_DLY_F0` reader - 12:8\\]
Additional delay to insert between reads to different chip selects. Program to a non-zero value. FC=0"]
pub type R2rDiffcsDlyF0R = crate::FieldReader;
#[doc = "Field `R2R_DIFFCS_DLY_F0` writer - 12:8\\]
Additional delay to insert between reads to different chip selects. Program to a non-zero value. FC=0"]
pub type R2rDiffcsDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `R2W_DIFFCS_DLY_F0` reader - 20:16\\]
Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value. FC=0"]
pub type R2wDiffcsDlyF0R = crate::FieldReader;
#[doc = "Field `R2W_DIFFCS_DLY_F0` writer - 20:16\\]
Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value. FC=0"]
pub type R2wDiffcsDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W2R_DIFFCS_DLY_F0` reader - 28:24\\]
Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system. FC=0"]
pub type W2rDiffcsDlyF0R = crate::FieldReader;
#[doc = "Field `W2R_DIFFCS_DLY_F0` writer - 28:24\\]
Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system. FC=0"]
pub type W2rDiffcsDlyF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=2"]
    #[inline(always)]
    pub fn rw2mrw_dly_f2(&self) -> Rw2mrwDlyF2R {
        Rw2mrwDlyF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Additional delay to insert between reads to different chip selects. Program to a non-zero value. FC=0"]
    #[inline(always)]
    pub fn r2r_diffcs_dly_f0(&self) -> R2rDiffcsDlyF0R {
        R2rDiffcsDlyF0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value. FC=0"]
    #[inline(always)]
    pub fn r2w_diffcs_dly_f0(&self) -> R2wDiffcsDlyF0R {
        R2wDiffcsDlyF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system. FC=0"]
    #[inline(always)]
    pub fn w2r_diffcs_dly_f0(&self) -> W2rDiffcsDlyF0R {
        W2rDiffcsDlyF0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Additional delay to insert between read or write and mode_reg_write. Allowed programming dependent on memory system. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn rw2mrw_dly_f2(&mut self) -> Rw2mrwDlyF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec> {
        Rw2mrwDlyF2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Additional delay to insert between reads to different chip selects. Program to a non-zero value. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn r2r_diffcs_dly_f0(
        &mut self,
    ) -> R2rDiffcsDlyF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec> {
        R2rDiffcsDlyF0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Additional delay to insert between reads and writes to different chip selects. Program to a non-zero value. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn r2w_diffcs_dly_f0(
        &mut self,
    ) -> R2wDiffcsDlyF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec> {
        R2wDiffcsDlyF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Additional delay to insert between writes and reads to different chip selects. Allowed programming dependent on memory system. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn w2r_diffcs_dly_f0(
        &mut self,
    ) -> W2rDiffcsDlyF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec> {
        W2rDiffcsDlyF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_377\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_377::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_377::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_377::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_377::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_377 to value 0x0101_0108"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl377Spec {
    const RESET_VALUE: u32 = 0x0101_0108;
}
