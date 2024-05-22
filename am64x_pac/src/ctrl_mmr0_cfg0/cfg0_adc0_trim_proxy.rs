#[doc = "Register `CFG0_ADC0_TRIM_PROXY` reader"]
pub type R = crate::R<Cfg0Adc0TrimProxySpec>;
#[doc = "Register `CFG0_ADC0_TRIM_PROXY` writer"]
pub type W = crate::W<Cfg0Adc0TrimProxySpec>;
#[doc = "Field `ADC0_TRIM_ENABLE_CAL_PROXY` reader - 4:0\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimEnableCalProxyR = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_ENABLE_CAL_PROXY` writer - 4:0\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimEnableCalProxyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC0_TRIM_ENABLE_CALB_PROXY` reader - 9:5\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimEnableCalbProxyR = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_ENABLE_CALB_PROXY` writer - 9:5\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimEnableCalbProxyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC0_TRIM_TRIM1_PROXY` reader - 13:10\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim1ProxyR = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM1_PROXY` writer - 13:10\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim1ProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADC0_TRIM_TRIM2_PROXY` reader - 17:14\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim2ProxyR = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM2_PROXY` writer - 17:14\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim2ProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADC0_TRIM_TRIM3_PROXY` reader - 20:18\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim3ProxyR = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM3_PROXY` writer - 20:18\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim3ProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADC0_TRIM_TRIM4_PROXY` reader - 23:21\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim4ProxyR = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM4_PROXY` writer - 23:21\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim4ProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADC0_TRIM_TRIM5_PROXY` reader - 26:24\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim5ProxyR = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM5_PROXY` writer - 26:24\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim5ProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_enable_cal_proxy(&self) -> Adc0TrimEnableCalProxyR {
        Adc0TrimEnableCalProxyR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_enable_calb_proxy(&self) -> Adc0TrimEnableCalbProxyR {
        Adc0TrimEnableCalbProxyR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim1_proxy(&self) -> Adc0TrimTrim1ProxyR {
        Adc0TrimTrim1ProxyR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim2_proxy(&self) -> Adc0TrimTrim2ProxyR {
        Adc0TrimTrim2ProxyR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim3_proxy(&self) -> Adc0TrimTrim3ProxyR {
        Adc0TrimTrim3ProxyR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim4_proxy(&self) -> Adc0TrimTrim4ProxyR {
        Adc0TrimTrim4ProxyR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim5_proxy(&self) -> Adc0TrimTrim5ProxyR {
        Adc0TrimTrim5ProxyR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_enable_cal_proxy(&mut self) -> Adc0TrimEnableCalProxyW<Cfg0Adc0TrimProxySpec> {
        Adc0TrimEnableCalProxyW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_enable_calb_proxy(
        &mut self,
    ) -> Adc0TrimEnableCalbProxyW<Cfg0Adc0TrimProxySpec> {
        Adc0TrimEnableCalbProxyW::new(self, 5)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim1_proxy(&mut self) -> Adc0TrimTrim1ProxyW<Cfg0Adc0TrimProxySpec> {
        Adc0TrimTrim1ProxyW::new(self, 10)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim2_proxy(&mut self) -> Adc0TrimTrim2ProxyW<Cfg0Adc0TrimProxySpec> {
        Adc0TrimTrim2ProxyW::new(self, 14)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim3_proxy(&mut self) -> Adc0TrimTrim3ProxyW<Cfg0Adc0TrimProxySpec> {
        Adc0TrimTrim3ProxyW::new(self, 18)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim4_proxy(&mut self) -> Adc0TrimTrim4ProxyW<Cfg0Adc0TrimProxySpec> {
        Adc0TrimTrim4ProxyW::new(self, 21)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim5_proxy(&mut self) -> Adc0TrimTrim5ProxyW<Cfg0Adc0TrimProxySpec> {
        Adc0TrimTrim5ProxyW::new(self, 24)
    }
}
#[doc = "CFG0_ADC0_TRIM_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_trim_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_trim_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Adc0TrimProxySpec;
impl crate::RegisterSpec for Cfg0Adc0TrimProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_adc0_trim_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Adc0TrimProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_adc0_trim_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Adc0TrimProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ADC0_TRIM_PROXY to value 0"]
impl crate::Resettable for Cfg0Adc0TrimProxySpec {
    const RESET_VALUE: u32 = 0;
}
