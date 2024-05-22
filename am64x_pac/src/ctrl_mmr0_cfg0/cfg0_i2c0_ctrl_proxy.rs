#[doc = "Register `CFG0_I2C0_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0I2c0CtrlProxySpec>;
#[doc = "Register `CFG0_I2C0_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0I2c0CtrlProxySpec>;
#[doc = "Field `I2C0_CTRL_HS_MCS_EN_PROXY` reader - 0:0\\]
HS Mode controller current source activate."]
pub type I2c0CtrlHsMcsEnProxyR = crate::BitReader;
#[doc = "Field `I2C0_CTRL_HS_MCS_EN_PROXY` writer - 0:0\\]
HS Mode controller current source activate."]
pub type I2c0CtrlHsMcsEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
HS Mode controller current source activate."]
    #[inline(always)]
    pub fn i2c0_ctrl_hs_mcs_en_proxy(&self) -> I2c0CtrlHsMcsEnProxyR {
        I2c0CtrlHsMcsEnProxyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
HS Mode controller current source activate."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_ctrl_hs_mcs_en_proxy(&mut self) -> I2c0CtrlHsMcsEnProxyW<Cfg0I2c0CtrlProxySpec> {
        I2c0CtrlHsMcsEnProxyW::new(self, 0)
    }
}
#[doc = "CFG0_I2C0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_i2c0_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_i2c0_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0I2c0CtrlProxySpec;
impl crate::RegisterSpec for Cfg0I2c0CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_i2c0_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0I2c0CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_i2c0_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0I2c0CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_I2C0_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0I2c0CtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
