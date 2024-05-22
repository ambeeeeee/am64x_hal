#[doc = "Register `CFG_TIOCP_CFG` reader"]
pub type R = crate::R<CfgTiocpCfgSpec>;
#[doc = "Register `CFG_TIOCP_CFG` writer"]
pub type W = crate::W<CfgTiocpCfgSpec>;
#[doc = "Field `SOFTRESET` reader - 0:0\\]
Software reset"]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - 0:0\\]
Software reset"]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMUFREE` reader - 1:1\\]
Emulation Mode"]
pub type EmufreeR = crate::BitReader;
#[doc = "Field `EMUFREE` writer - 1:1\\]
Emulation Mode"]
pub type EmufreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEMODE` reader - 3:2\\]
Idle Mode"]
pub type IdlemodeR = crate::FieldReader;
#[doc = "Field `IDLEMODE` writer - 3:2\\]
Idle Mode"]
pub type IdlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software reset"]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Emulation Mode"]
    #[inline(always)]
    pub fn emufree(&self) -> EmufreeR {
        EmufreeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Idle Mode"]
    #[inline(always)]
    pub fn idlemode(&self) -> IdlemodeR {
        IdlemodeR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SoftresetW<CfgTiocpCfgSpec> {
        SoftresetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Emulation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn emufree(&mut self) -> EmufreeW<CfgTiocpCfgSpec> {
        EmufreeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Idle Mode"]
    #[inline(always)]
    #[must_use]
    pub fn idlemode(&mut self) -> IdlemodeW<CfgTiocpCfgSpec> {
        IdlemodeW::new(self, 2)
    }
}
#[doc = "This register controls the various parameters of the OCP interface\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tiocp_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tiocp_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTiocpCfgSpec;
impl crate::RegisterSpec for CfgTiocpCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tiocp_cfg::R`](R) reader structure"]
impl crate::Readable for CfgTiocpCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tiocp_cfg::W`](W) writer structure"]
impl crate::Writable for CfgTiocpCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TIOCP_CFG to value 0x08"]
impl crate::Resettable for CfgTiocpCfgSpec {
    const RESET_VALUE: u32 = 0x08;
}
