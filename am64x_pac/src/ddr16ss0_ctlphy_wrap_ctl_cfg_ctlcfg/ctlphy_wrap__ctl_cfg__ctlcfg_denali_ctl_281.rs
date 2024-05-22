#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_281` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl281Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_281` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl281Spec>;
#[doc = "Field `BIST_DATA_CHECK` reader - 0:0\\]
Enable data checking with BIST operation. Set to 1 to enable."]
pub type BistDataCheckR = crate::BitReader;
#[doc = "Field `BIST_DATA_CHECK` writer - 0:0\\]
Enable data checking with BIST operation. Set to 1 to enable."]
pub type BistDataCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIST_ADDR_CHECK` reader - 8:8\\]
Enable address checking with BIST operation. Set to 1 to enable."]
pub type BistAddrCheckR = crate::BitReader;
#[doc = "Field `BIST_ADDR_CHECK` writer - 8:8\\]
Enable address checking with BIST operation. Set to 1 to enable."]
pub type BistAddrCheckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable data checking with BIST operation. Set to 1 to enable."]
    #[inline(always)]
    pub fn bist_data_check(&self) -> BistDataCheckR {
        BistDataCheckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable address checking with BIST operation. Set to 1 to enable."]
    #[inline(always)]
    pub fn bist_addr_check(&self) -> BistAddrCheckR {
        BistAddrCheckR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable data checking with BIST operation. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn bist_data_check(&mut self) -> BistDataCheckW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl281Spec> {
        BistDataCheckW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable address checking with BIST operation. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn bist_addr_check(&mut self) -> BistAddrCheckW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl281Spec> {
        BistAddrCheckW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_281\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_281::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_281::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl281Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl281Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_281::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl281Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_281::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl281Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_281 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl281Spec {
    const RESET_VALUE: u32 = 0;
}
