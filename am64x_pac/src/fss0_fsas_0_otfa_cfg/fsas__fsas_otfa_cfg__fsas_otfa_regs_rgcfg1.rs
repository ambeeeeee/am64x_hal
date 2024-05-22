#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg1` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg1` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec>;
#[doc = "Field `AES_MODE1` reader - 1:0\\]
AES mode"]
pub type AesMode1R = crate::FieldReader;
#[doc = "Field `AES_MODE1` writer - 1:0\\]
AES mode"]
pub type AesMode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAC_MODE1` reader - 3:2\\]
MAC mode"]
pub type MacMode1R = crate::FieldReader;
#[doc = "Field `MAC_MODE1` writer - 3:2\\]
MAC mode"]
pub type MacMode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRT_PROTECT1` reader - 4:4\\]
WRT protect"]
pub type WrtProtect1R = crate::BitReader;
#[doc = "Field `WRT_PROTECT1` writer - 4:4\\]
WRT protect"]
pub type WrtProtect1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
AES mode"]
    #[inline(always)]
    pub fn aes_mode1(&self) -> AesMode1R {
        AesMode1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
MAC mode"]
    #[inline(always)]
    pub fn mac_mode1(&self) -> MacMode1R {
        MacMode1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
WRT protect"]
    #[inline(always)]
    pub fn wrt_protect1(&self) -> WrtProtect1R {
        WrtProtect1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
AES mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes_mode1(&mut self) -> AesMode1W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec> {
        AesMode1W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
MAC mode"]
    #[inline(always)]
    #[must_use]
    pub fn mac_mode1(&mut self) -> MacMode1W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec> {
        MacMode1W::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
WRT protect"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_protect1(&mut self) -> WrtProtect1W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec> {
        WrtProtect1W::new(self, 4)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg1 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec {
    const RESET_VALUE: u32 = 0;
}
