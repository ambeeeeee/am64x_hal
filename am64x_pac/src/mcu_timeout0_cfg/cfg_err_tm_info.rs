#[doc = "Register `CFG_ERR_TM_INFO` reader"]
pub type R = crate::R<CfgErrTmInfoSpec>;
#[doc = "Register `CFG_ERR_TM_INFO` writer"]
pub type W = crate::W<CfgErrTmInfoSpec>;
#[doc = "Field `CNT` reader - 1:0\\]
Timeout Interrupt Count"]
pub type CntR = crate::FieldReader;
#[doc = "Field `CNT` writer - 1:0\\]
Timeout Interrupt Count"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Timeout Interrupt Count"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Timeout Interrupt Count"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<CfgErrTmInfoSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "This register contains information about timeout interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_tm_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_tm_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrTmInfoSpec;
impl crate::RegisterSpec for CfgErrTmInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err_tm_info::R`](R) reader structure"]
impl crate::Readable for CfgErrTmInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err_tm_info::W`](W) writer structure"]
impl crate::Writable for CfgErrTmInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR_TM_INFO to value 0"]
impl crate::Resettable for CfgErrTmInfoSpec {
    const RESET_VALUE: u32 = 0;
}
