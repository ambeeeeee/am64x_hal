#[doc = "Register `CFG_ERR_VAL` reader"]
pub type R = crate::R<CfgErrValSpec>;
#[doc = "Register `CFG_ERR_VAL` writer"]
pub type W = crate::W<CfgErrValSpec>;
#[doc = "Field `VAL` reader - 0:0\\]
Valid Indicator"]
pub type ValR = crate::BitReader;
#[doc = "Field `VAL` writer - 0:0\\]
Valid Indicator"]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYP` reader - 1:1\\]
Type Indicator"]
pub type TypR = crate::BitReader;
#[doc = "Field `TYP` writer - 1:1\\]
Type Indicator"]
pub type TypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - 2:2\\]
Direction Indicator"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - 2:2\\]
Direction Indicator"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OID` reader - 11:8\\]
Order ID Indicator"]
pub type OidR = crate::FieldReader;
#[doc = "Field `OID` writer - 11:8\\]
Order ID Indicator"]
pub type OidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RID` reader - 27:16\\]
Route ID Indicator"]
pub type RidR = crate::FieldReader<u16>;
#[doc = "Field `RID` writer - 27:16\\]
Route ID Indicator"]
pub type RidW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Valid Indicator"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Type Indicator"]
    #[inline(always)]
    pub fn typ(&self) -> TypR {
        TypR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Direction Indicator"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Order ID Indicator"]
    #[inline(always)]
    pub fn oid(&self) -> OidR {
        OidR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Route ID Indicator"]
    #[inline(always)]
    pub fn rid(&self) -> RidR {
        RidR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Valid Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CfgErrValSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Type Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn typ(&mut self) -> TypW<CfgErrValSpec> {
        TypW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Direction Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<CfgErrValSpec> {
        DirW::new(self, 2)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Order ID Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn oid(&mut self) -> OidW<CfgErrValSpec> {
        OidW::new(self, 8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Route ID Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn rid(&mut self) -> RidW<CfgErrValSpec> {
        RidW::new(self, 16)
    }
}
#[doc = "This register contains information about transaction that caused the interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrValSpec;
impl crate::RegisterSpec for CfgErrValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err_val::R`](R) reader structure"]
impl crate::Readable for CfgErrValSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err_val::W`](W) writer structure"]
impl crate::Writable for CfgErrValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR_VAL to value 0"]
impl crate::Resettable for CfgErrValSpec {
    const RESET_VALUE: u32 = 0;
}
