#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_361` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_361` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec>;
#[doc = "Field `OUT_OF_RANGE_ADDR_1` reader - 0:0\\]
Address of command that caused an out-of-range interrupt. READ-ONLY"]
pub type OutOfRangeAddr1R = crate::BitReader;
#[doc = "Field `OUT_OF_RANGE_ADDR_1` writer - 0:0\\]
Address of command that caused an out-of-range interrupt. READ-ONLY"]
pub type OutOfRangeAddr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_OF_RANGE_LENGTH` reader - 18:8\\]
Length of command that caused an out-of-range interrupt. READ-ONLY"]
pub type OutOfRangeLengthR = crate::FieldReader<u16>;
#[doc = "Field `OUT_OF_RANGE_LENGTH` writer - 18:8\\]
Length of command that caused an out-of-range interrupt. READ-ONLY"]
pub type OutOfRangeLengthW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `OUT_OF_RANGE_TYPE` reader - 30:24\\]
Type of command that caused an out-of-range interrupt. READ-ONLY"]
pub type OutOfRangeTypeR = crate::FieldReader;
#[doc = "Field `OUT_OF_RANGE_TYPE` writer - 30:24\\]
Type of command that caused an out-of-range interrupt. READ-ONLY"]
pub type OutOfRangeTypeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Address of command that caused an out-of-range interrupt. READ-ONLY"]
    #[inline(always)]
    pub fn out_of_range_addr_1(&self) -> OutOfRangeAddr1R {
        OutOfRangeAddr1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Length of command that caused an out-of-range interrupt. READ-ONLY"]
    #[inline(always)]
    pub fn out_of_range_length(&self) -> OutOfRangeLengthR {
        OutOfRangeLengthR::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Type of command that caused an out-of-range interrupt. READ-ONLY"]
    #[inline(always)]
    pub fn out_of_range_type(&self) -> OutOfRangeTypeR {
        OutOfRangeTypeR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Address of command that caused an out-of-range interrupt. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn out_of_range_addr_1(
        &mut self,
    ) -> OutOfRangeAddr1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec> {
        OutOfRangeAddr1W::new(self, 0)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Length of command that caused an out-of-range interrupt. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn out_of_range_length(
        &mut self,
    ) -> OutOfRangeLengthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec> {
        OutOfRangeLengthW::new(self, 8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Type of command that caused an out-of-range interrupt. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn out_of_range_type(
        &mut self,
    ) -> OutOfRangeTypeW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec> {
        OutOfRangeTypeW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_361\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_361::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_361::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_361::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_361::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_361 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl361Spec {
    const RESET_VALUE: u32 = 0;
}
