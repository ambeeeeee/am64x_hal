#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_280` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_280` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec>;
#[doc = "Field `FSP1_FRC` reader - 1:0\\]
Identifies which of the controller's frequency copy is associated with FSP1."]
pub type Fsp1FrcR = crate::FieldReader;
#[doc = "Field `FSP1_FRC` writer - 1:0\\]
Identifies which of the controller's frequency copy is associated with FSP1."]
pub type Fsp1FrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIST_GO` reader - 8:8\\]
Initiate a BIST operation. Set to 1 to trigger. WRITE-ONLY"]
pub type BistGoR = crate::BitReader;
#[doc = "Field `BIST_GO` writer - 8:8\\]
Initiate a BIST operation. Set to 1 to trigger. WRITE-ONLY"]
pub type BistGoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIST_RESULT` reader - 17:16\\]
BIST operation status \\[pass/fail\\]. Bit \\[0\\]
indicates data check status and bit \\[1\\]
indicates address check status. Value of 1 is a passing result. READ-ONLY"]
pub type BistResultR = crate::FieldReader;
#[doc = "Field `BIST_RESULT` writer - 17:16\\]
BIST operation status \\[pass/fail\\]. Bit \\[0\\]
indicates data check status and bit \\[1\\]
indicates address check status. Value of 1 is a passing result. READ-ONLY"]
pub type BistResultW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDR_SPACE` reader - 29:24\\]
Sets the number of address bits to check during BIST operation."]
pub type AddrSpaceR = crate::FieldReader;
#[doc = "Field `ADDR_SPACE` writer - 29:24\\]
Sets the number of address bits to check during BIST operation."]
pub type AddrSpaceW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Identifies which of the controller's frequency copy is associated with FSP1."]
    #[inline(always)]
    pub fn fsp1_frc(&self) -> Fsp1FrcR {
        Fsp1FrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate a BIST operation. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn bist_go(&self) -> BistGoR {
        BistGoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
BIST operation status \\[pass/fail\\]. Bit \\[0\\]
indicates data check status and bit \\[1\\]
indicates address check status. Value of 1 is a passing result. READ-ONLY"]
    #[inline(always)]
    pub fn bist_result(&self) -> BistResultR {
        BistResultR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Sets the number of address bits to check during BIST operation."]
    #[inline(always)]
    pub fn addr_space(&self) -> AddrSpaceR {
        AddrSpaceR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Identifies which of the controller's frequency copy is associated with FSP1."]
    #[inline(always)]
    #[must_use]
    pub fn fsp1_frc(&mut self) -> Fsp1FrcW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec> {
        Fsp1FrcW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate a BIST operation. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn bist_go(&mut self) -> BistGoW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec> {
        BistGoW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
BIST operation status \\[pass/fail\\]. Bit \\[0\\]
indicates data check status and bit \\[1\\]
indicates address check status. Value of 1 is a passing result. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn bist_result(&mut self) -> BistResultW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec> {
        BistResultW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Sets the number of address bits to check during BIST operation."]
    #[inline(always)]
    #[must_use]
    pub fn addr_space(&mut self) -> AddrSpaceW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec> {
        AddrSpaceW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_280\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_280::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_280::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_280::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_280::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_280 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl280Spec {
    const RESET_VALUE: u32 = 0;
}
