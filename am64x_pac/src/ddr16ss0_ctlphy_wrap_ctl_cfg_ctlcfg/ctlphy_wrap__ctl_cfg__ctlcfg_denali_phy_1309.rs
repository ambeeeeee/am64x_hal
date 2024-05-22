#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1309` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1309` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT0_2` reader - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit0_2R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT0_2` writer - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit0_2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT1_2` reader - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit1_2R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT1_2` writer - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit1_2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT2_2` reader - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit2_2R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT2_2` writer - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit2_2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT3_2` reader - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit3_2R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT3_2` writer - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit3_2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit0_2(&self) -> PhyCsAcsAllocationBit0_2R {
        PhyCsAcsAllocationBit0_2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit1_2(&self) -> PhyCsAcsAllocationBit1_2R {
        PhyCsAcsAllocationBit1_2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit2_2(&self) -> PhyCsAcsAllocationBit2_2R {
        PhyCsAcsAllocationBit2_2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit3_2(&self) -> PhyCsAcsAllocationBit3_2R {
        PhyCsAcsAllocationBit3_2R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit0_2(
        &mut self,
    ) -> PhyCsAcsAllocationBit0_2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec> {
        PhyCsAcsAllocationBit0_2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit1_2(
        &mut self,
    ) -> PhyCsAcsAllocationBit1_2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec> {
        PhyCsAcsAllocationBit1_2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit2_2(
        &mut self,
    ) -> PhyCsAcsAllocationBit2_2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec> {
        PhyCsAcsAllocationBit2_2W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 2. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_2 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_2, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit3_2(
        &mut self,
    ) -> PhyCsAcsAllocationBit3_2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec> {
        PhyCsAcsAllocationBit3_2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1309\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1309::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1309::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1309::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1309::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1309 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1309Spec {
    const RESET_VALUE: u32 = 0;
}
