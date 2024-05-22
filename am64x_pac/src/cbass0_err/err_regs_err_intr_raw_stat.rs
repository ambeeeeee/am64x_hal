#[doc = "Register `ERR_REGS_err_intr_raw_stat` reader"]
pub type R = crate::R<ErrRegsErrIntrRawStatSpec>;
#[doc = "Register `ERR_REGS_err_intr_raw_stat` writer"]
pub type W = crate::W<ErrRegsErrIntrRawStatSpec>;
#[doc = "Field `INTR` reader - 0:0\\]
Level Interrupt status"]
pub type IntrR = crate::BitReader;
#[doc = "Field `INTR` writer - 0:0\\]
Level Interrupt status"]
pub type IntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Level Interrupt status"]
    #[inline(always)]
    pub fn intr(&self) -> IntrR {
        IntrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Level Interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn intr(&mut self) -> IntrW<ErrRegsErrIntrRawStatSpec> {
        IntrW::new(self, 0)
    }
}
#[doc = "Global Interrupt Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_intr_raw_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_intr_raw_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrRegsErrIntrRawStatSpec;
impl crate::RegisterSpec for ErrRegsErrIntrRawStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_regs_err_intr_raw_stat::R`](R) reader structure"]
impl crate::Readable for ErrRegsErrIntrRawStatSpec {}
#[doc = "`write(|w| ..)` method takes [`err_regs_err_intr_raw_stat::W`](W) writer structure"]
impl crate::Writable for ErrRegsErrIntrRawStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_REGS_err_intr_raw_stat to value 0"]
impl crate::Resettable for ErrRegsErrIntrRawStatSpec {
    const RESET_VALUE: u32 = 0;
}
