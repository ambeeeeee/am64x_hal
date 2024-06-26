#[doc = "Register `CFG0_SDIO1_CTRL` reader"]
pub type R = crate::R<Cfg0Sdio1CtrlSpec>;
#[doc = "Register `CFG0_SDIO1_CTRL` writer"]
pub type W = crate::W<Cfg0Sdio1CtrlSpec>;
#[doc = "Field `SDIO1_CTRL_DRV_STR` reader - 4:0\\]
Selects the SDIO drive strength"]
pub type Sdio1CtrlDrvStrR = crate::FieldReader;
#[doc = "Field `SDIO1_CTRL_DRV_STR` writer - 4:0\\]
Selects the SDIO drive strength"]
pub type Sdio1CtrlDrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the SDIO drive strength"]
    #[inline(always)]
    pub fn sdio1_ctrl_drv_str(&self) -> Sdio1CtrlDrvStrR {
        Sdio1CtrlDrvStrR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Selects the SDIO drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_ctrl_drv_str(&mut self) -> Sdio1CtrlDrvStrW<Cfg0Sdio1CtrlSpec> {
        Sdio1CtrlDrvStrW::new(self, 0)
    }
}
#[doc = "CFG0_SDIO1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_sdio1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_sdio1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Sdio1CtrlSpec;
impl crate::RegisterSpec for Cfg0Sdio1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_sdio1_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Sdio1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_sdio1_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Sdio1CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_SDIO1_CTRL to value 0"]
impl crate::Resettable for Cfg0Sdio1CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
