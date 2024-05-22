#[doc = "Register `ERR_REGS_err_eoi` reader"]
pub type R = crate::R<ErrRegsErrEoiSpec>;
#[doc = "Register `ERR_REGS_err_eoi` writer"]
pub type W = crate::W<ErrRegsErrEoiSpec>;
#[doc = "Field `EOI_WR` reader - 15:0\\]
EOI Register"]
pub type EoiWrR = crate::FieldReader<u16>;
#[doc = "Field `EOI_WR` writer - 15:0\\]
EOI Register"]
pub type EoiWrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
EOI Register"]
    #[inline(always)]
    pub fn eoi_wr(&self) -> EoiWrR {
        EoiWrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
EOI Register"]
    #[inline(always)]
    #[must_use]
    pub fn eoi_wr(&mut self) -> EoiWrW<ErrRegsErrEoiSpec> {
        EoiWrW::new(self, 0)
    }
}
#[doc = "EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrRegsErrEoiSpec;
impl crate::RegisterSpec for ErrRegsErrEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_regs_err_eoi::R`](R) reader structure"]
impl crate::Readable for ErrRegsErrEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`err_regs_err_eoi::W`](W) writer structure"]
impl crate::Writable for ErrRegsErrEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_REGS_err_eoi to value 0"]
impl crate::Resettable for ErrRegsErrEoiSpec {
    const RESET_VALUE: u32 = 0;
}
