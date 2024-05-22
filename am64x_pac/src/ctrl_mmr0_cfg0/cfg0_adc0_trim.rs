#[doc = "Register `CFG0_ADC0_TRIM` reader"]
pub type R = crate::R<Cfg0Adc0TrimSpec>;
#[doc = "Register `CFG0_ADC0_TRIM` writer"]
pub type W = crate::W<Cfg0Adc0TrimSpec>;
#[doc = "Field `ADC0_TRIM_ENABLE_CAL` reader - 4:0\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimEnableCalR = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_ENABLE_CAL` writer - 4:0\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimEnableCalW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC0_TRIM_ENABLE_CALB` reader - 9:5\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimEnableCalbR = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_ENABLE_CALB` writer - 9:5\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimEnableCalbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC0_TRIM_TRIM1` reader - 13:10\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim1R = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM1` writer - 13:10\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADC0_TRIM_TRIM2` reader - 17:14\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim2R = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM2` writer - 17:14\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADC0_TRIM_TRIM3` reader - 20:18\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim3R = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM3` writer - 20:18\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADC0_TRIM_TRIM4` reader - 23:21\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim4R = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM4` writer - 23:21\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADC0_TRIM_TRIM5` reader - 26:24\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim5R = crate::FieldReader;
#[doc = "Field `ADC0_TRIM_TRIM5` writer - 26:24\\]
Trims Nonlinearities from ADC"]
pub type Adc0TrimTrim5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_enable_cal(&self) -> Adc0TrimEnableCalR {
        Adc0TrimEnableCalR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_enable_calb(&self) -> Adc0TrimEnableCalbR {
        Adc0TrimEnableCalbR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim1(&self) -> Adc0TrimTrim1R {
        Adc0TrimTrim1R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim2(&self) -> Adc0TrimTrim2R {
        Adc0TrimTrim2R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim3(&self) -> Adc0TrimTrim3R {
        Adc0TrimTrim3R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim4(&self) -> Adc0TrimTrim4R {
        Adc0TrimTrim4R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    pub fn adc0_trim_trim5(&self) -> Adc0TrimTrim5R {
        Adc0TrimTrim5R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_enable_cal(&mut self) -> Adc0TrimEnableCalW<Cfg0Adc0TrimSpec> {
        Adc0TrimEnableCalW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_enable_calb(&mut self) -> Adc0TrimEnableCalbW<Cfg0Adc0TrimSpec> {
        Adc0TrimEnableCalbW::new(self, 5)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim1(&mut self) -> Adc0TrimTrim1W<Cfg0Adc0TrimSpec> {
        Adc0TrimTrim1W::new(self, 10)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim2(&mut self) -> Adc0TrimTrim2W<Cfg0Adc0TrimSpec> {
        Adc0TrimTrim2W::new(self, 14)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim3(&mut self) -> Adc0TrimTrim3W<Cfg0Adc0TrimSpec> {
        Adc0TrimTrim3W::new(self, 18)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim4(&mut self) -> Adc0TrimTrim4W<Cfg0Adc0TrimSpec> {
        Adc0TrimTrim4W::new(self, 21)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Trims Nonlinearities from ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_trim_trim5(&mut self) -> Adc0TrimTrim5W<Cfg0Adc0TrimSpec> {
        Adc0TrimTrim5W::new(self, 24)
    }
}
#[doc = "CFG0_ADC0_TRIM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Adc0TrimSpec;
impl crate::RegisterSpec for Cfg0Adc0TrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_adc0_trim::R`](R) reader structure"]
impl crate::Readable for Cfg0Adc0TrimSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_adc0_trim::W`](W) writer structure"]
impl crate::Writable for Cfg0Adc0TrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ADC0_TRIM to value 0"]
impl crate::Resettable for Cfg0Adc0TrimSpec {
    const RESET_VALUE: u32 = 0;
}
