#[doc = "Register `CFG_GPMC_SYSCONFIG` reader"]
pub type R = crate::R<CfgGpmcSysconfigSpec>;
#[doc = "Register `CFG_GPMC_SYSCONFIG` writer"]
pub type W = crate::W<CfgGpmcSysconfigSpec>;
#[doc = "Field `AUTOIDLE` reader - 0:0\\]
Internal OCP clock gating strategy"]
pub type AutoidleR = crate::BitReader;
#[doc = "Field `AUTOIDLE` writer - 0:0\\]
Internal OCP clock gating strategy"]
pub type AutoidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEMODE` reader - "]
pub type IdlemodeR = crate::FieldReader;
#[doc = "Field `IDLEMODE` writer - "]
pub type IdlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP clock gating strategy"]
    #[inline(always)]
    pub fn autoidle(&self) -> AutoidleR {
        AutoidleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn idlemode(&self) -> IdlemodeR {
        IdlemodeR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal OCP clock gating strategy"]
    #[inline(always)]
    #[must_use]
    pub fn autoidle(&mut self) -> AutoidleW<CfgGpmcSysconfigSpec> {
        AutoidleW::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn idlemode(&mut self) -> IdlemodeW<CfgGpmcSysconfigSpec> {
        IdlemodeW::new(self, 3)
    }
}
#[doc = "This register controls the various parameters of the OCP interface\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_sysconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_sysconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcSysconfigSpec;
impl crate::RegisterSpec for CfgGpmcSysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_sysconfig::R`](R) reader structure"]
impl crate::Readable for CfgGpmcSysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_sysconfig::W`](W) writer structure"]
impl crate::Writable for CfgGpmcSysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_SYSCONFIG to value 0"]
impl crate::Resettable for CfgGpmcSysconfigSpec {
    const RESET_VALUE: u32 = 0;
}
