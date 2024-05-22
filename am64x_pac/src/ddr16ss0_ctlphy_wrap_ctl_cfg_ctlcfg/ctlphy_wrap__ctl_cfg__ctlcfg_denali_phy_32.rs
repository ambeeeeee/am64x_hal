#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_32` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy32Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_32` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy32Spec>;
#[doc = "Field `PHY_RDLVL_DATA_MASK_0` reader - 7:0\\]
Per-bit mask for read leveling for slice 0. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask0R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_DATA_MASK_0` writer - 7:0\\]
Per-bit mask for read leveling for slice 0. If all bits are not used, only 1 bit should be cleared to 0."]
pub type PhyRdlvlDataMask0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDLVL_DATA_SWIZZLE_0` reader - 25:8\\]
Read level bit swizzling for DDR4 operation for slice 0."]
pub type PhyRdlvlDataSwizzle0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_RDLVL_DATA_SWIZZLE_0` writer - 25:8\\]
Read level bit swizzling for DDR4 operation for slice 0."]
pub type PhyRdlvlDataSwizzle0W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Per-bit mask for read leveling for slice 0. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    pub fn phy_rdlvl_data_mask_0(&self) -> PhyRdlvlDataMask0R {
        PhyRdlvlDataMask0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:25 - 25:8\\]
Read level bit swizzling for DDR4 operation for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_data_swizzle_0(&self) -> PhyRdlvlDataSwizzle0R {
        PhyRdlvlDataSwizzle0R::new((self.bits >> 8) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Per-bit mask for read leveling for slice 0. If all bits are not used, only 1 bit should be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_data_mask_0(
        &mut self,
    ) -> PhyRdlvlDataMask0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy32Spec> {
        PhyRdlvlDataMask0W::new(self, 0)
    }
    #[doc = "Bits 8:25 - 25:8\\]
Read level bit swizzling for DDR4 operation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_data_swizzle_0(
        &mut self,
    ) -> PhyRdlvlDataSwizzle0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy32Spec> {
        PhyRdlvlDataSwizzle0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy32Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_32::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy32Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_32::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_32 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy32Spec {
    const RESET_VALUE: u32 = 0;
}
