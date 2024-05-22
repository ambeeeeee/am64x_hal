#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_phy_cap` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_phy_cap` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec>;
#[doc = "Field `SPEED_RANGE` reader - 7:6\\]
This field indicates supported Speed Range by the Host Controller '00' Range A \\[Default\\]
'01' Range A and Range B '10' Reserved '11' Reserved"]
pub type SpeedRangeR = crate::FieldReader;
#[doc = "Field `SPEED_RANGE` writer - 7:6\\]
This field indicates supported Speed Range by the Host Controller '00' Range A \\[Default\\]
'01' Range A and Range B '10' Reserved '11' Reserved"]
pub type SpeedRangeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `N_LSS_SYN` reader - 19:16\\]
This field indicates the minimum N_LSS_SYN required by the Host Controller. 0h - 4 x16 LSS 1h - 4 x 1 LSS 2h - 4 x 2 LSS 3h - 4 x 3 LSS ...... ...... Fh - 4 x 15 LSS"]
pub type NLssSynR = crate::FieldReader;
#[doc = "Field `N_LSS_SYN` writer - 19:16\\]
This field indicates the minimum N_LSS_SYN required by the Host Controller. 0h - 4 x16 LSS 1h - 4 x 1 LSS 2h - 4 x 2 LSS 3h - 4 x 3 LSS ...... ...... Fh - 4 x 15 LSS"]
pub type NLssSynW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `N_LSS_DIR` reader - 23:20\\]
This field indicates the minimum N_LSS_DIR required by the Host Controller. 0h - 4 x16 LSS 1h - 4 x 1 LSS 2h - 4 x 2 LSS 3h - 4 x 3 LSS ...... ...... Fh - 4 x 15 LSS"]
pub type NLssDirR = crate::FieldReader;
#[doc = "Field `N_LSS_DIR` writer - 23:20\\]
This field indicates the minimum N_LSS_DIR required by the Host Controller. 0h - 4 x16 LSS 1h - 4 x 1 LSS 2h - 4 x 2 LSS 3h - 4 x 3 LSS ...... ...... Fh - 4 x 15 LSS"]
pub type NLssDirW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 6:7 - 7:6\\]
This field indicates supported Speed Range by the Host Controller '00' Range A \\[Default\\]
'01' Range A and Range B '10' Reserved '11' Reserved"]
    #[inline(always)]
    pub fn speed_range(&self) -> SpeedRangeR {
        SpeedRangeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
This field indicates the minimum N_LSS_SYN required by the Host Controller. 0h - 4 x16 LSS 1h - 4 x 1 LSS 2h - 4 x 2 LSS 3h - 4 x 3 LSS ...... ...... Fh - 4 x 15 LSS"]
    #[inline(always)]
    pub fn n_lss_syn(&self) -> NLssSynR {
        NLssSynR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
This field indicates the minimum N_LSS_DIR required by the Host Controller. 0h - 4 x16 LSS 1h - 4 x 1 LSS 2h - 4 x 2 LSS 3h - 4 x 3 LSS ...... ...... Fh - 4 x 15 LSS"]
    #[inline(always)]
    pub fn n_lss_dir(&self) -> NLssDirR {
        NLssDirR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - 7:6\\]
This field indicates supported Speed Range by the Host Controller '00' Range A \\[Default\\]
'01' Range A and Range B '10' Reserved '11' Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn speed_range(&mut self) -> SpeedRangeW<SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec> {
        SpeedRangeW::new(self, 6)
    }
    #[doc = "Bits 16:19 - 19:16\\]
This field indicates the minimum N_LSS_SYN required by the Host Controller. 0h - 4 x16 LSS 1h - 4 x 1 LSS 2h - 4 x 2 LSS 3h - 4 x 3 LSS ...... ...... Fh - 4 x 15 LSS"]
    #[inline(always)]
    #[must_use]
    pub fn n_lss_syn(&mut self) -> NLssSynW<SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec> {
        NLssSynW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
This field indicates the minimum N_LSS_DIR required by the Host Controller. 0h - 4 x16 LSS 1h - 4 x 1 LSS 2h - 4 x 2 LSS 3h - 4 x 3 LSS ...... ...... Fh - 4 x 15 LSS"]
    #[inline(always)]
    #[must_use]
    pub fn n_lss_dir(&mut self) -> NLssDirW<SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec> {
        NLssDirW::new(self, 20)
    }
}
#[doc = "Start Address of PHY Capabilities is pointed by Pointer for UHS-II Host Capabilities Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_phy_cap to value 0x0011_0000"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec {
    const RESET_VALUE: u32 = 0x0011_0000;
}
