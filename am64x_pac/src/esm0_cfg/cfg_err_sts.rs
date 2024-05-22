#[doc = "Register `CFG_ERR_STS` reader"]
pub type R = crate::R<CfgErrStsSpec>;
#[doc = "Register `CFG_ERR_STS` writer"]
pub type W = crate::W<CfgErrStsSpec>;
#[doc = "Field `MSK` reader - 5:0\\]
This is the masked status/clear for config errors"]
pub type MskR = crate::FieldReader;
#[doc = "Field `MSK` writer - 5:0\\]
This is the masked status/clear for config errors"]
pub type MskW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
This is the masked status/clear for config errors"]
    #[inline(always)]
    pub fn msk(&self) -> MskR {
        MskR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
This is the masked status/clear for config errors"]
    #[inline(always)]
    #[must_use]
    pub fn msk(&mut self) -> MskW<CfgErrStsSpec> {
        MskW::new(self, 0)
    }
}
#[doc = "Config Error Enable and Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrStsSpec;
impl crate::RegisterSpec for CfgErrStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err_sts::R`](R) reader structure"]
impl crate::Readable for CfgErrStsSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err_sts::W`](W) writer structure"]
impl crate::Writable for CfgErrStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR_STS to value 0"]
impl crate::Resettable for CfgErrStsSpec {
    const RESET_VALUE: u32 = 0;
}
