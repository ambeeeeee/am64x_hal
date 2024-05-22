#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_46` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_46` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec>;
#[doc = "Field `TRAS_MIN_F0` reader - 8:0\\]
DRAM TRAS_MIN value in cycles. FC=0"]
pub type TrasMinF0R = crate::FieldReader<u16>;
#[doc = "Field `TRAS_MIN_F0` writer - 8:0\\]
DRAM TRAS_MIN value in cycles. FC=0"]
pub type TrasMinF0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TWTR_F0` reader - 21:16\\]
DRAM TWTR value in cycles. FC=0"]
pub type TwtrF0R = crate::FieldReader;
#[doc = "Field `TWTR_F0` writer - 21:16\\]
DRAM TWTR value in cycles. FC=0"]
pub type TwtrF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TWTR_L_F0` reader - 29:24\\]
DRAM TWTR_L value in cycles. FC=0"]
pub type TwtrLF0R = crate::FieldReader;
#[doc = "Field `TWTR_L_F0` writer - 29:24\\]
DRAM TWTR_L value in cycles. FC=0"]
pub type TwtrLF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
DRAM TRAS_MIN value in cycles. FC=0"]
    #[inline(always)]
    pub fn tras_min_f0(&self) -> TrasMinF0R {
        TrasMinF0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM TWTR value in cycles. FC=0"]
    #[inline(always)]
    pub fn twtr_f0(&self) -> TwtrF0R {
        TwtrF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
DRAM TWTR_L value in cycles. FC=0"]
    #[inline(always)]
    pub fn twtr_l_f0(&self) -> TwtrLF0R {
        TwtrLF0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
DRAM TRAS_MIN value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tras_min_f0(&mut self) -> TrasMinF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec> {
        TrasMinF0W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM TWTR value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn twtr_f0(&mut self) -> TwtrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec> {
        TwtrF0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
DRAM TWTR_L value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn twtr_l_f0(&mut self) -> TwtrLF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec> {
        TwtrLF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_46::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_46::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_46 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl46Spec {
    const RESET_VALUE: u32 = 0;
}
