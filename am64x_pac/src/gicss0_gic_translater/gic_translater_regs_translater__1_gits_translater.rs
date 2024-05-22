#[doc = "Register `GIC_TRANSLATER_REGS_TRANSLATER__1_GITS_TRANSLATER` reader"]
pub type R = crate::R<GicTranslaterRegsTranslater_1GitsTranslaterSpec>;
#[doc = "Register `GIC_TRANSLATER_REGS_TRANSLATER__1_GITS_TRANSLATER` writer"]
pub type W = crate::W<GicTranslaterRegsTranslater_1GitsTranslaterSpec>;
#[doc = "Field `TRANSLATER__1_GITS_TRANSLATER__0_32` reader - 31:0\\]
Input ID"]
pub type Translater_1GitsTranslater_0_32R = crate::FieldReader<u32>;
#[doc = "Field `TRANSLATER__1_GITS_TRANSLATER__0_32` writer - 31:0\\]
Input ID"]
pub type Translater_1GitsTranslater_0_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Input ID"]
    #[inline(always)]
    pub fn translater__1_gits_translater__0_32(&self) -> Translater_1GitsTranslater_0_32R {
        Translater_1GitsTranslater_0_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Input ID"]
    #[inline(always)]
    #[must_use]
    pub fn translater__1_gits_translater__0_32(
        &mut self,
    ) -> Translater_1GitsTranslater_0_32W<GicTranslaterRegsTranslater_1GitsTranslaterSpec> {
        Translater_1GitsTranslater_0_32W::new(self, 0)
    }
}
#[doc = "GITS_TRANSLATER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_translater_regs_translater__1_gits_translater::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_translater_regs_translater__1_gits_translater::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicTranslaterRegsTranslater_1GitsTranslaterSpec;
impl crate::RegisterSpec for GicTranslaterRegsTranslater_1GitsTranslaterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_translater_regs_translater__1_gits_translater::R`](R) reader structure"]
impl crate::Readable for GicTranslaterRegsTranslater_1GitsTranslaterSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_translater_regs_translater__1_gits_translater::W`](W) writer structure"]
impl crate::Writable for GicTranslaterRegsTranslater_1GitsTranslaterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_TRANSLATER_REGS_TRANSLATER__1_GITS_TRANSLATER to value 0"]
impl crate::Resettable for GicTranslaterRegsTranslater_1GitsTranslaterSpec {
    const RESET_VALUE: u32 = 0;
}
