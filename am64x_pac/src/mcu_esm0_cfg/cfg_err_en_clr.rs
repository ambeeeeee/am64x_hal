#[doc = "Register `CFG_ERR_EN_CLR` reader"]
pub type R = crate::R<CfgErrEnClrSpec>;
#[doc = "Register `CFG_ERR_EN_CLR` writer"]
pub type W = crate::W<CfgErrEnClrSpec>;
#[doc = "Field `MSK` reader - 2:0\\]
This is the mask enable clear for config errors"]
pub type MskR = crate::FieldReader;
#[doc = "Field `MSK` writer - 2:0\\]
This is the mask enable clear for config errors"]
pub type MskW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This is the mask enable clear for config errors"]
    #[inline(always)]
    pub fn msk(&self) -> MskR {
        MskR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This is the mask enable clear for config errors"]
    #[inline(always)]
    #[must_use]
    pub fn msk(&mut self) -> MskW<CfgErrEnClrSpec> {
        MskW::new(self, 0)
    }
}
#[doc = "Config Error Interrupt Enabled Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_en_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_en_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrEnClrSpec;
impl crate::RegisterSpec for CfgErrEnClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err_en_clr::R`](R) reader structure"]
impl crate::Readable for CfgErrEnClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err_en_clr::W`](W) writer structure"]
impl crate::Writable for CfgErrEnClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR_EN_CLR to value 0"]
impl crate::Resettable for CfgErrEnClrSpec {
    const RESET_VALUE: u32 = 0;
}
