#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_204` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl204Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_204` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl204Spec>;
#[doc = "Field `MR4_DLL_RST` reader - 0:0\\]
Asserted if DRAM DLL Reset bit resides in MR4."]
pub type Mr4DllRstR = crate::BitReader;
#[doc = "Field `MR4_DLL_RST` writer - 0:0\\]
Asserted if DRAM DLL Reset bit resides in MR4."]
pub type Mr4DllRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR0_DATA_F0_0` reader - 24:8\\]
Data to program into memory mode register 0. FC=0"]
pub type Mr0DataF0_0R = crate::FieldReader<u32>;
#[doc = "Field `MR0_DATA_F0_0` writer - 24:8\\]
Data to program into memory mode register 0. FC=0"]
pub type Mr0DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Asserted if DRAM DLL Reset bit resides in MR4."]
    #[inline(always)]
    pub fn mr4_dll_rst(&self) -> Mr4DllRstR {
        Mr4DllRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:24 - 24:8\\]
Data to program into memory mode register 0. FC=0"]
    #[inline(always)]
    pub fn mr0_data_f0_0(&self) -> Mr0DataF0_0R {
        Mr0DataF0_0R::new((self.bits >> 8) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Asserted if DRAM DLL Reset bit resides in MR4."]
    #[inline(always)]
    #[must_use]
    pub fn mr4_dll_rst(&mut self) -> Mr4DllRstW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl204Spec> {
        Mr4DllRstW::new(self, 0)
    }
    #[doc = "Bits 8:24 - 24:8\\]
Data to program into memory mode register 0. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0_data_f0_0(&mut self) -> Mr0DataF0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl204Spec> {
        Mr0DataF0_0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_204\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_204::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_204::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl204Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_204::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl204Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_204::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl204Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_204 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl204Spec {
    const RESET_VALUE: u32 = 0;
}
