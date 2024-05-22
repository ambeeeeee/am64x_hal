#[doc = "Register `REGS_sec_enable_set_reg0` reader"]
pub type R = crate::R<RegsSecEnableSetReg0Spec>;
#[doc = "Register `REGS_sec_enable_set_reg0` writer"]
pub type W = crate::W<RegsSecEnableSetReg0Spec>;
#[doc = "Field `SRAM_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for sram_pend"]
pub type SramEnableSetR = crate::BitReader;
#[doc = "Field `SRAM_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for sram_pend"]
pub type SramEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for sram_pend"]
    #[inline(always)]
    pub fn sram_enable_set(&self) -> SramEnableSetR {
        SramEnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for sram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn sram_enable_set(&mut self) -> SramEnableSetW<RegsSecEnableSetReg0Spec> {
        SramEnableSetW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_sec_enable_set_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_sec_enable_set_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsSecEnableSetReg0Spec;
impl crate::RegisterSpec for RegsSecEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_sec_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for RegsSecEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`regs_sec_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for RegsSecEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_sec_enable_set_reg0 to value 0"]
impl crate::Resettable for RegsSecEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
