#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg3` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg3` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec>;
#[doc = "Field `AES_MODE3` reader - 1:0\\]
AES mode"]
pub type AesMode3R = crate::FieldReader;
#[doc = "Field `AES_MODE3` writer - 1:0\\]
AES mode"]
pub type AesMode3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAC_MODE3` reader - 3:2\\]
MAC mode"]
pub type MacMode3R = crate::FieldReader;
#[doc = "Field `MAC_MODE3` writer - 3:2\\]
MAC mode"]
pub type MacMode3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRT_PROTECT3` reader - 4:4\\]
WRT protect"]
pub type WrtProtect3R = crate::BitReader;
#[doc = "Field `WRT_PROTECT3` writer - 4:4\\]
WRT protect"]
pub type WrtProtect3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
AES mode"]
    #[inline(always)]
    pub fn aes_mode3(&self) -> AesMode3R {
        AesMode3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
MAC mode"]
    #[inline(always)]
    pub fn mac_mode3(&self) -> MacMode3R {
        MacMode3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
WRT protect"]
    #[inline(always)]
    pub fn wrt_protect3(&self) -> WrtProtect3R {
        WrtProtect3R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
AES mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes_mode3(&mut self) -> AesMode3W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec> {
        AesMode3W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
MAC mode"]
    #[inline(always)]
    #[must_use]
    pub fn mac_mode3(&mut self) -> MacMode3W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec> {
        MacMode3W::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
WRT protect"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_protect3(&mut self) -> WrtProtect3W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec> {
        WrtProtect3W::new(self, 4)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg3 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec {
    const RESET_VALUE: u32 = 0;
}
