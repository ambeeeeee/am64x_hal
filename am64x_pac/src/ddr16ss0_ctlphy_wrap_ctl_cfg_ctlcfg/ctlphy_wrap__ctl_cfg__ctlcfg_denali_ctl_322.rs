#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_322` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl322Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_322` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl322Spec>;
#[doc = "Field `CS_LOWER_ADDR_EN` reader - 0:0\\]
Enables moving the CS field to lower in the address map. When set to 1, the memory address map will be changed to ROW__CS__BANK. Please refer to the limitations before setting this bit."]
pub type CsLowerAddrEnR = crate::BitReader;
#[doc = "Field `CS_LOWER_ADDR_EN` writer - 0:0\\]
Enables moving the CS field to lower in the address map. When set to 1, the memory address map will be changed to ROW__CS__BANK. Please refer to the limitations before setting this bit."]
pub type CsLowerAddrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APREBIT` reader - 28:24\\]
Location of the auto pre-charge bit in the DRAM address."]
pub type AprebitR = crate::FieldReader;
#[doc = "Field `APREBIT` writer - 28:24\\]
Location of the auto pre-charge bit in the DRAM address."]
pub type AprebitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables moving the CS field to lower in the address map. When set to 1, the memory address map will be changed to ROW__CS__BANK. Please refer to the limitations before setting this bit."]
    #[inline(always)]
    pub fn cs_lower_addr_en(&self) -> CsLowerAddrEnR {
        CsLowerAddrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Location of the auto pre-charge bit in the DRAM address."]
    #[inline(always)]
    pub fn aprebit(&self) -> AprebitR {
        AprebitR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables moving the CS field to lower in the address map. When set to 1, the memory address map will be changed to ROW__CS__BANK. Please refer to the limitations before setting this bit."]
    #[inline(always)]
    #[must_use]
    pub fn cs_lower_addr_en(&mut self) -> CsLowerAddrEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl322Spec> {
        CsLowerAddrEnW::new(self, 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Location of the auto pre-charge bit in the DRAM address."]
    #[inline(always)]
    #[must_use]
    pub fn aprebit(&mut self) -> AprebitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl322Spec> {
        AprebitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_322\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_322::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_322::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl322Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl322Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_322::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl322Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_322::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl322Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_322 to value 0x1000_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl322Spec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
