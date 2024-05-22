#[doc = "Register `BCDMA_GCFG_CAP1` reader"]
pub type R = crate::R<BcdmaGcfgCap1Spec>;
#[doc = "Register `BCDMA_GCFG_CAP1` writer"]
pub type W = crate::W<BcdmaGcfgCap1Spec>;
#[doc = "Field `AMODE` reader - 0:0\\]
The maximum AMODE that is supported. If AMODE is supported then DIR field must be supported for that AMODE."]
pub type AmodeR = crate::BitReader;
#[doc = "Field `AMODE` writer - 0:0\\]
The maximum AMODE that is supported. If AMODE is supported then DIR field must be supported for that AMODE."]
pub type AmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELTYPE` reader - 1:1\\]
Maximum element type value that is supported."]
pub type EltypeR = crate::BitReader;
#[doc = "Field `ELTYPE` writer - 1:1\\]
Maximum element type value that is supported."]
pub type EltypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFMT` reader - 2:2\\]
Maximum data reformatting function that is supported"]
pub type DfmtR = crate::BitReader;
#[doc = "Field `DFMT` writer - 2:2\\]
Maximum data reformatting function that is supported"]
pub type DfmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTR` reader - 3:3\\]
Maximum second TR function that is supported"]
pub type SectrR = crate::BitReader;
#[doc = "Field `SECTR` writer - 3:3\\]
Maximum second TR function that is supported"]
pub type SectrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The maximum AMODE that is supported. If AMODE is supported then DIR field must be supported for that AMODE."]
    #[inline(always)]
    pub fn amode(&self) -> AmodeR {
        AmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Maximum element type value that is supported."]
    #[inline(always)]
    pub fn eltype(&self) -> EltypeR {
        EltypeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Maximum data reformatting function that is supported"]
    #[inline(always)]
    pub fn dfmt(&self) -> DfmtR {
        DfmtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Maximum second TR function that is supported"]
    #[inline(always)]
    pub fn sectr(&self) -> SectrR {
        SectrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The maximum AMODE that is supported. If AMODE is supported then DIR field must be supported for that AMODE."]
    #[inline(always)]
    #[must_use]
    pub fn amode(&mut self) -> AmodeW<BcdmaGcfgCap1Spec> {
        AmodeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Maximum element type value that is supported."]
    #[inline(always)]
    #[must_use]
    pub fn eltype(&mut self) -> EltypeW<BcdmaGcfgCap1Spec> {
        EltypeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Maximum data reformatting function that is supported"]
    #[inline(always)]
    #[must_use]
    pub fn dfmt(&mut self) -> DfmtW<BcdmaGcfgCap1Spec> {
        DfmtW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Maximum second TR function that is supported"]
    #[inline(always)]
    #[must_use]
    pub fn sectr(&mut self) -> SectrW<BcdmaGcfgCap1Spec> {
        SectrW::new(self, 3)
    }
}
#[doc = "The Capabilities Register 1 specifies which standard features this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaGcfgCap1Spec;
impl crate::RegisterSpec for BcdmaGcfgCap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_gcfg_cap1::R`](R) reader structure"]
impl crate::Readable for BcdmaGcfgCap1Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_gcfg_cap1::W`](W) writer structure"]
impl crate::Writable for BcdmaGcfgCap1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_GCFG_CAP1 to value 0"]
impl crate::Resettable for BcdmaGcfgCap1Spec {
    const RESET_VALUE: u32 = 0;
}
