#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_254` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_254` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec>;
#[doc = "Field `MR11_DATA_F1_0` reader - 7:0\\]
Data to program into memory mode register 11. FC=1"]
pub type Mr11DataF1_0R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F1_0` writer - 7:0\\]
Data to program into memory mode register 11. FC=1"]
pub type Mr11DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR11_DATA_F2_0` reader - 15:8\\]
Data to program into memory mode register 11. FC=2"]
pub type Mr11DataF2_0R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F2_0` writer - 15:8\\]
Data to program into memory mode register 11. FC=2"]
pub type Mr11DataF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR11_DATA_F0_1` reader - 23:16\\]
Data to program into memory mode register 11. FC=0"]
pub type Mr11DataF0_1R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F0_1` writer - 23:16\\]
Data to program into memory mode register 11. FC=0"]
pub type Mr11DataF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR11_DATA_F1_1` reader - 31:24\\]
Data to program into memory mode register 11. FC=1"]
pub type Mr11DataF1_1R = crate::FieldReader;
#[doc = "Field `MR11_DATA_F1_1` writer - 31:24\\]
Data to program into memory mode register 11. FC=1"]
pub type Mr11DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 11. FC=1"]
    #[inline(always)]
    pub fn mr11_data_f1_0(&self) -> Mr11DataF1_0R {
        Mr11DataF1_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 11. FC=2"]
    #[inline(always)]
    pub fn mr11_data_f2_0(&self) -> Mr11DataF2_0R {
        Mr11DataF2_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 11. FC=0"]
    #[inline(always)]
    pub fn mr11_data_f0_1(&self) -> Mr11DataF0_1R {
        Mr11DataF0_1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 11. FC=1"]
    #[inline(always)]
    pub fn mr11_data_f1_1(&self) -> Mr11DataF1_1R {
        Mr11DataF1_1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 11. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f1_0(&mut self) -> Mr11DataF1_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec> {
        Mr11DataF1_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 11. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f2_0(&mut self) -> Mr11DataF2_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec> {
        Mr11DataF2_0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 11. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f0_1(&mut self) -> Mr11DataF0_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec> {
        Mr11DataF0_1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 11. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn mr11_data_f1_1(&mut self) -> Mr11DataF1_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec> {
        Mr11DataF1_1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_254\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_254::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_254::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_254::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_254::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_254 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl254Spec {
    const RESET_VALUE: u32 = 0;
}
