#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_68` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_68` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec>;
#[doc = "Field `TRP_AB_F1_1` reader - 7:0\\]
DRAM TRP all bank value in cycles. FC=1"]
pub type TrpAbF1_1R = crate::FieldReader;
#[doc = "Field `TRP_AB_F1_1` writer - 7:0\\]
DRAM TRP all bank value in cycles. FC=1"]
pub type TrpAbF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRP_AB_F2_1` reader - 15:8\\]
DRAM TRP all bank value in cycles. FC=2"]
pub type TrpAbF2_1R = crate::FieldReader;
#[doc = "Field `TRP_AB_F2_1` writer - 15:8\\]
DRAM TRP all bank value in cycles. FC=2"]
pub type TrpAbF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DIMM_ENABLE` reader - 16:16\\]
Enable registered DIMM operation of the controller. Set to 1 to enable."]
pub type RegDimmEnableR = crate::BitReader;
#[doc = "Field `REG_DIMM_ENABLE` writer - 16:16\\]
Enable registered DIMM operation of the controller. Set to 1 to enable."]
pub type RegDimmEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESS_MIRRORING` reader - 25:24\\]
Indicates which chip selects support address mirroring. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable."]
pub type AddressMirroringR = crate::FieldReader;
#[doc = "Field `ADDRESS_MIRRORING` writer - 25:24\\]
Indicates which chip selects support address mirroring. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable."]
pub type AddressMirroringW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP all bank value in cycles. FC=1"]
    #[inline(always)]
    pub fn trp_ab_f1_1(&self) -> TrpAbF1_1R {
        TrpAbF1_1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TRP all bank value in cycles. FC=2"]
    #[inline(always)]
    pub fn trp_ab_f2_1(&self) -> TrpAbF2_1R {
        TrpAbF2_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable registered DIMM operation of the controller. Set to 1 to enable."]
    #[inline(always)]
    pub fn reg_dimm_enable(&self) -> RegDimmEnableR {
        RegDimmEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Indicates which chip selects support address mirroring. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn address_mirroring(&self) -> AddressMirroringR {
        AddressMirroringR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRP all bank value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn trp_ab_f1_1(&mut self) -> TrpAbF1_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec> {
        TrpAbF1_1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TRP all bank value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn trp_ab_f2_1(&mut self) -> TrpAbF2_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec> {
        TrpAbF2_1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable registered DIMM operation of the controller. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn reg_dimm_enable(&mut self) -> RegDimmEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec> {
        RegDimmEnableW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Indicates which chip selects support address mirroring. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn address_mirroring(
        &mut self,
    ) -> AddressMirroringW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec> {
        AddressMirroringW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_68::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_68::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_68::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_68::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_68 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl68Spec {
    const RESET_VALUE: u32 = 0;
}
