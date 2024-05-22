#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_372` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_372` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec>;
#[doc = "Field `TODTH_RD_F1` reader - 3:0\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=1"]
pub type TodthRdF1R = crate::FieldReader;
#[doc = "Field `TODTH_RD_F1` writer - 3:0\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=1"]
pub type TodthRdF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TODTL_2CMD_F2` reader - 15:8\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=2"]
pub type Todtl2cmdF2R = crate::FieldReader;
#[doc = "Field `TODTL_2CMD_F2` writer - 15:8\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=2"]
pub type Todtl2cmdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TODTH_WR_F2` reader - 19:16\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=2"]
pub type TodthWrF2R = crate::FieldReader;
#[doc = "Field `TODTH_WR_F2` writer - 19:16\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=2"]
pub type TodthWrF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TODTH_RD_F2` reader - 27:24\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=2"]
pub type TodthRdF2R = crate::FieldReader;
#[doc = "Field `TODTH_RD_F2` writer - 27:24\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=2"]
pub type TodthRdF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=1"]
    #[inline(always)]
    pub fn todth_rd_f1(&self) -> TodthRdF1R {
        TodthRdF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=2"]
    #[inline(always)]
    pub fn todtl_2cmd_f2(&self) -> Todtl2cmdF2R {
        Todtl2cmdF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=2"]
    #[inline(always)]
    pub fn todth_wr_f2(&self) -> TodthWrF2R {
        TodthWrF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=2"]
    #[inline(always)]
    pub fn todth_rd_f2(&self) -> TodthRdF2R {
        TodthRdF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn todth_rd_f1(&mut self) -> TodthRdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec> {
        TodthRdF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn todtl_2cmd_f2(&mut self) -> Todtl2cmdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec> {
        Todtl2cmdF2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn todth_wr_f2(&mut self) -> TodthWrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec> {
        TodthWrF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn todth_rd_f2(&mut self) -> TodthRdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec> {
        TodthRdF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_372\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_372::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_372::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_372::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_372::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_372 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl372Spec {
    const RESET_VALUE: u32 = 0;
}
