#[doc = "Register `MMR__VBUSP__CFG1_DEVINFO` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1DevinfoSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_DEVINFO` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1DevinfoSpec>;
#[doc = "Field `VD_MAP` reader - 11:8\\]
Indicates the core voltage domain mapping of VTM VD. Device specific field. This field indicates to which SOC cVD is this VD of this VTM map to. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff, d_vd\\[a\\]_vd_map_ipcfg."]
pub type VdMapR = crate::FieldReader;
#[doc = "Field `VD_MAP` writer - 11:8\\]
Indicates the core voltage domain mapping of VTM VD. Device specific field. This field indicates to which SOC cVD is this VD of this VTM map to. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff, d_vd\\[a\\]_vd_map_ipcfg."]
pub type VdMapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AVS0_SUP` reader - 12:12\\]
Indicates VD0 AVS class0 support. Indicates if the cVD associated with this VTM's VD MMR supports AVS-Class0: 0: No AVS-Class0. 1: Supports-AVS-Class0. Reset value is from e-fuse at module reset, efuse_vd\\[a\\]_avs_sup."]
pub type Avs0SupR = crate::BitReader;
#[doc = "Field `AVS0_SUP` writer - 12:12\\]
Indicates VD0 AVS class0 support. Indicates if the cVD associated with this VTM's VD MMR supports AVS-Class0: 0: No AVS-Class0. 1: Supports-AVS-Class0. Reset value is from e-fuse at module reset, efuse_vd\\[a\\]_avs_sup."]
pub type Avs0SupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the core voltage domain mapping of VTM VD. Device specific field. This field indicates to which SOC cVD is this VD of this VTM map to. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff, d_vd\\[a\\]_vd_map_ipcfg."]
    #[inline(always)]
    pub fn vd_map(&self) -> VdMapR {
        VdMapR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Indicates VD0 AVS class0 support. Indicates if the cVD associated with this VTM's VD MMR supports AVS-Class0: 0: No AVS-Class0. 1: Supports-AVS-Class0. Reset value is from e-fuse at module reset, efuse_vd\\[a\\]_avs_sup."]
    #[inline(always)]
    pub fn avs0_sup(&self) -> Avs0SupR {
        Avs0SupR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the core voltage domain mapping of VTM VD. Device specific field. This field indicates to which SOC cVD is this VD of this VTM map to. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff, d_vd\\[a\\]_vd_map_ipcfg."]
    #[inline(always)]
    #[must_use]
    pub fn vd_map(&mut self) -> VdMapW<Mmr_Vbusp_Cfg1DevinfoSpec> {
        VdMapW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Indicates VD0 AVS class0 support. Indicates if the cVD associated with this VTM's VD MMR supports AVS-Class0: 0: No AVS-Class0. 1: Supports-AVS-Class0. Reset value is from e-fuse at module reset, efuse_vd\\[a\\]_avs_sup."]
    #[inline(always)]
    #[must_use]
    pub fn avs0_sup(&mut self) -> Avs0SupW<Mmr_Vbusp_Cfg1DevinfoSpec> {
        Avs0SupW::new(self, 12)
    }
}
#[doc = "Voltage domain a information register. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_devinfo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_devinfo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1DevinfoSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1DevinfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_devinfo::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1DevinfoSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_devinfo::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1DevinfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_DEVINFO to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1DevinfoSpec {
    const RESET_VALUE: u32 = 0;
}
