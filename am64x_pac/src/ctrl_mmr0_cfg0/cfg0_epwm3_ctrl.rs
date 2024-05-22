#[doc = "Register `CFG0_EPWM3_CTRL` reader"]
pub type R = crate::R<Cfg0Epwm3CtrlSpec>;
#[doc = "Register `CFG0_EPWM3_CTRL` writer"]
pub type W = crate::W<Cfg0Epwm3CtrlSpec>;
#[doc = "Field `EPWM3_CTRL_EALLOW` reader - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm3CtrlEallowR = crate::BitReader;
#[doc = "Field `EPWM3_CTRL_EALLOW` writer - 4:4\\]
Activate write access to EPWM tripzone registers"]
pub type Epwm3CtrlEallowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPWM3_CTRL_SYNCIN_SEL` reader - 10:8\\]
Selects the source of the EPWM3 synchronization input"]
pub type Epwm3CtrlSyncinSelR = crate::FieldReader;
#[doc = "Field `EPWM3_CTRL_SYNCIN_SEL` writer - 10:8\\]
Selects the source of the EPWM3 synchronization input"]
pub type Epwm3CtrlSyncinSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    pub fn epwm3_ctrl_eallow(&self) -> Epwm3CtrlEallowR {
        Epwm3CtrlEallowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the source of the EPWM3 synchronization input"]
    #[inline(always)]
    pub fn epwm3_ctrl_syncin_sel(&self) -> Epwm3CtrlSyncinSelR {
        Epwm3CtrlSyncinSelR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Activate write access to EPWM tripzone registers"]
    #[inline(always)]
    #[must_use]
    pub fn epwm3_ctrl_eallow(&mut self) -> Epwm3CtrlEallowW<Cfg0Epwm3CtrlSpec> {
        Epwm3CtrlEallowW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the source of the EPWM3 synchronization input"]
    #[inline(always)]
    #[must_use]
    pub fn epwm3_ctrl_syncin_sel(&mut self) -> Epwm3CtrlSyncinSelW<Cfg0Epwm3CtrlSpec> {
        Epwm3CtrlSyncinSelW::new(self, 8)
    }
}
#[doc = "CFG0_EPWM3_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm3_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm3_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Epwm3CtrlSpec;
impl crate::RegisterSpec for Cfg0Epwm3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_epwm3_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Epwm3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_epwm3_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Epwm3CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EPWM3_CTRL to value 0"]
impl crate::Resettable for Cfg0Epwm3CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
