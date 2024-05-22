#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_150` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl150Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_150` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl150Spec>;
#[doc = "Field `PPR_COMMAND` reader - 2:0\\]
Specifies the type of PPR command. Program to 1 for pre-charge all, program to 2 for MRW, program to 3 for activate, or program to 5 for write. All other values are reserved. WRITE-ONLY"]
pub type PprCommandR = crate::FieldReader;
#[doc = "Field `PPR_COMMAND` writer - 2:0\\]
Specifies the type of PPR command. Program to 1 for pre-charge all, program to 2 for MRW, program to 3 for activate, or program to 5 for write. All other values are reserved. WRITE-ONLY"]
pub type PprCommandW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPR_COMMAND_MRW_REGNUM` reader - 15:8\\]
Specifies the mode register to be used. Clear to 0 for MRW0 or program to 4 for MRW4. All other values are reserved.."]
pub type PprCommandMrwRegnumR = crate::FieldReader;
#[doc = "Field `PPR_COMMAND_MRW_REGNUM` writer - 15:8\\]
Specifies the mode register to be used. Clear to 0 for MRW0 or program to 4 for MRW4. All other values are reserved.."]
pub type PprCommandMrwRegnumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Specifies the type of PPR command. Program to 1 for pre-charge all, program to 2 for MRW, program to 3 for activate, or program to 5 for write. All other values are reserved. WRITE-ONLY"]
    #[inline(always)]
    pub fn ppr_command(&self) -> PprCommandR {
        PprCommandR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Specifies the mode register to be used. Clear to 0 for MRW0 or program to 4 for MRW4. All other values are reserved.."]
    #[inline(always)]
    pub fn ppr_command_mrw_regnum(&self) -> PprCommandMrwRegnumR {
        PprCommandMrwRegnumR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Specifies the type of PPR command. Program to 1 for pre-charge all, program to 2 for MRW, program to 3 for activate, or program to 5 for write. All other values are reserved. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn ppr_command(&mut self) -> PprCommandW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl150Spec> {
        PprCommandW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Specifies the mode register to be used. Clear to 0 for MRW0 or program to 4 for MRW4. All other values are reserved.."]
    #[inline(always)]
    #[must_use]
    pub fn ppr_command_mrw_regnum(
        &mut self,
    ) -> PprCommandMrwRegnumW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl150Spec> {
        PprCommandMrwRegnumW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_150\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_150::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_150::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl150Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_150::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl150Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_150::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl150Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_150 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl150Spec {
    const RESET_VALUE: u32 = 0;
}
