#[doc = "Register `CFG_GPMC_BCH_RESULT_1` reader"]
pub type R = crate::R<CfgGpmcBchResult1Spec>;
#[doc = "Register `CFG_GPMC_BCH_RESULT_1` writer"]
pub type W = crate::W<CfgGpmcBchResult1Spec>;
#[doc = "Field `BCH_RESULT_1` reader - 31:0\\]
BCH ECC result, bits 32 to 63"]
pub type BchResult1R = crate::FieldReader<u32>;
#[doc = "Field `BCH_RESULT_1` writer - 31:0\\]
BCH ECC result, bits 32 to 63"]
pub type BchResult1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
BCH ECC result, bits 32 to 63"]
    #[inline(always)]
    pub fn bch_result_1(&self) -> BchResult1R {
        BchResult1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
BCH ECC result, bits 32 to 63"]
    #[inline(always)]
    #[must_use]
    pub fn bch_result_1(&mut self) -> BchResult1W<CfgGpmcBchResult1Spec> {
        BchResult1W::new(self, 0)
    }
}
#[doc = "BCH ECC result, bits 32 to 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcBchResult1Spec;
impl crate::RegisterSpec for CfgGpmcBchResult1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_bch_result_1::R`](R) reader structure"]
impl crate::Readable for CfgGpmcBchResult1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_bch_result_1::W`](W) writer structure"]
impl crate::Writable for CfgGpmcBchResult1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_BCH_RESULT_1 to value 0"]
impl crate::Resettable for CfgGpmcBchResult1Spec {
    const RESET_VALUE: u32 = 0;
}
