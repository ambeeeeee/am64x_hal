#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1307` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1307` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT0_0` reader - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit0_0R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT0_0` writer - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit0_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT1_0` reader - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit1_0R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT1_0` writer - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT2_0` reader - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit2_0R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT2_0` writer - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit2_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT3_0` reader - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit3_0R = crate::FieldReader;
#[doc = "Field `PHY_CS_ACS_ALLOCATION_BIT3_0` writer - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
pub type PhyCsAcsAllocationBit3_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit0_0(&self) -> PhyCsAcsAllocationBit0_0R {
        PhyCsAcsAllocationBit0_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit1_0(&self) -> PhyCsAcsAllocationBit1_0R {
        PhyCsAcsAllocationBit1_0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit2_0(&self) -> PhyCsAcsAllocationBit2_0R {
        PhyCsAcsAllocationBit2_0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    pub fn phy_cs_acs_allocation_bit3_0(&self) -> PhyCsAcsAllocationBit3_0R {
        PhyCsAcsAllocationBit3_0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit0, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit0_0(
        &mut self,
    ) -> PhyCsAcsAllocationBit0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec> {
        PhyCsAcsAllocationBit0_0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit1, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit1_0(
        &mut self,
    ) -> PhyCsAcsAllocationBit1_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec> {
        PhyCsAcsAllocationBit1_0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit2 , 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit2_0(
        &mut self,
    ) -> PhyCsAcsAllocationBit2_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec> {
        PhyCsAcsAllocationBit2_0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
The map for which chip select is associated with each bit in the adrctl slice 0. Bit \\[n\\], 1 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is allocated on ACS_0 bit3, 0 means cs\\[n\\]'s signal\\[CS/CKE/ODT/RST\\]
is not tranfser on ACS_0, if the accroding cs\\[n\\]'s training is not enabled, need to set the value to all 1s."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cs_acs_allocation_bit3_0(
        &mut self,
    ) -> PhyCsAcsAllocationBit3_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec> {
        PhyCsAcsAllocationBit3_0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1307\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1307::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1307::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1307::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1307::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1307 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1307Spec {
    const RESET_VALUE: u32 = 0;
}
