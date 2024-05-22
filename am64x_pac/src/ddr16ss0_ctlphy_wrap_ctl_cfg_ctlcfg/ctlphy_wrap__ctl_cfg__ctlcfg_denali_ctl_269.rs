#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_269` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_269` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec>;
#[doc = "Field `MR16_DATA_1` reader - 7:0\\]
Data to program into memory mode register 16."]
pub type Mr16Data1R = crate::FieldReader;
#[doc = "Field `MR16_DATA_1` writer - 7:0\\]
Data to program into memory mode register 16."]
pub type Mr16Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR17_DATA_0` reader - 15:8\\]
Data to program into memory mode register 17."]
pub type Mr17Data0R = crate::FieldReader;
#[doc = "Field `MR17_DATA_0` writer - 15:8\\]
Data to program into memory mode register 17."]
pub type Mr17Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR17_DATA_1` reader - 23:16\\]
Data to program into memory mode register 17."]
pub type Mr17Data1R = crate::FieldReader;
#[doc = "Field `MR17_DATA_1` writer - 23:16\\]
Data to program into memory mode register 17."]
pub type Mr17Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MR20_DATA_0` reader - 31:24\\]
Data to program into memory mode register 20."]
pub type Mr20Data0R = crate::FieldReader;
#[doc = "Field `MR20_DATA_0` writer - 31:24\\]
Data to program into memory mode register 20."]
pub type Mr20Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 16."]
    #[inline(always)]
    pub fn mr16_data_1(&self) -> Mr16Data1R {
        Mr16Data1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 17."]
    #[inline(always)]
    pub fn mr17_data_0(&self) -> Mr17Data0R {
        Mr17Data0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 17."]
    #[inline(always)]
    pub fn mr17_data_1(&self) -> Mr17Data1R {
        Mr17Data1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 20."]
    #[inline(always)]
    pub fn mr20_data_0(&self) -> Mr20Data0R {
        Mr20Data0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data to program into memory mode register 16."]
    #[inline(always)]
    #[must_use]
    pub fn mr16_data_1(&mut self) -> Mr16Data1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec> {
        Mr16Data1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Data to program into memory mode register 17."]
    #[inline(always)]
    #[must_use]
    pub fn mr17_data_0(&mut self) -> Mr17Data0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec> {
        Mr17Data0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Data to program into memory mode register 17."]
    #[inline(always)]
    #[must_use]
    pub fn mr17_data_1(&mut self) -> Mr17Data1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec> {
        Mr17Data1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Data to program into memory mode register 20."]
    #[inline(always)]
    #[must_use]
    pub fn mr20_data_0(&mut self) -> Mr20Data0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec> {
        Mr20Data0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_269\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_269::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_269::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_269::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_269::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_269 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl269Spec {
    const RESET_VALUE: u32 = 0;
}
