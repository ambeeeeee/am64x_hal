#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg0` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg0` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec>;
#[doc = "Field `AES_MODE0` reader - 1:0\\]
AES mode"]
pub type AesMode0R = crate::FieldReader;
#[doc = "Field `AES_MODE0` writer - 1:0\\]
AES mode"]
pub type AesMode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAC_MODE0` reader - 3:2\\]
MAC mode"]
pub type MacMode0R = crate::FieldReader;
#[doc = "Field `MAC_MODE0` writer - 3:2\\]
MAC mode"]
pub type MacMode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRT_PROTECT0` reader - 4:4\\]
WRT protect"]
pub type WrtProtect0R = crate::BitReader;
#[doc = "Field `WRT_PROTECT0` writer - 4:4\\]
WRT protect"]
pub type WrtProtect0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
AES mode"]
    #[inline(always)]
    pub fn aes_mode0(&self) -> AesMode0R {
        AesMode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
MAC mode"]
    #[inline(always)]
    pub fn mac_mode0(&self) -> MacMode0R {
        MacMode0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
WRT protect"]
    #[inline(always)]
    pub fn wrt_protect0(&self) -> WrtProtect0R {
        WrtProtect0R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
AES mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes_mode0(&mut self) -> AesMode0W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec> {
        AesMode0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
MAC mode"]
    #[inline(always)]
    #[must_use]
    pub fn mac_mode0(&mut self) -> MacMode0W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec> {
        MacMode0W::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
WRT protect"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_protect0(&mut self) -> WrtProtect0W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec> {
        WrtProtect0W::new(self, 4)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg0 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec {
    const RESET_VALUE: u32 = 0;
}
