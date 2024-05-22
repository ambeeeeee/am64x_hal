#[doc = "Register `CFG_ERR_MSK_CLR` reader"]
pub type R = crate::R<CfgErrMskClrSpec>;
#[doc = "Register `CFG_ERR_MSK_CLR` writer"]
pub type W = crate::W<CfgErrMskClrSpec>;
#[doc = "Field `TIMEOUT` reader - 0:0\\]
Raw Timeout Error Interrupt Mask Clear"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - 0:0\\]
Raw Timeout Error Interrupt Mask Clear"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNEXP` reader - 1:1\\]
Raw Unexpected Error Interrupt Mask Clear"]
pub type UnexpR = crate::BitReader;
#[doc = "Field `UNEXP` writer - 1:1\\]
Raw Unexpected Error Interrupt Mask Clear"]
pub type UnexpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD` reader - 2:2\\]
Raw Command Error Interrupt Mask Clear"]
pub type CmdR = crate::BitReader;
#[doc = "Field `CMD` writer - 2:2\\]
Raw Command Error Interrupt Mask Clear"]
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Raw Timeout Error Interrupt Mask Clear"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw Unexpected Error Interrupt Mask Clear"]
    #[inline(always)]
    pub fn unexp(&self) -> UnexpR {
        UnexpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw Command Error Interrupt Mask Clear"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Raw Timeout Error Interrupt Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<CfgErrMskClrSpec> {
        TimeoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw Unexpected Error Interrupt Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn unexp(&mut self) -> UnexpW<CfgErrMskClrSpec> {
        UnexpW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw Command Error Interrupt Mask Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CfgErrMskClrSpec> {
        CmdW::new(self, 2)
    }
}
#[doc = "This register contains interrupt mask clear bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_msk_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_msk_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrMskClrSpec;
impl crate::RegisterSpec for CfgErrMskClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err_msk_clr::R`](R) reader structure"]
impl crate::Readable for CfgErrMskClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err_msk_clr::W`](W) writer structure"]
impl crate::Writable for CfgErrMskClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR_MSK_CLR to value 0x07"]
impl crate::Resettable for CfgErrMskClrSpec {
    const RESET_VALUE: u32 = 0x07;
}
