#[doc = "Register `CFG0_EPWM6_CTRL` reader"]
pub type R = crate::R<Cfg0Epwm6CtrlSpec>;
#[doc = "Register `CFG0_EPWM6_CTRL` writer"]
pub type W = crate::W<Cfg0Epwm6CtrlSpec>;
#[doc = "Field `EPWM6_CTRL_EALLOW` reader - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm6CtrlEallowR = crate::BitReader;
#[doc = "Field `EPWM6_CTRL_EALLOW` writer - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm6CtrlEallowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM6_CTRL_SYNCIN_SEL` reader - 10:8\\]
Selects the source of the EPWM6 synchronization input"]
pub type Epwm6CtrlSyncinSelR = crate::FieldReader;
#[doc = "Field `EPWM6_CTRL_SYNCIN_SEL` writer - 10:8\\]
Selects the source of the EPWM6 synchronization input"]
pub type Epwm6CtrlSyncinSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    pub fn epwm6_ctrl_eallow(&self) -> Epwm6CtrlEallowR {
        Epwm6CtrlEallowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the source of the EPWM6 synchronization input"]
    #[inline(always)]
    pub fn epwm6_ctrl_syncin_sel(&self) -> Epwm6CtrlSyncinSelR {
        Epwm6CtrlSyncinSelR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    #[must_use]
    pub fn epwm6_ctrl_eallow(&mut self) -> Epwm6CtrlEallowW<Cfg0Epwm6CtrlSpec> {
        Epwm6CtrlEallowW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the source of the EPWM6 synchronization input"]
    #[inline(always)]
    #[must_use]
    pub fn epwm6_ctrl_syncin_sel(&mut self) -> Epwm6CtrlSyncinSelW<Cfg0Epwm6CtrlSpec> {
        Epwm6CtrlSyncinSelW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM6_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm6_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm6_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Epwm6CtrlSpec;
impl crate::RegisterSpec for Cfg0Epwm6CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm6_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Epwm6CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm6_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Epwm6CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM6_CTRL to value 0"]
impl crate::Resettable for Cfg0Epwm6CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
