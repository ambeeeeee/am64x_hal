#[doc = "Register `CFG_I2C_EOI` reader"]
pub type R = crate::R<CfgI2cEoiSpec>;
#[doc = "Register `CFG_I2C_EOI` writer"]
pub type W = crate::W<CfgI2cEoiSpec>;
#[doc = "Field `LINE_NUMBER` reader - 0:0\\]
Software End Of Interrupt \\[EOI\\]
control Write number of interrupt output"]
pub type LineNumberR = crate::BitReader;
#[doc = "Field `LINE_NUMBER` writer - 0:0\\]
Software End Of Interrupt \\[EOI\\]
control Write number of interrupt output"]
pub type LineNumberW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software End Of Interrupt \\[EOI\\]
control Write number of interrupt output"]
    #[inline(always)]
    pub fn line_number(&self) -> LineNumberR {
        LineNumberR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software End Of Interrupt \\[EOI\\]
control Write number of interrupt output"]
    #[inline(always)]
    #[must_use]
    pub fn line_number(&mut self) -> LineNumberW<CfgI2cEoiSpec> {
        LineNumberW::new(self, 0)
    }
}
#[doc = "End Of Interrupt number specification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cEoiSpec;
impl crate::RegisterSpec for CfgI2cEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_eoi::R`](R) reader structure"]
impl crate::Readable for CfgI2cEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_eoi::W`](W) writer structure"]
impl crate::Writable for CfgI2cEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_EOI to value 0"]
impl crate::Resettable for CfgI2cEoiSpec {
    const RESET_VALUE: u32 = 0;
}
