#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_371` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_371` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec>;
#[doc = "Field `TODTH_WR_F0` reader - 3:0\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=0"]
pub type TodthWrF0R = crate::FieldReader;
#[doc = "Field `TODTH_WR_F0` writer - 3:0\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=0"]
pub type TodthWrF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TODTH_RD_F0` reader - 11:8\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=0"]
pub type TodthRdF0R = crate::FieldReader;
#[doc = "Field `TODTH_RD_F0` writer - 11:8\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=0"]
pub type TodthRdF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TODTL_2CMD_F1` reader - 23:16\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=1"]
pub type Todtl2cmdF1R = crate::FieldReader;
#[doc = "Field `TODTL_2CMD_F1` writer - 23:16\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=1"]
pub type Todtl2cmdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TODTH_WR_F1` reader - 27:24\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=1"]
pub type TodthWrF1R = crate::FieldReader;
#[doc = "Field `TODTH_WR_F1` writer - 27:24\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=1"]
pub type TodthWrF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=0"]
    #[inline(always)]
    pub fn todth_wr_f0(&self) -> TodthWrF0R {
        TodthWrF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=0"]
    #[inline(always)]
    pub fn todth_rd_f0(&self) -> TodthRdF0R {
        TodthRdF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=1"]
    #[inline(always)]
    pub fn todtl_2cmd_f1(&self) -> Todtl2cmdF1R {
        Todtl2cmdF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=1"]
    #[inline(always)]
    pub fn todth_wr_f1(&self) -> TodthWrF1R {
        TodthWrF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn todth_wr_f0(&mut self) -> TodthWrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec> {
        TodthWrF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a read command. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn todth_rd_f0(&mut self) -> TodthRdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec> {
        TodthRdF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn todtl_2cmd_f1(&mut self) -> Todtl2cmdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec> {
        Todtl2cmdF1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DRAM minimum ODT high time after an ODT assertion for a write command. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn todth_wr_f1(&mut self) -> TodthWrF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec> {
        TodthWrF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_371\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_371::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_371::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_371::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_371::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_371 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl371Spec {
    const RESET_VALUE: u32 = 0;
}
