#[doc = "Register `CFG_GPMC_BCH_RESULT_3` reader"]
pub type R = crate::R<CfgGpmcBchResult3Spec>;
#[doc = "Register `CFG_GPMC_BCH_RESULT_3` writer"]
pub type W = crate::W<CfgGpmcBchResult3Spec>;
#[doc = "Field `BCH_RESULT_3` reader - 31:0\\]
BCH ECC result, bits 96 to 127"]
pub type BchResult3R = crate::FieldReader<u32>;
#[doc = "Field `BCH_RESULT_3` writer - 31:0\\]
BCH ECC result, bits 96 to 127"]
pub type BchResult3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
BCH ECC result, bits 96 to 127"]
    #[inline(always)]
    pub fn bch_result_3(&self) -> BchResult3R {
        BchResult3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
BCH ECC result, bits 96 to 127"]
    #[inline(always)]
    #[must_use]
    pub fn bch_result_3(&mut self) -> BchResult3W<CfgGpmcBchResult3Spec> {
        BchResult3W::new(self, 0)
    }
}
#[doc = "BCH ECC result, bits 96 to 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcBchResult3Spec;
impl crate::RegisterSpec for CfgGpmcBchResult3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_bch_result_3::R`](R) reader structure"]
impl crate::Readable for CfgGpmcBchResult3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_bch_result_3::W`](W) writer structure"]
impl crate::Writable for CfgGpmcBchResult3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_BCH_RESULT_3 to value 0"]
impl crate::Resettable for CfgGpmcBchResult3Spec {
    const RESET_VALUE: u32 = 0;
}
