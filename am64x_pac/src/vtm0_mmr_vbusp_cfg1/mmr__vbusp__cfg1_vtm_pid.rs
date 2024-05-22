#[doc = "Register `MMR__VBUSP__CFG1_VTM_PID` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1VtmPidSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_VTM_PID` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1VtmPidSpec>;
#[doc = "Field `Y_MINOR` reader - 5:0\\]
Minor revision number - actual value determined by RTL"]
pub type YMinorR = crate::FieldReader;
#[doc = "Field `Y_MINOR` writer - 5:0\\]
Minor revision number - actual value determined by RTL"]
pub type YMinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Custom revision number - actual value determined by RTL"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Custom revision number - actual value determined by RTL"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `X_MAJOR` reader - 10:8\\]
Major revision number - actual value determined by RTL"]
pub type XMajorR = crate::FieldReader;
#[doc = "Field `X_MAJOR` writer - 10:8\\]
Major revision number - actual value determined by RTL"]
pub type XMajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R_RTL` reader - 15:11\\]
RTL revision number - actual value determined by RTL"]
pub type RRtlR = crate::FieldReader;
#[doc = "Field `R_RTL` writer - 15:11\\]
RTL revision number - actual value determined by RTL"]
pub type RRtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNC` reader - 27:16\\]
Module functional identifier - CTRL MMR"]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 27:16\\]
Module functional identifier - CTRL MMR"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BU` reader - 29:28\\]
Business unit - Processors"]
pub type BuR = crate::FieldReader;
#[doc = "Field `BU` writer - 29:28\\]
Business unit - Processors"]
pub type BuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
PID follows new scheme"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
PID follows new scheme"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number - actual value determined by RTL"]
    #[inline(always)]
    pub fn y_minor(&self) -> YMinorR {
        YMinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision number - actual value determined by RTL"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision number - actual value determined by RTL"]
    #[inline(always)]
    pub fn x_major(&self) -> XMajorR {
        XMajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision number - actual value determined by RTL"]
    #[inline(always)]
    pub fn r_rtl(&self) -> RRtlR {
        RRtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module functional identifier - CTRL MMR"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Business unit - Processors"]
    #[inline(always)]
    pub fn bu(&self) -> BuR {
        BuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID follows new scheme"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number - actual value determined by RTL"]
    #[inline(always)]
    #[must_use]
    pub fn y_minor(&mut self) -> YMinorW<Mmr_Vbusp_Cfg1VtmPidSpec> {
        YMinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision number - actual value determined by RTL"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<Mmr_Vbusp_Cfg1VtmPidSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision number - actual value determined by RTL"]
    #[inline(always)]
    #[must_use]
    pub fn x_major(&mut self) -> XMajorW<Mmr_Vbusp_Cfg1VtmPidSpec> {
        XMajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision number - actual value determined by RTL"]
    #[inline(always)]
    #[must_use]
    pub fn r_rtl(&mut self) -> RRtlW<Mmr_Vbusp_Cfg1VtmPidSpec> {
        RRtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module functional identifier - CTRL MMR"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<Mmr_Vbusp_Cfg1VtmPidSpec> {
        FuncW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Business unit - Processors"]
    #[inline(always)]
    #[must_use]
    pub fn bu(&mut self) -> BuW<Mmr_Vbusp_Cfg1VtmPidSpec> {
        BuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID follows new scheme"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<Mmr_Vbusp_Cfg1VtmPidSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "VTM Peripheral Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_vtm_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_vtm_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1VtmPidSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1VtmPidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_vtm_pid::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1VtmPidSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_vtm_pid::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1VtmPidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_VTM_PID to value 0x7563_a901"]
impl crate::Resettable for Mmr_Vbusp_Cfg1VtmPidSpec {
    const RESET_VALUE: u32 = 0x7563_a901;
}
