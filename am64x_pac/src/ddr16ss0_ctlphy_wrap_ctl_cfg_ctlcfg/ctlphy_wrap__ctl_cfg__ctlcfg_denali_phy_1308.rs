#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1308` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1308` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT0_1` reader - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit0_1R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT0_1` writer - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit0_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT1_1` reader - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit1_1R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT1_1` writer - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit1_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT2_1` reader - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit2_1R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT2_1` writer - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit2_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT3_1` reader - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit3_1R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT3_1` writer - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit3_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit0_1(&self) -> PhyCsAcsAllocationBit0_1R {
        PhyCsAcsAllocationBit0_1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit1_1(&self) -> PhyCsAcsAllocationBit1_1R {
        PhyCsAcsAllocationBit1_1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit2_1(&self) -> PhyCsAcsAllocationBit2_1R {
        PhyCsAcsAllocationBit2_1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit3_1(&self) -> PhyCsAcsAllocationBit3_1R {
        PhyCsAcsAllocationBit3_1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit0_1(
        &mut self,
    ) -> PhyCsAcsAllocationBit0_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec> {
        PhyCsAcsAllocationBit0_1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit1_1(
        &mut self,
    ) -> PhyCsAcsAllocationBit1_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec> {
        PhyCsAcsAllocationBit1_1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit2_1(
        &mut self,
    ) -> PhyCsAcsAllocationBit2_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec> {
        PhyCsAcsAllocationBit2_1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 1. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_1 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_1, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit3_1(
        &mut self,
    ) -> PhyCsAcsAllocationBit3_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec> {
        PhyCsAcsAllocationBit3_1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1308\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1308::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1308::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1308::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1308::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1308 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1308Spec {
    const RESET_VALUE: u32 = 0;
}
