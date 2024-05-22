#[doc = "Register `CFG0_I2C0_CTRL` reader"]
pub type R = crate::R<Cfg0I2c0CtrlSpec>;
#[doc = "Register `CFG0_I2C0_CTRL` writer"]
pub type W = crate::W<Cfg0I2c0CtrlSpec>;
#[doc = "Field `I2C0_CTRL_HS_MCS_EN` reader - 0:0\\]
HS Mode controller current source activate."]
pub type I2c0CtrlHsMcsEnR = crate::BitReader;
#[doc = "Field `I2C0_CTRL_HS_MCS_EN` writer - 0:0\\]
HS Mode controller current source activate."]
pub type I2c0CtrlHsMcsEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
HS Mode controller current source activate."]
    #[inline(always)]
    pub fn i2c0_ctrl_hs_mcs_en(&self) -> I2c0CtrlHsMcsEnR {
        I2c0CtrlHsMcsEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
HS Mode controller current source activate."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_ctrl_hs_mcs_en(&mut self) -> I2c0CtrlHsMcsEnW<Cfg0I2c0CtrlSpec> {
        I2c0CtrlHsMcsEnW::new(self, 0)
    }
}
#[doc = "CFG0_I2C0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_i2c0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_i2c0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0I2c0CtrlSpec;
impl crate::RegisterSpec for Cfg0I2c0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_i2c0_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0I2c0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_i2c0_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0I2c0CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_I2C0_CTRL to value 0"]
impl crate::Resettable for Cfg0I2c0CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
