#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1360` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1360` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec>;
#[doc = "Field `PHY_DATA_BYTE_ORDER_SEL_HIGH` reader - 7:0\\]
Used to define the data slice's byte swap for CA bits 9:8."]
pub type PhyDataByteOrderSelHighR = crate::FieldReader;
#[doc = "Field `PHY_DATA_BYTE_ORDER_SEL_HIGH` writer - 7:0\\]
Used to define the data slice's byte swap for CA bits 9:8."]
pub type PhyDataByteOrderSelHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_CALVL_DEVICE_MAP` reader - 12:8\\]
Define which device's DQ feedback data bits should be used during CA training"]
pub type PhyCalvlDeviceMapR = crate::FieldReader;
#[doc = "Field `PHY_CALVL_DEVICE_MAP` writer - 12:8\\]
Define which device's DQ feedback data bits should be used during CA training"]
pub type PhyCalvlDeviceMapW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_ADR_DISABLE` reader - 18:16\\]
Disable the unused adr slice to save power."]
pub type PhyAdrDisableR = crate::FieldReader;
#[doc = "Field `PHY_ADR_DISABLE` writer - 18:16\\]
Disable the unused adr slice to save power."]
pub type PhyAdrDisableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL_0` reader - 25:24\\]
Select adrctl_mstr_dly_enc for the address/control slice 0 ."]
pub type PhyAdrctlMstrDlyEncSel0R = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_MSTR_DLY_ENC_SEL_0` writer - 25:24\\]
Select adrctl_mstr_dly_enc for the address/control slice 0 ."]
pub type PhyAdrctlMstrDlyEncSel0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Used to define the data slice's byte swap for CA bits 9:8."]
    #[inline(always)]
    pub fn phy_data_byte_order_sel_high(&self) -> PhyDataByteOrderSelHighR {
        PhyDataByteOrderSelHighR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Define which device's DQ feedback data bits should be used during CA training"]
    #[inline(always)]
    pub fn phy_calvl_device_map(&self) -> PhyCalvlDeviceMapR {
        PhyCalvlDeviceMapR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Disable the unused adr slice to save power."]
    #[inline(always)]
    pub fn phy_adr_disable(&self) -> PhyAdrDisableR {
        PhyAdrDisableR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Select adrctl_mstr_dly_enc for the address/control slice 0 ."]
    #[inline(always)]
    pub fn phy_adrctl_mstr_dly_enc_sel_0(&self) -> PhyAdrctlMstrDlyEncSel0R {
        PhyAdrctlMstrDlyEncSel0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Used to define the data slice's byte swap for CA bits 9:8."]
    #[inline(always)]
    #[must_use]
    pub fn phy_data_byte_order_sel_high(
        &mut self,
    ) -> PhyDataByteOrderSelHighW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec> {
        PhyDataByteOrderSelHighW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Define which device's DQ feedback data bits should be used during CA training"]
    #[inline(always)]
    #[must_use]
    pub fn phy_calvl_device_map(
        &mut self,
    ) -> PhyCalvlDeviceMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec> {
        PhyCalvlDeviceMapW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Disable the unused adr slice to save power."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_disable(&mut self) -> PhyAdrDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec> {
        PhyAdrDisableW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Select adrctl_mstr_dly_enc for the address/control slice 0 ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_mstr_dly_enc_sel_0(
        &mut self,
    ) -> PhyAdrctlMstrDlyEncSel0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec> {
        PhyAdrctlMstrDlyEncSel0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1360\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1360::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1360::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1360::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1360::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1360 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1360Spec {
    const RESET_VALUE: u32 = 0;
}
