#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_370` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_370` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec>;
#[doc = "Field `PORT_CMD_ERROR_ADDR_1` reader - 0:0\\]
Address of command that caused the PORT command error. READ-ONLY"]
pub type PortCmdErrorAddr1R = crate::BitReader;
#[doc = "Field `PORT_CMD_ERROR_ADDR_1` writer - 0:0\\]
Address of command that caused the PORT command error. READ-ONLY"]
pub type PortCmdErrorAddr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_CMD_ERROR_ID` reader - 13:8\\]
Source ID of command that caused the PORT command error. READ-ONLY"]
pub type PortCmdErrorIdR = crate::FieldReader;
#[doc = "Field `PORT_CMD_ERROR_ID` writer - 13:8\\]
Source ID of command that caused the PORT command error. READ-ONLY"]
pub type PortCmdErrorIdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PORT_CMD_ERROR_TYPE` reader - 17:16\\]
Type of error and access type that caused the PORT command error. READ-ONLY"]
pub type PortCmdErrorTypeR = crate::FieldReader;
#[doc = "Field `PORT_CMD_ERROR_TYPE` writer - 17:16\\]
Type of error and access type that caused the PORT command error. READ-ONLY"]
pub type PortCmdErrorTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TODTL_2CMD_F0` reader - 31:24\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=0"]
pub type Todtl2cmdF0R = crate::FieldReader;
#[doc = "Field `TODTL_2CMD_F0` writer - 31:24\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=0"]
pub type Todtl2cmdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Address of command that caused the PORT command error. READ-ONLY"]
    #[inline(always)]
    pub fn port_cmd_error_addr_1(&self) -> PortCmdErrorAddr1R {
        PortCmdErrorAddr1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Source ID of command that caused the PORT command error. READ-ONLY"]
    #[inline(always)]
    pub fn port_cmd_error_id(&self) -> PortCmdErrorIdR {
        PortCmdErrorIdR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Type of error and access type that caused the PORT command error. READ-ONLY"]
    #[inline(always)]
    pub fn port_cmd_error_type(&self) -> PortCmdErrorTypeR {
        PortCmdErrorTypeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=0"]
    #[inline(always)]
    pub fn todtl_2cmd_f0(&self) -> Todtl2cmdF0R {
        Todtl2cmdF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Address of command that caused the PORT command error. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn port_cmd_error_addr_1(
        &mut self,
    ) -> PortCmdErrorAddr1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec> {
        PortCmdErrorAddr1W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Source ID of command that caused the PORT command error. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn port_cmd_error_id(
        &mut self,
    ) -> PortCmdErrorIdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec> {
        PortCmdErrorIdW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Type of error and access type that caused the PORT command error. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn port_cmd_error_type(
        &mut self,
    ) -> PortCmdErrorTypeW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec> {
        PortCmdErrorTypeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn todtl_2cmd_f0(&mut self) -> Todtl2cmdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec> {
        Todtl2cmdF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_370\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_370::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_370::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_370::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_370::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_370 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl370Spec {
    const RESET_VALUE: u32 = 0;
}
