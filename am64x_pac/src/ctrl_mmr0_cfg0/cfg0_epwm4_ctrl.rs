#[doc = "Register `CFG0_EPWM4_CTRL` reader"]
pub type R = crate::R<Cfg0Epwm4CtrlSpec>;
#[doc = "Register `CFG0_EPWM4_CTRL` writer"]
pub type W = crate::W<Cfg0Epwm4CtrlSpec>;
#[doc = "Field `EPWM4_CTRL_EALLOW` reader - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm4CtrlEallowR = crate::BitReader;
#[doc = "Field `EPWM4_CTRL_EALLOW` writer - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm4CtrlEallowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    pub fn epwm4_ctrl_eallow(&self) -> Epwm4CtrlEallowR {
        Epwm4CtrlEallowR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    #[must_use]
    pub fn epwm4_ctrl_eallow(&mut self) -> Epwm4CtrlEallowW<Cfg0Epwm4CtrlSpec> {
        Epwm4CtrlEallowW::new(self, 4)
    }
}
#[doc = "CFG0_EPWM4_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm4_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm4_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Epwm4CtrlSpec;
impl crate::RegisterSpec for Cfg0Epwm4CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm4_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Epwm4CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm4_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Epwm4CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM4_CTRL to value 0"]
impl crate::Resettable for Cfg0Epwm4CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
