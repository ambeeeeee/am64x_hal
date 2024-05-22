#[doc = "Register `CFG_GPMC_BCH_RESULT_6` reader"]
pub type R = crate::R<CfgGpmcBchResult6Spec>;
#[doc = "Register `CFG_GPMC_BCH_RESULT_6` writer"]
pub type W = crate::W<CfgGpmcBchResult6Spec>;
#[doc = "Field `BCH_RESULT_6` reader - 15:0\\]
BCH ECC result, bits 192 to 207"]
pub type BchResult6R = crate::FieldReader<u16>;
#[doc = "Field `BCH_RESULT_6` writer - 15:0\\]
BCH ECC result, bits 192 to 207"]
pub type BchResult6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
BCH ECC result, bits 192 to 207"]
    #[inline(always)]
    pub fn bch_result_6(&self) -> BchResult6R {
        BchResult6R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
BCH ECC result, bits 192 to 207"]
    #[inline(always)]
    #[must_use]
    pub fn bch_result_6(&mut self) -> BchResult6W<CfgGpmcBchResult6Spec> {
        BchResult6W::new(self, 0)
    }
}
#[doc = "BCH ECC result, bits 192 to 207\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_result_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_result_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcBchResult6Spec;
impl crate::RegisterSpec for CfgGpmcBchResult6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_bch_result_6::R`](R) reader structure"]
impl crate::Readable for CfgGpmcBchResult6Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_bch_result_6::W`](W) writer structure"]
impl crate::Writable for CfgGpmcBchResult6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_BCH_RESULT_6 to value 0"]
impl crate::Resettable for CfgGpmcBchResult6Spec {
    const RESET_VALUE: u32 = 0;
}
