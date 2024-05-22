#[doc = "Register `CFG0_EPWM7_CTRL` reader"]
pub type R = crate::R<Cfg0Epwm7CtrlSpec>;
#[doc = "Register `CFG0_EPWM7_CTRL` writer"]
pub type W = crate::W<Cfg0Epwm7CtrlSpec>;
#[doc = "Field `EPWM7_CTRL_EALLOW` reader - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm7CtrlEallowR = crate::BitReader;
#[doc = "Field `EPWM7_CTRL_EALLOW` writer - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm7CtrlEallowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    pub fn epwm7_ctrl_eallow(&self) -> Epwm7CtrlEallowR {
        Epwm7CtrlEallowR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    #[must_use]
    pub fn epwm7_ctrl_eallow(&mut self) -> Epwm7CtrlEallowW<Cfg0Epwm7CtrlSpec> {
        Epwm7CtrlEallowW::new(self, 4)
    }
}
#[doc = "CFG0_EPWM7_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm7_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm7_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Epwm7CtrlSpec;
impl crate::RegisterSpec for Cfg0Epwm7CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm7_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Epwm7CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm7_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Epwm7CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM7_CTRL to value 0"]
impl crate::Resettable for Cfg0Epwm7CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
