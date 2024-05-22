#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIGATE` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtigateSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIGATE` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtigateSpec>;
#[doc = "Field `CTIGATEEN0` reader - 0:0\\]
Enable CTICHOUT0. Set to 0 to disable channel propagation."]
pub type Ctigateen0R = crate::BitReader;
#[doc = "Field `CTIGATEEN0` writer - 0:0\\]
Enable CTICHOUT0. Set to 0 to disable channel propagation."]
pub type Ctigateen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIGATEEN1` reader - 1:1\\]
Enable CTICHOUT1. Set to 0 to disable channel propagation."]
pub type Ctigateen1R = crate::BitReader;
#[doc = "Field `CTIGATEEN1` writer - 1:1\\]
Enable CTICHOUT1. Set to 0 to disable channel propagation."]
pub type Ctigateen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIGATEEN2` reader - 2:2\\]
Enable CTICHOUT2. Set to 0 to disable channel propagation."]
pub type Ctigateen2R = crate::BitReader;
#[doc = "Field `CTIGATEEN2` writer - 2:2\\]
Enable CTICHOUT2. Set to 0 to disable channel propagation."]
pub type Ctigateen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIGATEEN3` reader - 3:3\\]
Enable CTICHOUT3. Set to 0 to disable channel propagation."]
pub type Ctigateen3R = crate::BitReader;
#[doc = "Field `CTIGATEEN3` writer - 3:3\\]
Enable CTICHOUT3. Set to 0 to disable channel propagation."]
pub type Ctigateen3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable CTICHOUT0. Set to 0 to disable channel propagation."]
    #[inline(always)]
    pub fn ctigateen0(&self) -> Ctigateen0R {
        Ctigateen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable CTICHOUT1. Set to 0 to disable channel propagation."]
    #[inline(always)]
    pub fn ctigateen1(&self) -> Ctigateen1R {
        Ctigateen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable CTICHOUT2. Set to 0 to disable channel propagation."]
    #[inline(always)]
    pub fn ctigateen2(&self) -> Ctigateen2R {
        Ctigateen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable CTICHOUT3. Set to 0 to disable channel propagation."]
    #[inline(always)]
    pub fn ctigateen3(&self) -> Ctigateen3R {
        Ctigateen3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable CTICHOUT0. Set to 0 to disable channel propagation."]
    #[inline(always)]
    #[must_use]
    pub fn ctigateen0(&mut self) -> Ctigateen0W<Cti_Cfg_CsctiCfgCtigateSpec> {
        Ctigateen0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable CTICHOUT1. Set to 0 to disable channel propagation."]
    #[inline(always)]
    #[must_use]
    pub fn ctigateen1(&mut self) -> Ctigateen1W<Cti_Cfg_CsctiCfgCtigateSpec> {
        Ctigateen1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable CTICHOUT2. Set to 0 to disable channel propagation."]
    #[inline(always)]
    #[must_use]
    pub fn ctigateen2(&mut self) -> Ctigateen2W<Cti_Cfg_CsctiCfgCtigateSpec> {
        Ctigateen2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable CTICHOUT3. Set to 0 to disable channel propagation."]
    #[inline(always)]
    #[must_use]
    pub fn ctigateen3(&mut self) -> Ctigateen3W<Cti_Cfg_CsctiCfgCtigateSpec> {
        Ctigateen3W::new(self, 3)
    }
}
#[doc = "The Gate Enable Register prevents the channels from propagating through the CTM to other CTIs. This enables local cross-triggering, for example for causing an interrupt when the ETM trigger occurs. It can be used effectively with CTIAPPSET, CTIAPPCLEAR, and CTIAPPPULSE for asserting trigger outputs by asserting channels, without affecting the rest of the system. On reset, this register is 0xF, and channel propagation is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctigate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctigate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtigateSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtigateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctigate::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtigateSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctigate::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtigateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTIGATE to value 0x0f"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtigateSpec {
    const RESET_VALUE: u32 = 0x0f;
}
